#!/usr/bin/env python3
"""
Staged top-down screw grasp executor.

Idle until /start_grasp (StartGrasp: screw_id) is called. Then:

  1. Seed: look up TF frame `screw_<id>` (published by the detector) -> x, y, yaw.
  2. Open the gripper to its approach width.
  3. For each height in `stage_heights` (fingertip z, base frame):
         move above the current estimate at that height (tool pointing straight down,
         fingers closing perpendicular to the screw axis), wait for the arm to settle,
         then re-estimate the screw from the detector's raw per-frame observations.
     (Stage 0 is taken in place by default: the arm is already at the high view pose.)
  4. Re-align at the last stage height, descend straight to `grasp_z`, close, lift.

The detector does the perception work (pixel -> world, axis direction). This node
only consumes world-frame poses:
  TF  screw_<id>              (tracked, stable IDs)   -> used once to pick the screw
  /screw_observations         (raw, per frame)        -> used for each re-estimate; the
                                                         screw nearest the last estimate
                                                         is followed (IDs can change as the
                                                         camera moves, position does not)
"""
import math
import re
import time
from collections import deque
from threading import Lock, Thread

import numpy as np
import rclpy
from rclpy.action import ActionClient
from rclpy.executors import MultiThreadedExecutor
from rclpy.node import Node
from rclpy.time import Time

from geometry_msgs.msg import PoseArray
from std_msgs.msg import String
from tf2_ros import Buffer, TransformException, TransformListener

from common_interfaces_merlab.srv import SendPose
from control_msgs.action import GripperCommand
from screw_sorting_interfaces.srv import StartGrasp


def wrap_pi(a):
    return (a + math.pi) % (2.0 * math.pi) - math.pi


class GripperExecutor(Node):

    def __init__(self):
        super().__init__('gripper_executor')

        # ---------------- Parameters (all readable live with `ros2 param set`) ----------------
        # perception interface

        self.declare_parameter('observations_topic', '/screw_observations')
        self.declare_parameter('base_frame', 'base_link')          # must match detector world_frame
        self.declare_parameter('screw_frame_prefix', 'screw')
        self.declare_parameter('seed_max_age', 2.0)                # s, max age of the screw_<id> TF

        # staged approach: fingertip z (base frame, m) at which each estimate is taken
        self.declare_parameter('stage_heights', [0.35, 0.15, 0.08])
        self.declare_parameter('skip_first_stage_move', True)      # arm is already at the high view pose
        self.declare_parameter('settle_time', 2.0)                 # s to wait after each move
        self.declare_parameter('num_samples', 10)
        self.declare_parameter('max_position_spread', 0.005)       # m, std dev of samples
        self.declare_parameter('stage_timeout', 10.0)              # s per estimate
        self.declare_parameter('reacquire_radius', 0.04)           # m, only follow screws this close to last estimate

        # grasp
        self.declare_parameter('grasp_z', 0.0)                     # fingertip z at grasp (base frame)
        self.declare_parameter('lift_height', 0.20)                # m above grasp_z after closing
        self.declare_parameter('tcp_offset', 0.1139)                # tool0 -> fingertips (m)
        self.declare_parameter('min_z', -0.005)                    # abort if fingertips would go below
        self.declare_parameter('finger_close_axis', 'y')           # tool axis fingers close along: x | y
        self.declare_parameter('yaw_offset_deg', 0.0)

        # gripper
        self.declare_parameter('gripper_command_topic', '/gen3_lite_2f_gripper_controller/gripper_cmd')
        self.declare_parameter('gripper_approach_position', 0.65)  # width used while approaching (tune!)
        self.declare_parameter('gripper_closed_position', 0.80)
        self.declare_parameter('gripper_max_effort', 100.0)
        self.declare_parameter('pose_timeout', 180.0)


        # xy correction added to every commanded fingertip position (m, positive = move that way).
        # grasp_offset_*: fixed in the BASE frame.  tool_offset_*: fixed in the TOOL frame (rotates with the gripper).
        self.declare_parameter('grasp_offset_x', 0.0)
        self.declare_parameter('grasp_offset_y', 0.0)
        self.declare_parameter('tool_offset_x', 0.007)
        self.declare_parameter('tool_offset_y', 0.002)

        self.p = lambda name: self.get_parameter(name).value

        # ---------------- Interfaces ----------------
        self.pose_client = self.create_client(SendPose, 'send_pose')
        self.gripper_client = ActionClient(self, GripperCommand, self.p('gripper_command_topic'))

        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)

        self.create_subscription(PoseArray, self.p('observations_topic'),
                                 self.observations_callback, 10)
        self.create_service(StartGrasp, 'start_grasp', self.start_grasp_callback)
        self.status_pub = self.create_publisher(String, 'grasp_status', 10)

        # ---------------- State ----------------
        self.busy = False
        self.busy_lock = Lock()
        self.collecting = False
        self.samples = deque(maxlen=int(self.p('num_samples')))
        self.samples_lock = Lock()
        self.gate_center = (0.0, 0.0)
        self.t_min_ns = 0
        self.last_tool_yaw = None

        self.set_status('idle')
        self.get_logger().info("Ready. Call /start_grasp with a screw_id to begin.")

    # =====================================================
    # Utilities
    # =====================================================
    def set_status(self, text):
        self.status_pub.publish(String(data=text))

    def wait_for_future(self, future, timeout, name):
        deadline = time.monotonic() + timeout
        while rclpy.ok() and not future.done():
            if time.monotonic() >= deadline:
                self.get_logger().error(f'Timed out waiting for {name}.')
                return None
            time.sleep(0.01)
        if not future.done():
            return None
        try:
            return future.result()
        except Exception as exc:
            self.get_logger().error(f'Call to {name} failed: {exc}')
            return None

    @staticmethod
    def matrix_to_quat(R):
        tr = np.trace(R)
        if tr > 0.0:
            s = math.sqrt(tr + 1.0) * 2.0
            w = 0.25 * s
            x = (R[2, 1] - R[1, 2]) / s
            y = (R[0, 2] - R[2, 0]) / s
            z = (R[1, 0] - R[0, 1]) / s
        elif R[0, 0] > R[1, 1] and R[0, 0] > R[2, 2]:
            s = math.sqrt(1.0 + R[0, 0] - R[1, 1] - R[2, 2]) * 2.0
            w = (R[2, 1] - R[1, 2]) / s
            x = 0.25 * s
            y = (R[0, 1] + R[1, 0]) / s
            z = (R[0, 2] + R[2, 0]) / s
        elif R[1, 1] > R[2, 2]:
            s = math.sqrt(1.0 + R[1, 1] - R[0, 0] - R[2, 2]) * 2.0
            w = (R[0, 2] - R[2, 0]) / s
            x = (R[0, 1] + R[1, 0]) / s
            y = 0.25 * s
            z = (R[1, 2] + R[2, 1]) / s
        else:
            s = math.sqrt(1.0 + R[2, 2] - R[0, 0] - R[1, 1]) * 2.0
            w = (R[1, 0] - R[0, 1]) / s
            x = (R[0, 2] + R[2, 0]) / s
            y = (R[1, 2] + R[2, 1]) / s
            z = 0.25 * s
        q = np.array([x, y, z, w])
        return q / np.linalg.norm(q)

    @staticmethod
    def quat_to_yaw(q):
        return math.atan2(2.0 * (q.w * q.z + q.x * q.y), 1.0 - 2.0 * (q.y * q.y + q.z * q.z))

    # =====================================================
    # Perception input
    # =====================================================
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
        return (t.x, t.y, self.quat_to_yaw(q)), None

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
            self.samples.append((best.position.x, best.position.y,
                                 self.quat_to_yaw(best.orientation)))

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

    # =====================================================
    # Motion
    # =====================================================
    def wait_for_services(self, timeout=30.0):
        if not self.pose_client.wait_for_service(timeout_sec=timeout):
            self.get_logger().error('Service send_pose is unavailable.')
            return False
        return True

    def move_to_pose(self, x, y, z, q):
        request = SendPose.Request()
        request.pose.position.x, request.pose.position.y, request.pose.position.z = \
            float(x), float(y), float(z)
        request.pose.orientation.x, request.pose.orientation.y = float(q[0]), float(q[1])
        request.pose.orientation.z, request.pose.orientation.w = float(q[2]), float(q[3])

        self.get_logger().info(f'Moving tool0 to ({x:.3f}, {y:.3f}, {z:.3f})')
        future = self.pose_client.call_async(request)
        response = self.wait_for_future(future, float(self.p('pose_timeout')), '/send_pose')
        if response is None or not response.success:
            self.get_logger().error('Pose motion failed.')
            return False
        return True

    def apply_offsets(self, x, y):
        """Add the base-frame and tool-frame xy corrections to an estimated screw position."""
        a = self.last_tool_yaw if self.last_tool_yaw is not None else 0.0
        tx, ty = float(self.p('tool_offset_x')), float(self.p('tool_offset_y'))
        # tool x axis = (cos a, sin a); tool y axis = (sin a, -cos a) for a straight-down tool
        return (x + float(self.p('grasp_offset_x')) + tx * math.cos(a) + ty * math.sin(a),
                y + float(self.p('grasp_offset_y')) + tx * math.sin(a) - ty * math.cos(a))

    def move_tcp(self, x, y, tcp_z, q):
        """Move so the FINGERTIPS are at (x, y, tcp_z). tool0 sits tcp_offset above them."""
        if tcp_z < float(self.p('min_z')):
            self.get_logger().error(f'Fingertip z={tcp_z:.3f} below min_z; refusing to move.')
            return False
        cx, cy = self.apply_offsets(x, y)
        return self.move_to_pose(cx, cy, tcp_z + float(self.p('tcp_offset')), q)

    def set_gripper(self, position):
        self.get_logger().info(f'Gripper -> {position:.2f}')
        if not self.gripper_client.wait_for_server(timeout_sec=10.0):
            self.get_logger().error('Gripper action server unavailable.')
            return False

        goal = GripperCommand.Goal()
        goal.command.position = float(position)
        goal.command.max_effort = float(self.p('gripper_max_effort'))

        goal_handle = self.wait_for_future(
            self.gripper_client.send_goal_async(goal), 10.0, 'gripper goal')
        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().error('Gripper command rejected.')
            return False
        result = self.wait_for_future(goal_handle.get_result_async(), 15.0, 'gripper movement')
        if result is None:
            self.get_logger().error('Gripper movement timed out.')
            return False
        return True

    # =====================================================
    # Grasp geometry
    # =====================================================
    def top_down_orientation(self, axis_yaw):
        """
        Quaternion [x, y, z, w]: tool z straight down, finger closing direction
        perpendicular to the screw axis. Of the two equivalent (180 deg apart) solutions,
        picks the one closest to the previous commanded tool yaw to avoid wrist flips.
        """
        offset = math.radians(float(self.p('yaw_offset_deg')))
        close_axis = str(self.p('finger_close_axis')).lower()
        if close_axis not in ('x', 'y'):
            raise ValueError("finger_close_axis must be 'x' or 'y'")
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

        return self.matrix_to_quat(np.column_stack((pick[0], pick[1], z_t)))

    # =====================================================
    # Service + sequence
    # =====================================================
    def validate_params(self):
        heights = list(self.p('stage_heights'))
        if len(heights) == 0:
            return 'stage_heights is empty'
        min_z = float(self.p('min_z'))
        if min(heights) < min_z:
            return f'a stage height is below min_z={min_z}'
        if float(self.p('grasp_z')) < min_z:
            return f"grasp_z={self.p('grasp_z')} is below min_z={min_z}"
        if not all(a >= b for a, b in zip(heights, heights[1:])):
            return 'stage_heights must be non-increasing (high -> low)'
        if float(self.p('lift_height')) < 0.0:
            return 'lift_height cannot be negative'
        return None

    def start_grasp_callback(self, request, response):
        with self.busy_lock:
            if self.busy:
                response.accepted, response.message = False, 'Busy: a grasp is already running.'
                return response

            error = self.validate_params()
            seed, reason = (None, error) if error else self.lookup_seed(request.screw_id)
            if seed is None:
                response.accepted, response.message = False, reason
                self.get_logger().warning(f'Start rejected: {reason}')
                return response

            self.busy = True

        Thread(target=self.run_sequence, args=(int(request.screw_id), seed), daemon=True).start()
        response.accepted = True
        response.message = (f'Started grasp of screw {request.screw_id} at '
                            f'({seed[0]:.3f}, {seed[1]:.3f}).')
        self.get_logger().info(response.message)
        return response

    def run_sequence(self, screw_id, seed):
        ok = False
        try:
            ok = self.sequence(screw_id, seed)
        except Exception as exc:
            self.get_logger().error(f'Unexpected error: {exc}')
        finally:
            self.collecting = False
            self.set_status('done' if ok else 'failed')
            self.get_logger().info('Grasp sequence ' + ('completed.' if ok else 'ABORTED.'))
            with self.busy_lock:
                self.busy = False

    def sequence(self, screw_id, seed):
        x, y, yaw = seed
        heights = [float(h) for h in self.p('stage_heights')]
        grasp_z = float(self.p('grasp_z'))
        lift_z = grasp_z + float(self.p('lift_height'))
        self.last_tool_yaw = None

        if not self.wait_for_services():
            return False

        self.set_status('opening gripper')
        if not self.set_gripper(float(self.p('gripper_approach_position'))):
            return False

        # ---- staged approach with re-estimation ----
        for i, h in enumerate(heights):
            self.set_status(f'stage {i + 1}/{len(heights)}')
            self.get_logger().info(f'Stage {i + 1}/{len(heights)} (fingertip z = {h:.3f})')

            if not (i == 0 and bool(self.p('skip_first_stage_move'))):
                q = self.top_down_orientation(yaw)
                if not self.move_tcp(x, y, h, q):
                    return False

            est = self.estimate_screw((x, y))
            if est is None:
                return False
            x, y, yaw = est

        # ---- final alignment, descent, grasp, lift ----
        q = self.top_down_orientation(yaw)

        self.set_status('aligning')
        if not self.move_tcp(x, y, heights[-1], q):       # rotate/centre at the last stage height
            return False

        self.set_status('descending')
        if not self.move_tcp(x, y, grasp_z, q):           # straight down
            return False

        self.set_status('closing')
        if not self.set_gripper(float(self.p('gripper_closed_position'))):
            return False

        self.set_status('lifting')
        return self.move_tcp(x, y, lift_z, q)


def main(args=None):
    rclpy.init(args=args)
    node = GripperExecutor()
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
