import rclpy
from rclpy.node import Node
from rclpy.duration import Duration

from geometry_msgs.msg import PoseStamped
from trajectory_msgs.msg import JointTrajectoryPoint

from moveit_msgs.srv import GetPositionIK
from moveit_msgs.msg import MoveItErrorCodes

from control_msgs.action import FollowJointTrajectory, GripperCommand
from rclpy.action import ActionClient

arm_group = "arm"
base_frame = "base_link"
ee_link = "end_effector_link"

joints = ["joint_1", "joint_2", "joint_3", "joint_4", "joint_5", "joint_6"]

arm_action = "/joint_trajectory_controller/follow_joint_trajectory"
gripper_action = "/gen3_lite_2f_gripper_controller/gripper_cmd"
ik_service = "/compute_ik"

#quart waypoint format: x, y, z, qx, qy, qz, qw

#home
#approach s, grasp s, lift s, position s
#pick b, position b, drop b, leave 
#home

waypoints = {
    "home" : (0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),

    "scooper_approach" : (0.2, 0.3, 0.5, 0.0, 0.0, 0.0, 1.0), 
    "scooper_grasp" : (0.2, 0.25, 0.5, 0.0, 0.0, 0.0, 1.0),
    "scooper_lift" : (0.2, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0),

    "bucket_approach" : (-0.3, -0.4, 0.0, 0.0, 0.0, 0.0, 1.0),
    "pick_bucket" : (-0.2, -0.3, 0.0, 0.0, 0.0, 0.0, 1.0),
    "center_screws" : (0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0),

    #rotation test here! 
    "drop_bucket" : (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0),

    "leave_scoop" : (0.2, 0.20, 0.5, 0.0, 0.0, 0.0, 1.0),
} 

class Phase_1(Node):
    def __init__(self):
        super().__init__("Phase_1")

        self.ik_client = self.create_client(GetPositionIK, ik_service)

        self.arm_client = ActionClient(self, FollowJointTrajectory, arm_action)

        self.gripper_client = ActionClient(self, GripperCommand, gripper_action)

        self.get_logger().info("Phase 1 control initialized!")


    #getting the pose:
    def ik(self, waypoint):
        request = GetPositionIK.Request()

        request.ik_request.group_name = arm_group
        request.ik_request.ik_link_name = ee_link
        request.ik_request.pose_stamped.header.frame_id = base_frame
        request.ik_request.pose_stamped.header.stamp = (self.get_clock().now().to_msg())

        x, y, z, qx, qy, qz, qw = waypoint

        pose = request.ik_request.pose_stamped.pose

        pose.position.x = x
        pose.position.y = y
        pose.position.z = z

        pose.orientation.x = qx
        pose.orientation.y = qy
        pose.orientation.z = qz
        pose.orientation.w = qw

        #need to look into these more:
        request.ik_request.avoid_collisions = True
        request.ik_request.robot_state.is_diff = True
        request.ik_request.timeout.sec = 2

        #if IK timesout:
        if not self.ik_client.wait_for_service(timeout_sec = 5.0):
            self.get_logger().error("MoveIt IK service unavailable :( ")
            return None
        
        future = self.ik_client.call_async(request)
        rclpy.spin_until_future_complete(self, future)

        response = future.result()

        #if IK doesnt get a valid response:
        if response is None: 
            self.get_logger().error("IK request failed :(")
            return None

        #if it fails anyway: 
        if response.error_code.val != MoveItErrorCodes.SUCCESS:
            self.get_logger().error(f"IK failure: {response.error_code.val}")
            return None

        #if it succeeds:
        joint_state = response.solution.joint_state
        joint_positions = dict(zip(joint_state.name, joint_state.position))

        return [
            joint_positions[joint]
            for joint in joints
        ]


    #executing the poses: 
    def move(self, waypoint):
        self.get_logger().info(f"Moving to {waypoint}")

        waypoint = waypoints[waypoint]

        joint_positions = self.ik(waypoint)

        #no movement
        if joint_positions is None:
            return False

        #timeout 
        if not self.arm_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error("arm trajectory is unavailable :(")
            return False

        #setting point as a goal:
        goal = FollowJointTrajectory.Goal()
        goal.trajectory.joint_names = joints


        point = JointTrajectoryPoint()
        point.positions = joint_positions
        point.time_from_start.sec = 4

        goal.trajectory.points.append(point)

        future = self.arm_client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, future)

        goal_handle = future.result()


        #failed to use ik to go to new position
        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().error("failed to make it to the arm goal")
            return False

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)
        result = result_future.result()

        if result is None or result.result.error_code != 0:
            self.get_logger().error("Failed to complete arm movement.")
            return False

        #if successful: 
        self.get_logger().info(f"reached {waypoint} !")
        return True


    #taken from previous lab
    def gripper(self, position):
        if not self.gripper_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error(
                "Gripper action server is unavailable."
            )
            return False

        goal = GripperCommand.Goal()
        goal.command.position = position
        goal.command.max_effort = 10.0

        future = self.gripper_client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, future)

        goal_handle = future.result()

        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().error("Gripper command rejected.")
            return False

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)

        result = result_future.result()

        return result is not None and result.result.reached_goal

    #modelled off old hw5? 
    def control_loop(self):

        #make sure teh gripper is to start: 
        self.gripper(0.0)

        initial_sequence = [
            "scooper_approach",
            "scooper_grasp"
        ]


        for waypoint in initial_sequence:
            if not self.move(waypoint):
                return

        #closing to grasp the scoop: 
        if not self.gripper(0.5):
            return

        middle_sequence = [
            "scooper_approach", 
            "scooper_grasp" ,
            "scooper_lift" ,
            "bucket_appraoch", 
            "pick_bucket" ,
            "center_screws", 
            "drop_bucket" 
            
        ]

        for waypoint in middle_sequence:
            if not self.move(waypoint):
                return

        closing_sequence = [
            "leave_bucket",
            "home"
        ]

        #open to leave the scoop: 
        if not self.gripper(0.5):
            return

        for waypoint in middle_sequence:
            if not self.move(waypoint):
                return

        self.get_logger().info("Phase 1 Complete!")
            


def main(args=None):
    rclpy.init(args=args)

    node = Phase_1()

    try:
        node.control_loop()
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()