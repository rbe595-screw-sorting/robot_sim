import threading

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.executors import MultiThreadedExecutor

from control_msgs.action import GripperCommand

from helpers.robot_motion import ArmMover, wait_for


# ============================================================
# ROBOT CONFIGURATION
# ============================================================

arm_group = "arm"
base_frame = "base_link"
ee_link = "end_effector_link"

gripper_action = "/gen3_lite_2f_gripper_controller/gripper_cmd"


# ============================================================
# WAYPOINTS
# Format: x, y, z, qx, qy, qz, qw
# Position in meters, orientation in quaternion.
# ============================================================

waypoints = {
    # "home" : (0.2, -0.3, 0.5, 1.0, 1.0, 1.0, 1.0),
    # "scooper_approach" : (0.3, 0.4, 0.3, 0.7071, 0.7071, 0.0, 0.0), 

    "workspace_home" : (0.0, 0.4, 0.3, 1.0, 0.0, 0.0, 0.0)
}


# ============================================================
# MOTION PLANNING CONFIGURATION
# ============================================================

# Each waypoint can use either:
#   "cartesian"  - Straight-line end-effector movement
#   "rrtconnect" - General collision-aware motion planning

waypoint_motion = {

    "workspace_home": "rrtconnect",
    # "home": "rrtconnect",
    # "scooper_approach": "rrtconnect",
}


# ============================================================
# PHASE 1 NODE
# ============================================================

class Phase_2(Node):

    def __init__(self):

        super().__init__("Phase_2", 
            parameter_overrides=[
            rclpy.parameter.Parameter(
                'use_sim_time',
                rclpy.Parameter.Type.BOOL,
                True
            )
        ])

        # ----------------------------------------------------
        # MoveIt motion planning
        # ----------------------------------------------------

        self.arm_mover = ArmMover(
            self,
            group=arm_group,
            ee_link=ee_link,
            base_frame=base_frame,
            speed=0.1,
            motion="cartesian",
            cartesian_fallback=False
        )

        # ----------------------------------------------------
        # Gripper action client
        # ----------------------------------------------------

        self.gripper_client = ActionClient(
            self,
            GripperCommand,
            gripper_action
        )

        self.get_logger().info(
            "Phase 1 control initialized!"
        )




    # ========================================================
    # ARM MOVEMENT
    # ========================================================

    def move(self, waypoint):

        self.get_logger().info(
            f"Moving to {waypoint}"
        )

        if waypoint not in waypoints:

            self.get_logger().error(
                f"Waypoint {waypoint} does not exist!"
            )

            return False

        pose = waypoints[waypoint]

        motion = waypoint_motion.get(
            waypoint,
            "cartesian"
        )

        self.get_logger().info(
            f"Using {motion} motion planning"
        )

        success = self.arm_mover.move_to(
            pose,
            motion=motion,
            cartesian_fallback=False
        )

        if not success:

            self.get_logger().error(
                f"Failed to reach {waypoint}"
            )

            return False

        self.get_logger().info(
            f"Reached {waypoint}!"
        )

        return True


    # ========================================================
    # GRIPPER CONTROL
    # ========================================================

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


    # ========================================================
    # HIGH LEVEL CONTROL SEQUENCE
    # ========================================================

    def control_loop(self):

        # ----------------------------------------------------
        # Initialize gripper
        # ----------------------------------------------------

        if not self.gripper(0.0):
            return

        # ----------------------------------------------------
        # 1. Approach and grasp scooper
        # ----------------------------------------------------

        initial_sequence = [

            "workspace_home",
            # "home",
            # "scooper_approach",

        ]

        for waypoint in initial_sequence:

            if not self.move(waypoint):
                return

        # Close gripper

        if not self.gripper(0.5):
            return


# ============================================================
# MAIN
# ============================================================

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

        node.control_loop()

    except KeyboardInterrupt:

        pass

    finally:

        executor.shutdown()

        spin_thread.join(timeout=2.0)

        node.destroy_node()

        rclpy.shutdown()


if __name__ == "__main__":

    main()