#!/usr/bin/env python3
import threading
import cv2
import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.executors import MultiThreadedExecutor
from cv_bridge import CvBridge
from sensor_msgs.msg import Image
from rclpy.qos import qos_profile_sensor_data
from control_msgs.action import GripperCommand
import numpy as np
from helpers.robot_motion import ArmMover, wait_for
import time
from rclpy.callback_groups import ReentrantCallbackGroup

from rclpy.action import ActionClient

from screw_sorting_interfaces.action import Relocalize
from screw_sorting_interfaces.srv import GetLocations, StartGrasp

# ROBOT CONFIGURATION

arm_group = "arm"
base_frame = "base_link"
ee_link = "end_effector_link"

gripper_action = "/gen3_lite_2f_gripper_controller/gripper_cmd"


# WAYPOINTS
# Format: x, y, z, qx, qy, qz, qw
# Position in meters, orientation in quaternion.

waypoints = {
    "phase2_home" : (0.3, 0.06, 0.4, 1.0, 0.0, 0.0, 0.0),
    "phase2_floor" : (0.3, 0.06, 0.1142, 1.0, 0.0, 0.0, 0.0),
    "screwattempt" : (0.201, -0.110, 0.1147, 1.0, 0.0, 0.0, 0.0)
    # "bin_location_guess" : (-0.2, -0.3, 0.3, 1.0, 0.0, 0.0, 0.0)
}


# MOTION PLANNING CONFIGURATION

# Each waypoint can use either:
#   "cartesian"  - Straight-line end-effector movement
#   "rrtconnect" - General collision-aware motion planning

waypoint_motion = {

    "phase2_home": "rrtconnect",
    "phase2_floor": "rrtconnect",
    "screwattempt" : "rrtconnect"
    # "bin_location_guess": "rrtconnect"

}


# PHASE 2 NODE

class Phase_2(Node):

    def __init__(self):

        super().__init__("phase_2")

        self.motion_client_group = ReentrantCallbackGroup()


        # MoveIt motion planning
        self.arm_mover = ArmMover(
            self,
            group="arm",
            ee_link="end_effector_link",
            base_frame="base_link",
            speed=0.2,
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

        # Localization action client
        self.localization_client = ActionClient(
            self,
            Relocalize,
            '/localization/relocalize'
        )

        # Localization service client
        self.get_locations_client = self.create_client(
            GetLocations,
            '/localization/get_locations'
        )

        self.start_grasp_client = self.create_client(
            StartGrasp,
            '/start_grasp'
        )

        self.state = "UNLOCALIZED"
        self.localization_in_progress = False
        self.localization_goal_handle = None
        self.object_transforms = {}
        self.feature_transforms = {}

        self.get_logger().info(
            "Phase 2 control initialized!"
        )
        self.create_timer(0.1, self.control_loop)

    #Localization
    def start_localization(self, target_ids):
        if self.state != "UNLOCALIZED":
            return

        self.state = "LOCALIZING"
        if not self.localization_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error("Localization action server unavailable")
            return
        self.localization_in_progress = True
        goal = Relocalize.Goal()
        goal.mode = Relocalize.Goal.SELECTIVE
        goal.target_ids = target_ids

        self.get_logger().info(
            f"Starting localization for: {target_ids}"
        )

        future = self.localization_client.send_goal_async(
            goal,
            feedback_callback=self.localization_feedback_callback
        )

        future.add_done_callback(
            self.localization_goal_response_callback
        ) 
    def localization_feedback_callback(self, feedback_msg):

        feedback = feedback_msg.feedback

        self.get_logger().info(
            f"Localizing {feedback.current_target}: "
            f"{feedback.targets_completed} completed, "
            f"{feedback.targets_remaining} remaining"
        )
    def localization_goal_response_callback(self, future):

        self.localization_goal_handle = future.result()

        if not self.localization_goal_handle.accepted:
            self.get_logger().error("Localization goal rejected")
            return

        self.get_logger().info("Localization goal accepted")

        result_future = self.localization_goal_handle.get_result_async()

        result_future.add_done_callback(
            self.localization_result_callback
        )
    def localization_result_callback(self, future):

        try:
            action_result = future.result()
            result = action_result.result

        except Exception as e:
            self.get_logger().error(
                f"Localization action failed: {e}"
            )
            self.state = "ERROR"
            self.localization_in_progress = False
            return

        self.get_logger().info(result.message)

        if not result.success:
            self.get_logger().error(
                f"Localization failed: {result.failed_ids}"
            )
            self.state = "ERROR"
            self.localization_in_progress = False
            return

        self.get_logger().info(
            f"Localized objects: {result.localized_ids}"
        )

        self.request_object_locations(result.localized_ids)
    def request_object_locations(self, object_ids):

        if not self.get_locations_client.wait_for_service(timeout_sec=5.0):
            self.get_logger().error("GetLocations service unavailable")
            return

        request = GetLocations.Request()

        request.object_ids = object_ids
        request.get_all = False

        future = self.get_locations_client.call_async(request)

        future.add_done_callback(
            self.object_locations_callback
        )
    def object_locations_callback(self, future):

        try:
            response = future.result()

        except Exception as e:
            self.get_logger().error(
                f"Failed to retrieve object locations: {e}"
            )
            return

        if not response.success:
            self.get_logger().error(
                "Failed to retrieve localized transforms"
            )
            return

        # Store object transforms
        self.object_transforms = {
            object_id: transform
            for object_id, transform in zip(
                response.object_ids,
                response.object_transforms
            )
        }

        # Store feature transforms
        self.feature_transforms = {
            frame_id: transform
            for frame_id, transform in zip(
                response.frame_ids,
                response.feature_transforms
            )
        }

        self.get_logger().info(
            f"Retrieved {len(self.object_transforms)} object transforms "
            f"and {len(self.feature_transforms)} feature transforms"
        )

        # Localization is complete; advance the state machine
        self.state = "PHASE1"
        self.localization_in_progress = False
    # Movement
    def grasp(self, screw_id):

        if not self.start_grasp_client.wait_for_service(timeout_sec=5.0):
            self.get_logger().error("StartGrasp service unavailable.")
            return False

        request = StartGrasp.Request()
        request.screw_id = screw_id

        self.get_logger().info(f"Requesting grasp of screw {screw_id}")

        future = self.start_grasp_client.call_async(request)

        response = wait_for(
            future,
            timeout=60.0
        )

        if response is None:
            self.get_logger().error("Grasp service timed out.")
            return False

        if not response.success:
            self.get_logger().error(
                f"Grasp failed for screw {screw_id}: {response.message}"
            )
            return False

        self.get_logger().info(
            f"Successfully grasped screw {screw_id}"
        )

        return True
    def move(self, target):

        # Case 1: Named waypoint from dictionary
        if isinstance(target, str):

            if target not in waypoints:
                self.get_logger().error(
                    f"Waypoint '{target}' does not exist!"
                )
                return False

            pose = waypoints[target]
            motion = waypoint_motion.get(target, "cartesian")
            label = target

        # Case 2: Direct pose (position + quaternion)
        else:

            if len(target) != 7:
                self.get_logger().error(
                    "Pose must contain 7 values: (x, y, z, qw, qx, qy, qz)"
                )
                return False

            pose = tuple(target)
            motion = "rrtconnect"
            label = "custom pose"

        self.get_logger().info(f"Moving to {label}: {pose}")
        self.get_logger().info(f"Using {motion} motion planning")

        success = self.arm_mover.move_to(
            pose,
            motion=motion,
            cartesian_fallback=False
        )

        if not success:
            self.get_logger().error(f"Failed to reach {label}")
            return False

        self.get_logger().info(f"Reached {label}!")
        return True
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
        goal.command.max_effort = 10.0

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

        self.get_logger().info(
            f"Gripper reached position {position}"
        )

        return True

    # Control loop
    def control_loop(self):
        """
        1. Localize all objects
        2. Phase 1 (To be developed still)
        3. Identify screws on the table and choose target
        4. Plan and execute grasp
        5. Deposit screw into appropriate bin
        6. Go to step 3 and repeat until table is empty
        7. Go to step 2 and repeat until bucket is empty
        """

        #1
        if self.state == "UNLOCALIZED":
            self.start_localization(['bin'])

        elif self.state == "PHASE1":
            #TODO implement phase 1
            if not self.move("phase2_home"):
                self.state = "ERROR"
                return
            self.state = "PHASE2"

        elif self.state == "PHASE2":
            #TODO grab workspace scan, classify and segment
            scan = [0, 0, 0]
            if len(scan) == 0:
                self.state = "PHASE1"

            #TODO plan grasp then execute it

            #placeholder grasp motion
            # if not self.gripper(0.65):
            #     self.state = "ERROR"
            #     return
            # if not self.move("phase2_floor"):
            #     self.state = "ERROR"
            #     return
            # if not self.gripper(0.75):
            #     self.state = "ERROR"
            #     return
            # if not self.move("phase2_home"):
            #     self.state = "ERROR"
            #     return
            
            if not self.grasp(0):
                self.state = "ERROR"
                return

            
            #Bin Deposit
            # tf = self.object_transforms['bin'].transform
            tf = self.feature_transforms['container_0_4'].transform

            p = tf.translation
            # q = tf.rotation
            waypoint = (
                p.x,
                p.y,
                p.z + 0.1675,
                1.0,
                0.0,
                0.0,
                0.0
            )
            if not self.move(waypoint):
                self.state = "ERROR"
                return
            if not self.gripper(0.55):
                return
            if not self.move("phase2_home"):
                self.state = "ERROR"
                return
            if not self.gripper(0.0):
                return


            self.state = "COMPLETE"


        # if not self.gripper(0.65):
        #     self.state = "ERROR"
        #     return
        # if not self.move("screwattempt"):
        #     self.state = "ERROR"
        #     return
        # # if not self.move("phase2_floor"):
        # #     self.state = "ERROR"
        # #     return
        # if not self.gripper(0.75):
        #     self.state = "ERROR"
        #     return


# MAIN
def main(args=None):


    rclpy.init(args=args)

    node = Phase_2()

    # Run ROS callbacks in a separate thread.
    # This allows asynchronous service/action futures
    # to complete while control_loop() is executing.

    executor = MultiThreadedExecutor(
        num_threads=2
    )

    executor.add_node(node)

    spin_thread = threading.Thread(
        target=executor.spin,
        daemon=True
    )

    spin_thread.start()

    try:
        while rclpy.ok():
            time.sleep(0.1)

    except KeyboardInterrupt:
        pass

    finally:
        executor.shutdown()
        spin_thread.join(timeout=2.0)
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":

    main()