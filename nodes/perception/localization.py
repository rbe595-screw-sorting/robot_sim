#!/usr/bin/env python3

import asyncio

import rclpy
from rclpy.node import Node
from rclpy.action import ActionServer, GoalResponse, CancelResponse
from rclpy.callback_groups import ReentrantCallbackGroup
from rclpy.executors import MultiThreadedExecutor
from rclpy.time import Time
from rclpy.duration import Duration

from tf2_ros import Buffer, TransformListener, TransformException

from geometry_msgs.msg import PoseStamped

from screw_sorting_interfaces.action import Relocalize
from screw_sorting_interfaces.srv import GetLocations

import time

import threading
from common_interfaces_merlab.srv import SendPose
from rclpy.callback_groups import ReentrantCallbackGroup


class LocalizationNode(Node):

    def __init__(self):

        super().__init__('localization_node')

        self.callback_group = ReentrantCallbackGroup()

        self.base_frame = 'base_link'

        self.arm_client_group = ReentrantCallbackGroup()

        self.arm_pose_client = self.create_client(
            SendPose,
            '/send_pose',
            callback_group=self.arm_client_group
        )

        # --------------------------------------------------
        # Configuration
        # --------------------------------------------------

        self.world_frame = 'base_link'

        # AprilTag-localized rigid bodies.
        #
        # Keys: IDs used by the state machine.
        # Values: TF frames representing the physical rigid bodies.

        self.object_frames = {
            'bucket': 'bucket',
            'scoop': 'scoop',
            'bin': 'bin',
        }


        # Relevant feature frames associated with each rigid body.
        #
        # All frames are defined relative to their respective rigid body
        # through static transforms.

        self.feature_frames = {

            # 'bucket': [
            #     'bucket_center',
            # ],

            'scoop': [
                'scoop_grasp',
            ],
        }
        self.feature_frames['bin'] = [
            'bin_drop_location'
        ] + [
            f'bin_container_{row}_{col}'
            for row in range(3)
            for col in range(4)
        ]


        # --------------------------------------------------
        # TF2
        # --------------------------------------------------

        self.tf_buffer = Buffer()

        self.tf_listener = TransformListener(
            self.tf_buffer,
            self,
            spin_thread=False
        )

        # --------------------------------------------------
        # Action Server
        # --------------------------------------------------

        self.localization_action = ActionServer(
            self,
            Relocalize,
            '/localization/relocalize',
            execute_callback=self.execute_relocalization,
            goal_callback=self.goal_callback,
            cancel_callback=self.cancel_callback,
            callback_group=self.callback_group
        )

        # --------------------------------------------------
        # Service Server
        # --------------------------------------------------

        self.location_service = self.create_service(
            GetLocations,
            '/localization/get_locations',
            self.get_locations_callback,
            callback_group=self.callback_group
        )

        self.get_logger().info(
            'Localization node initialized.'
        )
    async def ros_sleep(self, duration_sec):

        future = rclpy.task.Future()

        timer = self.create_timer(
            duration_sec,
            lambda: future.set_result(True)
        )

        try:
            await future
        finally:
            timer.cancel()
            self.destroy_timer(timer)
    # ======================================================
    # ACTION SERVER
    # ======================================================

    def goal_callback(self, goal_request):

        if goal_request.mode not in [
            Relocalize.Goal.GLOBAL,
            Relocalize.Goal.SELECTIVE
        ]:
            self.get_logger().warning(
                'Rejected localization request: invalid mode.'
            )
            return GoalResponse.REJECT

        if goal_request.mode == Relocalize.Goal.SELECTIVE:

            if not goal_request.target_ids:
                self.get_logger().warning(
                    'Selective localization requires target IDs.'
                )
                return GoalResponse.REJECT

            for object_id in goal_request.target_ids:

                if object_id not in self.object_frames:
                    self.get_logger().warning(
                        f'Unknown localization target: {object_id}'
                    )
                    return GoalResponse.REJECT

        return GoalResponse.ACCEPT

    def cancel_callback(self, goal_handle):

        self.get_logger().info(
            'Localization cancellation requested.'
        )

        return CancelResponse.ACCEPT

    async def execute_relocalization(self, goal_handle):

        request = goal_handle.request

        result = Relocalize.Result()

        # Determine localization targets.

        if request.mode == Relocalize.Goal.GLOBAL:

            targets = list(self.object_frames.keys())

        else:

            targets = list(request.target_ids)

        self.get_logger().info(
            f'Starting localization for: {targets}'
        )

        localized = set()

        # --------------------------------------------------
        # Generate observation poses
        # --------------------------------------------------

        observation_poses = self.generate_scan_poses(
            targets=targets,
            mode=request.mode
        )

        if not observation_poses:

            result.success = False
            result.failed_ids = targets
            result.message = 'No observation poses generated.'

            goal_handle.abort()

            return result

        # --------------------------------------------------
        # Execute localization routine
        # --------------------------------------------------

        for i, pose in enumerate(observation_poses):

            if goal_handle.is_cancel_requested:

                goal_handle.canceled()

                result.success = False
                result.localized_ids = list(localized)
                result.failed_ids = [
                    obj for obj in targets if obj not in localized
                ]
                result.message = 'Localization cancelled.'

                return result

            self.get_logger().info(
                f'Moving to observation pose {i + 1}/'
                f'{len(observation_poses)}'
            )

            # Move robot to observation pose.
            # movement_success = self.move_to_observation_pose(pose)
            movement_success = await self.move_to_observation_pose(pose)
            # movement_success = True

            if not movement_success:

                self.get_logger().warning(
                    'Failed to reach observation pose.'
                )

                continue

            # Give the camera/detector time to acquire observations.
            # time.sleep(1.0)
            await self.ros_sleep(1.0)
            # Determine which requested objects are now available.
            for object_id in targets:

                if object_id in localized:
                    continue

                frame = self.object_frames[object_id]

                if self.transform_available(frame):

                    localized.add(object_id)

            # --------------------------------------------------
            # Publish action feedback
            # --------------------------------------------------

            feedback = Relocalize.Feedback()

            feedback.current_target = (
                targets[-1] if len(targets) > 0 else ''
            )

            feedback.targets_completed = len(localized)

            feedback.targets_remaining = (
                len(targets) - len(localized)
            )

            goal_handle.publish_feedback(feedback)

            # Stop scanning early if everything was found.

            if len(localized) == len(targets):
                break

        # --------------------------------------------------
        # Final result
        # --------------------------------------------------

        failed = [
            obj for obj in targets
            if obj not in localized
        ]

        result.localized_ids = list(localized)
        result.failed_ids = failed

        result.success = len(failed) == 0

        if result.success:

            result.message = 'All requested objects localized.'

            goal_handle.succeed()

        else:

            result.message = (
                f'Localization incomplete. '
                f'{len(localized)}/{len(targets)} objects found.'
            )

            goal_handle.abort()

        self.get_logger().info(result.message)

        return result

    # ======================================================
    # SERVICE SERVER
    # ======================================================

    def get_locations_callback(self, request, response):

        # Determine which objects to retrieve.

        if request.get_all:

            object_ids = list(self.object_frames.keys())

        else:

            object_ids = list(request.object_ids)

        response.success = True

        # --------------------------------------------------
        # Retrieve rigid-body transforms
        # --------------------------------------------------

        for object_id in object_ids:

            if object_id not in self.object_frames:

                response.unavailable_ids.append(object_id)
                response.success = False

                continue

            frame = self.object_frames[object_id]

            transform = self.get_world_transform(frame)

            if transform is None:

                response.unavailable_ids.append(object_id)
                response.success = False

                continue

            response.object_ids.append(object_id)

            response.object_transforms.append(transform)

            # --------------------------------------------------
            # Retrieve associated feature transforms
            # --------------------------------------------------

            features = self.feature_frames.get(
                object_id, []
            )

            for feature_frame in features:

                feature_transform = self.get_world_transform(
                    feature_frame
                )

                if feature_transform is None:

                    response.unavailable_ids.append(
                        feature_frame
                    )

                    response.success = False

                    continue

                response.feature_frame_ids.append(
                    feature_frame
                )

                response.feature_transforms.append(
                    feature_transform
                )

        return response

    # ======================================================
    # TF2 UTILITIES
    # ======================================================

    def transform_available(self, frame):

        try:

            return self.tf_buffer.can_transform(
                self.world_frame,
                frame,
                Time(),
                timeout=Duration(seconds=0.1)
            )

        except TransformException:

            return False

    def get_world_transform(self, frame):

        try:

            return self.tf_buffer.lookup_transform(
                self.world_frame,
                frame,
                Time()
            )

        except TransformException as e:

            self.get_logger().debug(
                f'Could not retrieve transform for {frame}: {e}'
            )

            return None

    # ======================================================
    # LOCALIZATION ROUTINE
    # ======================================================

    def generate_scan_poses(self, targets, mode):

        """
        TODO: Implement observation pose generation.

        GLOBAL:
            Generate poses covering the entire workspace.

        SELECTIVE:
            Generate poses based on the previous locations
            of the requested rigid bodies.

        Returns:
            List[PoseStamped]
        """

        observation_poses = [(-0.2, -0.3, 0.3, 1.0, 0.0, 0.0, 0.0)]
        poses = []

        for x, y, z, qx, qy, qz, qw in observation_poses:

            pose = PoseStamped()
            pose.header.frame_id = self.base_frame
            pose.header.stamp = self.get_clock().now().to_msg()

            pose.pose.position.x = x
            pose.pose.position.y = y
            pose.pose.position.z = z

            pose.pose.orientation.x = qx
            pose.pose.orientation.y = qy
            pose.pose.orientation.z = qz
            pose.pose.orientation.w = qw

            poses.append(pose)




        # TODO: Implement scan planning.

        return poses


    async def move_to_observation_pose(self, pose: PoseStamped):

        if pose.header.frame_id != self.base_frame:
            return False

        if not self.arm_pose_client.wait_for_service(timeout_sec=5.0):
            return False

        request = SendPose.Request()
        request.pose = pose.pose

        self.get_logger().info('Calling /send_pose...')

        try:
            future = self.arm_pose_client.call_async(request)

            response = await future

            if response.success:
                self.get_logger().info('Observation pose reached.')
                return True

            return False

        except Exception as e:
            self.get_logger().error(f'/send_pose failed: {e}')
            return False

def main(args=None):

    rclpy.init(args=args)

    node = LocalizationNode()

    executor = MultiThreadedExecutor(num_threads=4)

    executor.add_node(node)

    try:

        executor.spin()

    except KeyboardInterrupt:
        pass

    finally:

        node.destroy_node()

        rclpy.shutdown()


if __name__ == '__main__':
    main()