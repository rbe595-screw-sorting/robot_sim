#!/usr/bin/env python3

import math
import re
import time
from collections import deque
from threading import Lock, Thread

import rclpy
from rclpy.action import ActionClient
from rclpy.executors import MultiThreadedExecutor
from rclpy.node import Node
from rclpy.time import Time

from geometry_msgs.msg import PoseArray
from std_msgs.msg import String
from tf2_ros import Buffer, TransformException, TransformListener

from control_msgs.action import GripperCommand
from screw_sorting_interfaces.action import GraspScrew

from rclpy.callback_groups import ReentrantCallbackGroup
from helpers.robot_motion import ArmMover, wait_for
from rclpy.action import ActionServer, GoalResponse, CancelResponse
import numpy as np
from scipy.spatial.transform import Rotation as R



import asyncio


def wrap_pi(a):
    return (a + math.pi) % (2.0 * math.pi) - math.pi

class GraspExecutor(Node):
    def __init__(self):
        super().__init__('gripper_executor')
        self.motion_client_group = ReentrantCallbackGroup()

        self.arm_mover = ArmMover(
                    self,
                    group="arm",
                    ee_link="end_effector_link",
                    base_frame="base_link",
                    speed=0.5,
                    motion="cartesian",
                    cartesian_fallback=False,
                    callback_group=self.motion_client_group
                )
        
                # Gripper action client
        self.gripper_client = ActionClient(
            self,
            GripperCommand,
            '/gen3_lite_2f_gripper_controller/gripper_cmd',
            callback_group=self.motion_client_group
        )

        self.declare_parameter('observations_topic', '/screw_observations')
        self.declare_parameter('base_frame', 'base_link')          # must match detector world_frame
        self.declare_parameter('screw_frame_prefix', 'screw')
        self.declare_parameter('seed_max_age', 2.0)                # s, max age of the screw_<id> TF

        # staged approach: fingertip z (base frame, m) at which each estimate is taken
        self.declare_parameter('stage_heights', [0.35, 0.20, 0.10])
        self.declare_parameter('skip_first_stage_move', True)      # arm is already at the high view pose
        self.declare_parameter('settle_time', 2.0)                 # s to wait after each move
        self.declare_parameter('num_samples', 10)
        self.declare_parameter('max_position_spread', 0.005)       # m, std dev of samples
        self.declare_parameter('stage_timeout', 15.0)              # s per estimate
        self.declare_parameter('reacquire_radius', 0.05)           # m, only follow screws this close to last estimate

        # grasp
        self.declare_parameter('grasp_z', 0.0)                     # fingertip z at grasp (base frame)
        self.declare_parameter('lift_height', 0.20)                # m above grasp_z after closing
        self.declare_parameter('tcp_offset', 0.1147)                # tool0 -> fingertips (m)
        self.declare_parameter('min_z', -0.005)                    # abort if fingertips would go below
        self.declare_parameter('finger_close_axis', 'y')           # tool axis fingers close along: x | y
        self.declare_parameter('yaw_offset_deg', 0.0)

        # gripper
        self.declare_parameter('gripper_command_topic', '/gen3_lite_2f_gripper_controller/gripper_cmd')
        self.declare_parameter('gripper_approach_position', 0.65)  # width used while approaching (tune!)
        self.declare_parameter('gripper_closed_position', 0.8)
        self.declare_parameter('gripper_max_effort', 100.0)
        self.declare_parameter('pose_timeout', 180.0)


        # xy correction added to every commanded fingertip position (m, positive = move that way).
        # grasp_offset_*: fixed in the BASE frame.  tool_offset_*: fixed in the TOOL frame (rotates with the gripper).
        self.declare_parameter('grasp_offset_x', 0.0)
        self.declare_parameter('grasp_offset_y', 0.0)
        # self.declare_parameter('tool_offset_x', 0.007)
        # self.declare_parameter('tool_offset_y', 0.002)
        self.declare_parameter('tool_offset_x', 0.00)
        self.declare_parameter('tool_offset_y', 0.00)

        self.p = lambda name: self.get_parameter(name).value


        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)

        self.create_subscription(PoseArray, self.p('observations_topic'),
                                 self.observations_callback, 10)
        

        self.callback_group = ReentrantCallbackGroup()
        self.action_server = ActionServer(
            self,
            GraspScrew,
            '/grasp_screw',

            execute_callback=self.execute_callback,
            goal_callback=self.goal_callback,
            cancel_callback=self.cancel_callback,

            callback_group=self.callback_group
        )

        self.busy = False
        self.busy_lock = Lock()
        self.collecting = False
        self.samples = deque(maxlen=int(self.p('num_samples')))
        self.samples_lock = Lock()
        self.gate_center = (0.0, 0.0)
        self.t_min_ns = 0
        self.last_tool_yaw = None
        

    async def execute_callback(self, goal_handle):
        try:
            feedback = GraspScrew.Feedback()
            result = GraspScrew.Result()

            seed, reason = self.lookup_seed(
                goal_handle.request.screw_id
            )

            if seed is None:

                result.success = False
                goal_handle.abort()

                return result

            x, y, yaw = seed    
            original_yaw = yaw        
            heights = [float(h) for h in self.p('stage_heights')]
            grasp_z = float(self.p('grasp_z'))
            lift_z = grasp_z + float(self.p('lift_height'))
            self.last_tool_yaw = None

            feedback.current_state = 'opening gripper'
            if not self.gripper(float(self.p('gripper_approach_position'))):
                raise RuntimeError("Failed to open gripper")

            # ---- staged approach with re-estimation ----
            for i, h in enumerate(heights):
                if goal_handle.is_cancel_requested:

                    goal_handle.canceled()

                    result.success = False

                    return result
                feedback.current_state = f'stage {i + 1}/{len(heights)}'

                if not (i == 0 and bool(self.p('skip_first_stage_move'))):
                    q = self.top_down_orientation(yaw)
                    if not self.move_tcp(x, y, h, q):
                        raise RuntimeError("Failed to reach above screw")

                est = self.estimate_screw((x, y))
                if est is None:
                    raise RuntimeError("Failed to see screw")
                x, y, yaw = est

                

            # ---- final alignment, descent, grasp, lift ----
            # q = self.top_down_orientation(yaw)
            q = self.top_down_orientation(original_yaw)

            feedback.current_state = 'aligning'
            goal_handle.publish_feedback(feedback)

            if not self.move_tcp(x, y, heights[-1], q):       # rotate/centre at the last stage height
                raise RuntimeError("Failed to reach above screw ")

            feedback.current_state = 'descending'
            goal_handle.publish_feedback(feedback)

            if not self.move_tcp(x, y, grasp_z, q):           # straight down
                raise RuntimeError("Failed to reach screw")

            feedback.current_state = 'closing'
            goal_handle.publish_feedback(feedback)

            if not self.gripper(float(self.p('gripper_closed_position'))):
                # raise RuntimeError("Failed to close gripper")
                pass
            feedback.current_state = 'lifting'
            goal_handle.publish_feedback(feedback)
            if not self.move_tcp(x, y, lift_z, q):
                raise RuntimeError("Failed to lift screw")

            result.success = True

            goal_handle.succeed()

            return result

    
        except Exception as e:

            self.get_logger().error(f"Grasp failed: {e}")

            result.success = False

            goal_handle.abort()

            return result

        finally:

            with self.busy_lock:
                self.busy = False


        
        
    def goal_callback(self, request):

        with self.busy_lock:

            if self.busy:
                return GoalResponse.REJECT

            seed, reason = self.lookup_seed(request.screw_id)

            if seed is None:
                self.get_logger().warning(reason)
                return GoalResponse.REJECT

            self.busy = True

        return GoalResponse.ACCEPT
    
    def cancel_callback(self, goal_handle):
        self.get_logger().info("Cancellation requested")

        return CancelResponse.ACCEPT

    #Perception
    def available_screw_ids(self):
        """IDs whose screw_<id> TF is currently being published (fresh)."""
        prefix = f"{self.p('screw_frame_prefix')}_"
        names = re.findall(r'Frame (\S+) exists', self.tf_buffer.all_frames_as_string())
        ids = []
        for n in names:
            if n.startswith(prefix) and n[len(prefix):].isdigit():
                try:
                    tf = self.tf_buffer.lookup_transform(self.p('base_frame'), n, Time())
                except TransformException:
                    continue
                age = (self.get_clock().now() - Time.from_msg(tf.header.stamp)).nanoseconds * 1e-9
                if age <= float(self.p('seed_max_age')):
                    ids.append(int(n[len(prefix):]))
        return sorted(ids)

    def lookup_seed(self, screw_id):
        """(x, y, yaw) of screw_<id> from TF, or (None, reason)."""
        if screw_id == -1:

            available = self.available_screw_ids()

            if not available:
                return None, "No screws currently tracked"

            screw_id = available[0]


        child = f"{self.p('screw_frame_prefix')}_{screw_id}"
        base = self.p('base_frame')
        hint = f" Currently tracked IDs: {self.available_screw_ids()}"
        try:
            tf = self.tf_buffer.lookup_transform(base, child, Time())
        except TransformException:
            return None, f"No TF '{child}' in '{base}'.{hint}"
        age = (self.get_clock().now() - Time.from_msg(tf.header.stamp)).nanoseconds * 1e-9
        if age > float(self.p('seed_max_age')):
            return None, f"TF '{child}' is stale ({age:.1f} s old), screw no longer tracked.{hint}"
        t, q = tf.transform.translation, tf.transform.rotation
        quat = [q.x, q.y, q.z, q.w]

        yaw = R.from_quat(quat).as_euler('zyx')[0]

        return (t.x, t.y, yaw), None

    def observations_callback(self, msg):
        """Collect the observation nearest the gate centre (while an estimate is running)."""
        if not self.collecting:
            return
        if msg.header.frame_id != self.p('base_frame'):
            self.get_logger().error(
                f"/screw_observations frame '{msg.header.frame_id}' != base_frame "
                f"'{self.p('base_frame')}'", throttle_duration_sec=5.0)
            return
        if msg.header.stamp.sec * 10**9 + msg.header.stamp.nanosec < self.t_min_ns:
            return   # captured before the arm settled

        cx, cy = self.gate_center
        best, best_d = None, float(self.p('reacquire_radius'))
        for pose in msg.poses:
            d = math.hypot(pose.position.x - cx, pose.position.y - cy)
            if d < best_d:
                best, best_d = pose, d
        if best is None:
            return
        with self.samples_lock:
            q = best.orientation
            quat = [q.x, q.y, q.z, q.w]

            yaw = R.from_quat(quat).as_euler('zyx')[0]

            self.samples.append((
                best.position.x,
                best.position.y,
                yaw
            ))

    def estimate_screw(self, center_xy):
        """Settle, then average N raw observations near center_xy. Returns (x, y, yaw) or None."""
        n = int(self.p('num_samples'))
        max_spread = float(self.p('max_position_spread'))
        timeout = float(self.p('stage_timeout'))

        time.sleep(float(self.p('settle_time')))

        with self.samples_lock:
            self.samples = deque(maxlen=n)
        self.gate_center = center_xy
        self.t_min_ns = self.get_clock().now().nanoseconds
        self.collecting = True
        deadline = time.monotonic() + timeout

        try:
            while rclpy.ok() and time.monotonic() < deadline:
                with self.samples_lock:
                    samples = list(self.samples)
                if len(samples) == n:
                    arr = np.array(samples)
                    spread = float(np.max(np.std(arr[:, :2], axis=0)))
                    if spread <= max_spread:
                        x, y = np.median(arr[:, :2], axis=0)
                        # axis is a line (period pi): average the doubled angle
                        yaw = 0.5 * math.atan2(np.mean(np.sin(2.0 * arr[:, 2])),
                                               np.mean(np.cos(2.0 * arr[:, 2])))
                        self.get_logger().info(
                            f'  estimate: x={x:.3f} y={y:.3f} axis={math.degrees(yaw):.1f} deg '
                            f'(spread {spread * 1000:.1f} mm)')
                        return float(x), float(y), float(yaw)
                time.sleep(0.05)
            self.get_logger().error(
                f'No stable estimate within {timeout:.0f} s (screw lost, outside '
                f'reacquire_radius, or spread > {max_spread * 1000:.0f} mm).')
            return None
        finally:
            self.collecting = False


    #Movement helpers
    def move(self, target):
        success = self.arm_mover.move_to(
            target,
            motion= "rrtconnect",
            cartesian_fallback=True
        )
        if not success:
            return False
        return True

    def move_tcp(self, x, y, tcp_z, q):
        """Move so the FINGERTIPS are at (x, y, tcp_z). tool0 sits tcp_offset above them."""
        if tcp_z < float(self.p('min_z')):
            self.get_logger().error(f'Fingertip z={tcp_z:.3f} below min_z; refusing to move.')
            return False
        return self.move((
            x,
            y,
            tcp_z + float(self.p('tcp_offset')),
            q[0],
            q[1],
            q[2],
            q[3]
        ))
    def gripper(self, position):
    
        if not self.gripper_client.wait_for_server(
            timeout_sec=5.0
        ):

            self.get_logger().error(
                "Gripper action server unavailable."
            )

            return False

        goal = GripperCommand.Goal()

        goal.command.position = position
        goal.command.max_effort = 30.0

        future = self.gripper_client.send_goal_async(goal)

        goal_handle = wait_for(
            future,
            timeout=10.0
        )

        if goal_handle is None or not goal_handle.accepted:

            self.get_logger().error(
                "Gripper command rejected."
            )

            return False

        result_future = goal_handle.get_result_async()

        result = wait_for(
            result_future,
            timeout=30.0
        )

        if result is None:
            self.get_logger().error(
                "Gripper action timed out."
            )
            return False

        if not result.result.reached_goal:
            self.get_logger().error(
                "Gripper failed to reach commanded position."
            )
            return False

        return True

    def top_down_orientation(self, axis_yaw):
        """
        Quaternion [x, y, z, w]: tool z straight down, finger closing direction
        perpendicular to the screw axis. Of the two equivalent (180 deg apart) solutions,
        picks the one closest to the previous commanded tool yaw to avoid wrist flips.
        """
        offset = math.radians(float(self.p('yaw_offset_deg')))
        close_axis = str(self.p('finger_close_axis')).lower()
        z_t = np.array([0.0, 0.0, -1.0])

        def build(close_yaw):
            c = np.array([math.cos(close_yaw), math.sin(close_yaw), 0.0])
            if close_axis == 'y':
                y_t = c
                x_t = np.cross(y_t, z_t)
            else:
                x_t = c
                y_t = np.cross(z_t, x_t)
            return x_t, y_t

        base = axis_yaw + math.pi / 2.0 + offset
        cands = [build(base), build(base + math.pi)]
        tool_yaw = lambda c: math.atan2(c[0][1], c[0][0])

        if self.last_tool_yaw is None:
            pick = cands[0] if cands[0][0][0] >= 0.0 else cands[1]   # x axis away from robot
        else:
            pick = min(cands, key=lambda c: abs(wrap_pi(tool_yaw(c) - self.last_tool_yaw)))
        self.last_tool_yaw = tool_yaw(pick)

        return R.from_matrix(np.column_stack((pick[0], pick[1], z_t))).as_quat()

def main(args=None):
    rclpy.init(args=args)
    node = GraspExecutor()
    executor = MultiThreadedExecutor(num_threads=4)
    executor.add_node(node)
    try:
        executor.spin()
    except KeyboardInterrupt:
        pass
    executor.shutdown()
    node.destroy_node()
    if rclpy.ok():
        rclpy.shutdown()


if __name__ == '__main__':
    main()
