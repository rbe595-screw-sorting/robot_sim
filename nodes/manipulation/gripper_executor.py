#!/usr/bin/env python3
"""
Open-loop screw grasp executor.

Pipeline:
  1. Collect N screw detections (pixel centroid + long-axis angle) from the
     screw detection node while the arm sits at its observation pose.
  2. Back-project the centroid (and a second point along the screw axis) onto
     the table plane z = screw_plane_z in the base frame using camera_info + TF.
  3. Build a straight-down tool orientation whose finger-closing direction is
     perpendicular to the screw's principal axis.
  4. Pre-grasp (above) -> descend -> close gripper -> lift.
"""
import math
import time
from collections import deque
from threading import Lock, Thread

import numpy as np
import rclpy
from rclpy.action import ActionClient
from rclpy.executors import SingleThreadedExecutor
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from rclpy.time import Time

from tf2_ros import Buffer, TransformException, TransformListener

from common_interfaces_merlab.srv import SendPose
from control_msgs.action import GripperCommand
from sensor_msgs.msg import CameraInfo
from screw_sorting_interfaces.msg import Screw


class GripperExecutor(Node):

    def __init__(self):
        super().__init__('gripper_executor')

        # ---------------- Parameters ----------------
        self.declare_parameter('screw_topic', '/perception/screw_location')
        self.declare_parameter('camera_info_topic', '/camera/camera/color/camera_info')
        self.declare_parameter('camera_frame', 'camera_color_optical_frame')
        self.declare_parameter('base_frame', 'base_link')

        # Height (base frame) of the plane the screw centroid lies on.
        self.declare_parameter('screw_plane_z', 0.0)
        # Height (base frame) of the fingertips at the grasp.
        self.declare_parameter('grasp_z', 0.0)
        self.declare_parameter('pregrasp_distance', 0.15)
        self.declare_parameter('lift_height', 0.20)
        # Distance from tool0 (what /send_pose commands) to the fingertips.
        # self.declare_parameter('tcp_offset', 0.157)
        self.declare_parameter('tcp_offset', 0.107)
        # Abort if the fingertips would go below this z.
        self.declare_parameter('min_z', -0.005)

        # Which tool axis the fingers close along: 'x' or 'y'. Verify on hardware.
        self.declare_parameter('finger_close_axis', 'y')
        # Extra yaw (deg) added to the closing direction for calibration.
        self.declare_parameter('yaw_offset_deg', 0.0)

        # Detection averaging.
        self.declare_parameter('num_samples', 10)
        self.declare_parameter('max_position_spread', 0.01)  # m, std dev
        self.declare_parameter('detection_timeout', 30.0)    # s

        self.declare_parameter(
            'gripper_command_topic', '/gen3_lite_2f_gripper_controller/gripper_cmd')
        self.declare_parameter('gripper_open_position', 0.0)
        self.declare_parameter('gripper_closed_position', 0.80)

        # ---------------- Clients ----------------
        self.pose_client = self.create_client(SendPose, 'send_pose')
        self.gripper_client = ActionClient(
            self, GripperCommand,
            self.get_parameter('gripper_command_topic').value)

        # ---------------- TF / camera ----------------
        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)
        self.intrinsics = None  # (fx, fy, cx, cy)

        self.create_subscription(
            CameraInfo,
            self.get_parameter('camera_info_topic').value,
            self.camera_info_callback,
            qos_profile_sensor_data,
        )

        # ---------------- Screw samples ----------------
        self.accepting = False
        self.samples = deque(maxlen=int(self.get_parameter('num_samples').value))
        self.samples_lock = Lock()

        self.create_subscription(
            Screw,
            self.get_parameter('screw_topic').value,
            self.screw_callback,
            10,
        )

    # =====================================================
    # Utilities
    # =====================================================

    def wait_for_services(self, timeout=30.0):
        self.get_logger().info('Waiting for send_pose...')
        if not self.pose_client.wait_for_service(timeout_sec=timeout):
            self.get_logger().error('Service send_pose is unavailable.')
            return False
        return True

    def wait_for_future(self, future, timeout, service_name):
        """Wait on a future while a background executor spins the node."""
        deadline = time.monotonic() + timeout
        while rclpy.ok() and not future.done():
            if time.monotonic() >= deadline:
                self.get_logger().error(f'Timed out waiting for {service_name}.')
                return None
            time.sleep(0.01)

        if not future.done():
            return None
        try:
            return future.result()
        except Exception as exc:
            self.get_logger().error(f'Call to {service_name} failed: {exc}')
            return None

    @staticmethod
    def quat_to_matrix(x, y, z, w):
        n = math.sqrt(x * x + y * y + z * z + w * w)
        x, y, z, w = x / n, y / n, z / n, w / n
        return np.array([
            [1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)],
            [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
            [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)],
        ])

    @staticmethod
    def matrix_to_quat(R):
        """Rotation matrix -> quaternion [x, y, z, w]."""
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

    # =====================================================
    # Perception input
    # =====================================================

    def camera_info_callback(self, msg):
        if self.intrinsics is None:
            k = msg.k
            self.intrinsics = (k[0], k[4], k[2], k[5])
            self.get_logger().info(
                f'Camera intrinsics received: fx={k[0]:.1f} fy={k[4]:.1f} '
                f'cx={k[2]:.1f} cy={k[5]:.1f}')

    def pixel_to_plane(self, u, v, R, t, plane_z):
        """Intersect the camera ray through pixel (u, v) with z = plane_z (base frame)."""
        fx, fy, cx, cy = self.intrinsics
        ray_cam = np.array([(u - cx) / fx, (v - cy) / fy, 1.0])
        ray_base = R @ ray_cam

        if abs(ray_base[2]) < 1e-6:
            return None
        s = (plane_z - t[2]) / ray_base[2]
        if s <= 0.0:
            return None
        return t + s * ray_base

    def screw_callback(self, msg):
        """Convert a pixel-space detection into a base-frame (x, y, axis_yaw) sample."""
        if not self.accepting or self.intrinsics is None:
            return

        base_frame = self.get_parameter('base_frame').value
        camera_frame = self.get_parameter('camera_frame').value
        plane_z = float(self.get_parameter('screw_plane_z').value)

        try:
            tf = self.tf_buffer.lookup_transform(base_frame, camera_frame, Time())
        except TransformException as exc:
            self.get_logger().warning(
                f'TF {base_frame} <- {camera_frame} unavailable: {exc}',
                throttle_duration_sec=5.0)
            return

        q = tf.transform.rotation
        tr = tf.transform.translation
        R = self.quat_to_matrix(q.x, q.y, q.z, q.w)
        t = np.array([tr.x, tr.y, tr.z])

        u = msg.centroid_position.x
        v = msg.centroid_position.y
        angle = msg.angle  # long-axis direction in the image, radians

        # Project the centroid and a second point along the screw axis so the
        # axis direction accounts for perspective / camera tilt.
        step = max(20.0, float(msg.length))
        p0 = self.pixel_to_plane(u, v, R, t, plane_z)
        p1 = self.pixel_to_plane(
            u + step * math.cos(angle), v + step * math.sin(angle), R, t, plane_z)

        if p0 is None or p1 is None:
            self.get_logger().warning(
                'Pixel ray does not hit the screw plane; ignoring sample.',
                throttle_duration_sec=5.0)
            return

        axis = p1[:2] - p0[:2]
        if np.linalg.norm(axis) < 1e-6:
            return
        axis_yaw = math.atan2(axis[1], axis[0])

        with self.samples_lock:
            self.samples.append((p0[0], p0[1], axis_yaw))

    def wait_for_screw_estimate(self):
        """
        Wait for a full window of consistent samples.
        Returns (x, y, axis_yaw) in the base frame, or None on timeout.
        """
        n = int(self.get_parameter('num_samples').value)
        max_spread = float(self.get_parameter('max_position_spread').value)
        timeout = float(self.get_parameter('detection_timeout').value)

        with self.samples_lock:
            self.samples = deque(maxlen=n)

        self.get_logger().info('Waiting for screw detections...')
        self.accepting = True
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
                        # Axis is a line (period pi): average the doubled angle.
                        yaw = 0.5 * math.atan2(
                            np.mean(np.sin(2.0 * arr[:, 2])),
                            np.mean(np.cos(2.0 * arr[:, 2])))
                        self.get_logger().info(
                            f'Screw estimate: x={x:.3f} y={y:.3f} '
                            f'axis_yaw={math.degrees(yaw):.1f} deg '
                            f'(spread {spread * 1000:.1f} mm)')
                        return float(x), float(y), float(yaw)

                time.sleep(0.05)

            if self.intrinsics is None:
                self.get_logger().error('No camera_info received.')
            return None
        finally:
            self.accepting = False

    # =====================================================
    # Motion
    # =====================================================

    def move_to_pose(self, x, y, z, qx, qy, qz, qw):
        """Move tool0 via /send_pose (meters, base frame, quaternion xyzw)."""
        if not self.pose_client.service_is_ready():
            self.get_logger().error('/send_pose service is unavailable.')
            return False

        request = SendPose.Request()
        request.pose.position.x = float(x)
        request.pose.position.y = float(y)
        request.pose.position.z = float(z)
        request.pose.orientation.x = float(qx)
        request.pose.orientation.y = float(qy)
        request.pose.orientation.z = float(qz)
        request.pose.orientation.w = float(qw)

        self.get_logger().info(f'Moving to ({x:.3f}, {y:.3f}, {z:.3f})')

        future = self.pose_client.call_async(request)
        response = self.wait_for_future(future, timeout=180.0, service_name='/send_pose')

        if response is None or not response.success:
            self.get_logger().error('Pose motion failed.')
            return False

        self.get_logger().info('Pose motion completed.')
        return True

    def set_gripper(self, close):
        """Control the Kinova Gen3 Lite integrated gripper."""
        close_val = float(self.get_parameter('gripper_closed_position').value)
        open_val = float(self.get_parameter('gripper_open_position').value)
        position = close_val if close else open_val

        self.get_logger().info(f'{"Closing" if close else "Opening"} gripper.')

        if not self.gripper_client.wait_for_server(timeout_sec=10.0):
            self.get_logger().error('Gen3 Lite gripper action server unavailable.')
            return False

        goal = GripperCommand.Goal()
        goal.command.position = position
        goal.command.max_effort = 100.0

        goal_handle = self.wait_for_future(
            self.gripper_client.send_goal_async(goal),
            timeout=10.0, service_name='Gen3 Lite gripper')

        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().error('Gripper command rejected.')
            return False

        result = self.wait_for_future(
            goal_handle.get_result_async(),
            timeout=15.0, service_name='Gen3 Lite gripper movement')

        if result is None:
            self.get_logger().error('Gripper movement timed out.')
            return False

        self.get_logger().info('Gripper movement completed.')
        return True

    # =====================================================
    # Grasp geometry
    # =====================================================

    def top_down_orientation(self, axis_yaw):
        """
        Quaternion [x, y, z, w] for a straight-down tool (tool z = -Z_base)
        whose finger-closing direction is perpendicular to the screw axis.
        """
        offset = math.radians(float(self.get_parameter('yaw_offset_deg').value))
        close_axis = str(self.get_parameter('finger_close_axis').value).lower()
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

        close_yaw = axis_yaw + math.pi / 2.0 + offset
        x_t, y_t = build(close_yaw)

        # The gripper is symmetric: pick the equivalent solution that keeps
        # the tool x axis pointing away from the robot (less wrist rotation).
        if x_t[0] < 0.0:
            x_t, y_t = build(close_yaw + math.pi)

        R = np.column_stack((x_t, y_t, z_t))
        return self.matrix_to_quat(R)

    # =====================================================
    # Grasp execution
    # =====================================================

    def execute_grasp(self, x, y, axis_yaw):
        """Pre-grasp, vertical descent, close, lift."""
        pre_dist = float(self.get_parameter('pregrasp_distance').value)
        lift = float(self.get_parameter('lift_height').value)
        tcp = float(self.get_parameter('tcp_offset').value)
        grasp_z = float(self.get_parameter('grasp_z').value)
        min_z = float(self.get_parameter('min_z').value)

        if pre_dist < 0.0:
            self.get_logger().error('pregrasp_distance cannot be negative.')
            return False

        if grasp_z < min_z:
            self.get_logger().error(
                f'grasp_z={grasp_z:.3f} is below min_z={min_z:.3f}; aborting.')
            return False

        q = self.top_down_orientation(axis_yaw)

        # Tool points straight down, so tool0 sits tcp_offset above the fingertips.
        grasp_tool0 = np.array([x, y, grasp_z + tcp])
        pregrasp_tool0 = grasp_tool0 + np.array([0.0, 0.0, pre_dist])
        lifted_tool0 = grasp_tool0 + np.array([0.0, 0.0, lift])

        self.get_logger().info(
            f'Screw at ({x:.3f}, {y:.3f}), axis {math.degrees(axis_yaw):.1f} deg; '
            f'tool quat={np.round(q, 3)}')
        self.get_logger().info(f'Pre-grasp tool0: {np.round(pregrasp_tool0, 3)}')
        self.get_logger().info(f'Grasp tool0:     {np.round(grasp_tool0, 3)}')
        self.get_logger().info(f'Lift tool0:      {np.round(lifted_tool0, 3)}')

        def go(p):
            return self.move_to_pose(p[0], p[1], p[2], q[0], q[1], q[2], q[3])

        # 1. Pre-grasp (above the screw, already aligned).
        if not go(pregrasp_tool0):
            return False

        # 2. Straight down.
        if not go(grasp_tool0):
            self.get_logger().error('Final approach motion failed.')
            return False

        # 3. Close.
        if not self.set_gripper(close=True):
            return False

        # 4. Lift.
        if not go(lifted_tool0):
            self.get_logger().error('Lift motion failed.')
            return False

        return True

    # =====================================================
    # Main sequence
    # =====================================================

    def run(self):
        if not self.wait_for_services(timeout=30.0):
            return

        # 1. Open gripper.
        if not self.set_gripper(close=False):
            return

        # 2. Estimate the screw pose (arm stays at its observation pose).
        estimate = self.wait_for_screw_estimate()
        if estimate is None:
            self.get_logger().warning('No stable screw estimate. Exiting.')
            return
        x, y, axis_yaw = estimate

        # 3. Execute the open-loop grasp.
        if self.execute_grasp(x, y, axis_yaw):
            self.get_logger().info('Grasp and lift completed successfully.')
        else:
            self.get_logger().error('Grasp sequence aborted.')


def main(args=None):
    rclpy.init(args=args)
    node = GripperExecutor()

    executor = SingleThreadedExecutor()
    executor.add_node(node)
    spin_thread = Thread(target=executor.spin, daemon=True)
    spin_thread.start()

    try:
        node.run()
    except KeyboardInterrupt:
        node.get_logger().info('Interrupted by user.')
    except Exception as exc:
        node.get_logger().error(f'Unexpected error during grasp execution: {exc}')
    finally:
        executor.shutdown()
        spin_thread.join(timeout=2.0)
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()