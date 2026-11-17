#!/usr/bin/env python3
import time
from threading import Thread

import numpy as np
import rclpy

from builtin_interfaces.msg import Duration
from common_interfaces_merlab.srv import (
    SendJointTrajectoryPoint,
    SendPose,
)
from geometry_msgs.msg import PoseStamped
from rclpy.executors import SingleThreadedExecutor
from rclpy.node import Node

from rclpy.action import ActionClient
from control_msgs.action import GripperCommand



# HOME_JOINTS = [0.471, 0.0523, 1.606, -1.64, -1.518, -1.134]


class GripperExecutor(Node):

    def __init__(self):
        super().__init__('gripper_executor')

        # Parameters
        self.declare_parameter('grasp_pose_topic','pc_processing_template/grasp_pose')
        self.declare_parameter('base_frame', 'base_link')
        self.declare_parameter('pregrasp_distance', 0.20)
        self.declare_parameter('lift_height', 0.20)
        self.declare_parameter('tcp_offset', 0.162)
        self.declare_parameter('min_z', 0.0)
        self.declare_parameter('gripper_command_topic', '/gen3_lite_2f_gripper_controller/gripper_cmd')
        self.declare_parameter('gripper_open_position', 0.0)
        self.declare_parameter('gripper_closed_position', 0.80)

        self.pose_client = self.create_client(
            SendPose,
            'send_pose',
        )

        self.joint_client = self.create_client(
            SendJointTrajectoryPoint,
            'send_joint_trajectory_point',
        )

        self.gripper_client = ActionClient(
            self,
            GripperCommand,
            self.get_parameter('gripper_command_topic').value,
        )

        self.accepting = False
        self.grasp_pose = None

        self.create_subscription(
            PoseStamped,
            self.get_parameter('grasp_pose_topic').value,
            self.grasp_pose_callback,
            1,
        )

    def wait_for_services(self, timeout=30.0):
        """Wait for all services provided by ArmServices."""

        services = [
            (self.pose_client, 'send_pose'),
            (self.joint_client, 'send_joint_trajectory_point'),
        ]

        self.get_logger().info(
            'Waiting for ArmServices...'
        )

        for client, name in services:

            self.get_logger().info(
                f'Waiting for {name}...'
            )

            if not client.wait_for_service(
                timeout_sec=timeout
            ):
                self.get_logger().error(
                    f'Service {name} is unavailable.'
                )
                return False

            self.get_logger().info(
                f'{name} is available.'
            )

        self.get_logger().info(
            'All ArmServices are available.'
        )

        return True

    def wait_for_future(self, future, timeout, service_name):
        """
        Wait for an asynchronous service response.

        A background executor spins the node, so this method
        does not call spin_until_future_complete().
        """

        deadline = time.monotonic() + timeout

        while rclpy.ok() and not future.done():
            if time.monotonic() >= deadline:
                self.get_logger().error(
                    f'Timed out waiting for {service_name}.'
                )
                return None

            time.sleep(0.01)

        if not future.done():
            return None

        try:
            return future.result()

        except Exception as exc:
            self.get_logger().error(
                f'Service call to {service_name} failed: {exc}'
            )
            return None

    # Grasp pose handling

    def grasp_pose_callback(self, msg):
        """Accept the first grasp pose expressed in the base frame."""

        if not self.accepting or self.grasp_pose is not None:
            return

        base_frame = self.get_parameter('base_frame').value

        if msg.header.frame_id != base_frame:
            self.get_logger().warning(
                f'Ignoring grasp pose in frame '
                f'{msg.header.frame_id}; expected {base_frame}.',
                throttle_duration_sec=5.0,
            )
            return

        self.grasp_pose = msg.pose

        self.get_logger().info(
            'Received grasp pose in the base frame.'
        )

    def wait_for_grasp_pose(self):
        """Wait until a valid grasp pose is received."""

        self.get_logger().info('Waiting for grasp pose...')

        self.grasp_pose = None
        self.accepting = True

        try:
            while rclpy.ok() and self.grasp_pose is None:
                time.sleep(0.05)

            return self.grasp_pose

        finally:
            self.accepting = False

    # Cartesian motion

    def move_to_pose(
        self,
        x,
        y,
        z,
        qx,
        qy,
        qz,
        qw,
    ):
        """
        Move the end effector using /send_pose.

        Position is in meters relative to the configured base frame.
        Orientation is the quaternion [qx, qy, qz, qw].
        """

        if not self.pose_client.service_is_ready():
            self.get_logger().error(
                '/send_pose service is unavailable.'
            )
            return False

        request = SendPose.Request()

        request.pose.position.x = float(x)
        request.pose.position.y = float(y)
        request.pose.position.z = float(z)

        request.pose.orientation.x = float(qx)
        request.pose.orientation.y = float(qy)
        request.pose.orientation.z = float(qz)
        request.pose.orientation.w = float(qw)

        self.get_logger().info(
            f'Moving to position ({x:.3f}, {y:.3f}, {z:.3f})'
        )

        try:
            future = self.pose_client.call_async(request)

            response = self.wait_for_future(
                future,
                timeout=180.0,
                service_name='/send_pose',
            )

            if response is None or not response.success:
                self.get_logger().error(
                    'Pose motion failed.'
                )
                return False

            self.get_logger().info(
                'Pose motion completed successfully.'
            )
            return True

        except Exception as exc:
            self.get_logger().error(
                f'Pose service call failed: {exc}'
            )
            return False

    # Joint motion

    # def move_home(self, positions=HOME_JOINTS, move_time=5.0):
    #     """Move the robot to its predefined home configuration."""

    #     if not self.joint_client.service_is_ready():
    #         self.get_logger().error(
    #             '/move_traj_single_point is unavailable.'
    #         )
    #         return False

    #     request = SendJointTrajectoryPoint.Request()

    #     request.goal_point.positions = list(positions)
    #     request.goal_point.time_from_start = Duration(
    #         sec=int(move_time),
    #         nanosec=int((move_time % 1.0) * 1e9),
    #     )

    #     self.get_logger().info(
    #         'Moving to home configuration.'
    #     )

    #     try:
    #         future = self.joint_client.call_async(request)

    #         response = self.wait_for_future(
    #             future,
    #             timeout=120.0,
    #             service_name='/move_traj_single_point',
    #         )

    #         if response is None or not response.success:
    #             self.get_logger().error(
    #                 'Home motion failed.'
    #             )
    #             return False

    #         self.get_logger().info(
    #             'Home motion completed successfully.'
    #         )
    #         return True

    #     except Exception as exc:
    #         self.get_logger().error(
    #             f'Joint service call failed: {exc}'
    #         )
    #         return False

    # Gripper control

    def set_gripper(self, close):
        """Control the Kinova Gen3 Lite integrated gripper."""
        close_val = float(self.get_parameter('gripper_closed_position').value)
        open_val = float(self.get_parameter('gripper_open_position').value)


        position = close_val if close else open_val

        self.get_logger().info(
            f'{"Closing" if close else "Opening"} gripper.'
        )

        if not self.gripper_client.wait_for_server(
            timeout_sec=10.0
        ):
            self.get_logger().error(
                'Gen3 Lite gripper action server unavailable.'
            )
            return False

        goal = GripperCommand.Goal()

        goal.command.position = position
        goal.command.max_effort = 100.0

        future = self.gripper_client.send_goal_async(goal)

        goal_handle = self.wait_for_future(
            future,
            timeout=10.0,
            service_name='Gen3 Lite gripper',
        )

        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().error(
                'Gripper command rejected.'
            )
            return False

        result_future = goal_handle.get_result_async()

        result = self.wait_for_future(
            result_future,
            timeout=15.0,
            service_name='Gen3 Lite gripper movement',
        )

        if result is None:
            self.get_logger().error(
                'Gripper movement timed out.'
            )
            return False

        self.get_logger().info(
            'Gripper movement completed.'
        )

        return True


    # Grasp geometry

    @staticmethod
    def tool_z_axis(q):
        """
        Return the tool's local Z axis expressed in the base frame.

        Quaternion convention: [qx, qy, qz, qw].
        """

        x, y, z, w = q

        return np.array([
            2.0 * (x * z + y * w),
            2.0 * (y * z - x * w),
            1.0 - 2.0 * (x * x + y * y),
        ])


    # Grasp execution

    def execute_grasp(self, pose):
        """Execute pre-grasp, approach, grasp, and lift."""

        p = np.array([
            pose.position.x,
            pose.position.y,
            pose.position.z,
        ])

        q = np.array([
            pose.orientation.x,
            pose.orientation.y,
            pose.orientation.z,
            pose.orientation.w,
        ])

        q_norm = np.linalg.norm(q)

        if q_norm < 1e-8:
            self.get_logger().error(
                'Invalid grasp orientation: zero-length quaternion.'
            )
            return False

        q = q / q_norm

        approach = self.tool_z_axis(q)

        pre_dist = float(
            self.get_parameter('pregrasp_distance').value
        )
        lift = float(
            self.get_parameter('lift_height').value
        )
        tcp = float(
            self.get_parameter('tcp_offset').value
        )
        min_z = float(
            self.get_parameter('min_z').value
        )


        if pre_dist < 0.0:
            self.get_logger().error(
                'pregrasp_distance cannot be negative.'
            )
            return False

        # Convert the detected grasp pose to the tool0 target.
        grasp_tool0 = p - approach * tcp

        # Position before beginning the approach.
        pregrasp_tool0 = grasp_tool0 - approach * pre_dist

        # Lift vertically in the base frame.
        lifted_tool0 = grasp_tool0 + np.array([
            0.0,
            0.0,
            lift,
        ])

        # Check that the grasp and pre-grasp are above the minimum Z.
        if min(grasp_tool0[2], pregrasp_tool0[2]) < min_z:
            self.get_logger().error(
                f'Target below min_z={min_z:.3f} m '
                f'(grasp z={grasp_tool0[2]:.3f}, '
                f'pre-grasp z={pregrasp_tool0[2]:.3f}); aborting.'
            )
            return False

        self.get_logger().info(
            f'Approach vector: {np.round(approach, 3)}'
        )

        self.get_logger().info(
            f'Pre-grasp position: '
            f'{np.round(pregrasp_tool0, 3)}'
        )

        self.get_logger().info(
            f'Grasp position: {np.round(grasp_tool0, 3)}'
        )

        self.get_logger().info(
            f'Lift position: {np.round(lifted_tool0, 3)}'
        )

        # 1. Move to pre-grasp.
        if not self.move_to_pose(
            x=pregrasp_tool0[0],
            y=pregrasp_tool0[1],
            z=pregrasp_tool0[2],
            qx=q[0],
            qy=q[1],
            qz=q[2],
            qw=q[3],
        ):
            return False

        # 2. Approach the object using position control.
        self.get_logger().info(
            'Moving from pre-grasp to grasp position.'
        )

        if not self.move_to_pose(
            x=grasp_tool0[0],
            y=grasp_tool0[1],
            z=grasp_tool0[2],
            qx=q[0],
            qy=q[1],
            qz=q[2],
            qw=q[3],
        ):
            self.get_logger().error(
                'Final approach motion failed.'
            )
            return False



        # 3. Close the gripper.
        if not self.set_gripper(close=True):
            return False

        # 4. Lift the grasped object.
        if not self.move_to_pose(
            x=lifted_tool0[0],
            y=lifted_tool0[1],
            z=lifted_tool0[2],
            qx=q[0],
            qy=q[1],
            qz=q[2],
            qw=q[3],
        ):
            self.get_logger().error(
                'Lift motion failed.'
            )
            return False

        return True

    # ---------------------------------------------------------
    # Main sequence
    # ---------------------------------------------------------

    def run(self):
        """Run the complete grasp sequence."""

        if not self.wait_for_services(timeout=30.0):
            return

        self.get_logger().info(
            'All required robot services are ready.'
        )

        # 1. Open gripper.
        if not self.set_gripper(close=False):
            return

        # 2. Move to home.
        # if not self.move_home():
        #     return

        # 3. Wait for a grasp pose from the perception node.
        pose = self.wait_for_grasp_pose()

        if pose is None:
            self.get_logger().warning(
                'No grasp pose received. Exiting.'
            )
            return

        # 4. Execute the grasp.
        if self.execute_grasp(pose):
            self.get_logger().info(
                'Grasp and lift completed successfully.'
            )
        else:
            self.get_logger().error(
                'Grasp sequence aborted.'
            )


def main(args=None):
    rclpy.init(args=args)

    node = GripperExecutor()

    # Run callbacks in a background executor thread.
    executor = SingleThreadedExecutor()
    executor.add_node(node)

    spin_thread = Thread(
        target=executor.spin,
        daemon=True,
    )
    spin_thread.start()

    try:
        node.run()

    except KeyboardInterrupt:
        node.get_logger().info(
            'Interrupted by user.'
        )

    except Exception as exc:
        node.get_logger().error(
            f'Unexpected error during grasp execution: {exc}'
        )

    finally:
        executor.shutdown()
        spin_thread.join(timeout=2.0)

        node.destroy_node()

        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()