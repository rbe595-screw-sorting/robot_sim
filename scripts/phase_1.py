#looked this one up: im confused: 
import os
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PACKAGE_DIR = os.path.dirname(SCRIPT_DIR)

sys.path.insert(0, PACKAGE_DIR)

print("Python package directory:", PACKAGE_DIR)
print("Helpers directory exists:", os.path.isdir(os.path.join(PACKAGE_DIR, "helpers")))
################

import rclpy
import threading
import cv2
import numpy as np
import time

from rclpy.node import Node
from rclpy.duration import Duration
from rclpy.action import ActionClient
from rclpy.executors import MultiThreadedExecutor
from rclpy.qos import qos_profile_sensor_data

import tf2_ros

from geometry_msgs.msg import PoseStamped
from trajectory_msgs.msg import JointTrajectoryPoint

from moveit_msgs.srv import GetPositionIK
from moveit_msgs.msg import MoveItErrorCodes

from sensor_msgs.msg import Image
from control_msgs.action import GripperCommand

from control_msgs.action import FollowJointTrajectory, GripperCommand

from cv_bridge import CvBridge

from helpers.robot_motion import ArmMover, wait_for
from helpers.apriltag_detector import AprilTagDetector

from rclpy.time import Time

from scipy.spatial.transform import Rotation


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
    "home": (0.4, 0.0, 0.3, 1.0, 0.0, 0.0, 0.0),

    #scooper located at 0.3 0.5 0 1.57 0 -1.57
    #bucket located at 0.2 -0.3 0 1.57 0 1.57
    #bucket at located at 0.05 -0.2 0 0 0 0
    #screw sorting located at -0.3, 0, 1.57, 0, 1.57

    "scooper_approach": (0.2, 0.0, 0.3, 1.0, 0.0, 0.0, 0.0), 
    
    "bucket_approach" : (0.05, -0.2, 0.3, 1.0, 0, 0, 0),

    "sorting_screw" : (0.3, -0.4, 0.5,  0.7071, 0.7071, 0.0, 0.0),

} 


# Each waypoint can use either:
#   "cartesian"  - Straight-line end-effector movement
#   "rrtconnect" - General collision-aware motion planning

waypoint_motion = {

    "workspace_home": "rrtconnect",

}

class Phase_1(Node):
    def __init__(self):
        super().__init__("Phase_1", 
                parameter_overrides=[
                    rclpy.parameter.Parameter(
                        'use_sim_time',
                        rclpy.Parameter.Type.BOOL,
                        True
                    )
                ])

        # MoveIt motion planning
        self.arm_mover = ArmMover(
            self,
            group=arm_group,
            ee_link=ee_link,
            base_frame=base_frame,
            speed=0.1,
            motion="rrtconnect",
            cartesian_fallback=False
        )

        #camera init: 
        #simulated values from ros2 topic echo
        self.camera_focal_length_x = 355.3948
        self.camera_focal_length_y = 355.3948
        self.camera_depth = 0.3

        self.image_center_x = None
        self.image_center_y = None

        self.camera_frame = 'color_optical_frame'

        #april tag init: 
        self.apriltag_detector = AprilTagDetector()
        self.apriltags_detected = []
        self.apriltag_centroid = None
        self.bucket_tag_centroid = None

        self.apriltag_scanning = True
        self.scooper_identified = False
        self.bucket_identified = False


        self.scooper_tag_id = 20
        self.bucket_tag_id = 16 

        #camera --> robot 
        self.tf_buffer = tf2_ros.Buffer()
        self.tf_listener = tf2_ros.TransformListener(self.tf_buffer, self)
        self.wrist_image_header = None

        # self.ik_client = self.create_client(GetPositionIK, ik_service)

        self.arm_client = ActionClient(self, FollowJointTrajectory, arm_action)

        self.gripper_client = ActionClient(self, GripperCommand, gripper_action)

        self.bridge = CvBridge()

        self.wrist_image = None; 
        self.wrist_image_subscriber = self.create_subscription(Image, '/wrist_mounted_camera/image', self.wrist_image_callback, qos_profile_sensor_data)
        self.image_pub = self.create_publisher(Image, '/perception/annotated_image', 10)


        #grasping: 
        #self.latest_grasp_pose = None
        self.wrist_image_subscriber = self.create_subscription(PoseStamped, '/grasp_pose', self.grasp_pose_callback, qos_profile_sensor_data)

        self.get_logger().info("phase 1 control initialized!")

    #Gage's image processing: 
    def wrist_image_callback(self,msg):
        """Convert the ROS image to an OpenCV image (BGR NumPy array)."""      
        self.wrist_image = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')
        self.wrist_image_header = msg.header
        cv2.imwrite("/tmp/live_bucket.png", self.wrist_image)
        self.apriltags_detected, output = self.apriltag_detector.detect(self.wrist_image)
        detected_ids = [tag["id"] for tag in self.apriltags_detected]
        # self.get_logger().info(f"Detected IDs: {detected_ids}")

        for tag in self.apriltags_detected: 
            tag_id = tag["id"]
            centroid = tag["centroid"]

            if tag_id == self.scooper_tag_id:
                self.apriltag_centroid = centroid   

                if not self.scooper_identified:
                    self.get_logger().info(f"scooper AT detected at {centroid}!")
                    self.scooper_identified = True

            if tag_id == self.bucket_tag_id:
                self.bucket_tag_centroid = centroid

            if not self.bucket_identified:
                self.get_logger().info(f"bucket AT detected at {centroid}!")
                self.bucket_identified = True
            
            # self.get_logger().info(f"scooper AT detected at {self.apriltag_centroid}")

        #publish img to view in rviz:
        annotated_msg = self.bridge.cv2_to_imgmsg(output,encoding='bgr8')
        annotated_msg.header = msg.header
        self.image_pub.publish(annotated_msg)
        
    def grasp_pose_callback(self, msg):
        self.latest_grasp_pose = msg

        self.get_logger().info(
            f"Received grasp pose in {msg.header.frame_id}: "
            f"x={msg.pose.position.x:.3f}, "
            f"y={msg.pose.position.y:.3f}, "
            f"z={msg.pose.position.z:.3f}"
        )

    def detect_edges(self, image, lower_threshold=50, upper_threshold=150, blur_kernel=(5, 5)):
        """
        Apply Gaussian blur and Canny edge detection.

        Args:
            image: Input BGR or grayscale image.
            lower_threshold: Canny lower threshold.
            upper_threshold: Canny upper threshold.
            blur_kernel: Gaussian blur kernel size.

        Returns:
            edges: Binary edge image (uint8).
        """

        # Convert to grayscale if necessary
        if len(image.shape) == 3:
            gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
        else:
            gray = image.copy()

        # Reduce noise
        blur = cv2.GaussianBlur(gray, blur_kernel, 0)

        # Canny edge detection
        edges = cv2.Canny(
            blur,
            lower_threshold,
            upper_threshold
        )

        return edges

    def find_oriented_bounding_boxes(
            self,
            edges,
            min_area=100,
            max_area=None
        ):
            """
            Extract contours and calculate oriented bounding boxes.
    
            Args:
                edges: Binary edge image from detect_edges().
                min_area: Minimum contour area to retain.
                max_area: Maximum contour area (None disables limit).
    
            Returns:
                List of dictionaries containing:
                    center: (cx, cy)
                    size: (width, height)
                    angle: Rotation in degrees
                    corners: Four bounding box corner coordinates
                    contour: Original contour
            """
    
            contours, hierarchy = cv2.findContours(
                edges,
                cv2.RETR_EXTERNAL,
                cv2.CHAIN_APPROX_SIMPLE
            )
    
            bounding_boxes = []
    
            for contour in contours:
    
                # Filter by contour area
                area = cv2.contourArea(contour)
    
                if area < min_area:
                    continue
    
                if max_area is not None and area > max_area:
                    continue
    
                # Calculate minimum-area rotated rectangle
                rect = cv2.minAreaRect(contour)
    
                (cx, cy), (width, height), angle = rect
    
                # Convert rectangle to four corner coordinates
                corners = cv2.boxPoints(rect)
                corners = np.intp(corners)
    
                bounding_boxes.append({
                    "center": (cx, cy),
                    "size": (width, height),
                    "angle": angle,
                    "corners": corners,
                    "contour": contour
                })
    
            return bounding_boxes

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

        if waypoint not in waypoints: 
            self.get_logger().error(f"Waypoint: {waypoint} does not exist! :(")
            return False

        self.get_logger().info(f"Moving to {waypoint}")

        pose = waypoints[waypoint]

        motion = waypoint_motion.get(waypoint, "cartesian")
        self.get_logger().info(f"Using {motion} motion planning")

        success = self.arm_mover.move_to(pose, motion=motion, cartesian_fallback=False)

        if not success: 
            self.get_logger().error(f"failed to reach waypoint: {waypoint} :(")
            return False

        self.get_logger().info(f"reached waypoint: {waypoint}!")
        return True

    #taken from previous lab
    def gripper(self, position):
        if not self.gripper_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error(f"gripper action server is unavailable :(")
            return False

        goal = GripperCommand.Goal()
        goal.command.position = position
        goal.command.max_effort = 10.0

        future = self.gripper_client.send_goal_async(goal)
        # rclpy.spin_until_future_complete(self, future)
        # goal_handle = future.result()

        goal_handle = wait_for(future, timeout = 10.0)


        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().error("gripper command rejected :( ")
            return False

        result_future = goal_handle.get_result_async()
        # rclpy.spin_until_future_complete(self, result_future)
        # result = result_future.result()

        result = wait_for(result_future, timeout = 30.0)

        if result is None: 
            self.get_logger().error("gripper timed out :(")
            return False

        if not result.result.reached_goal:
            self.get_logger().error("gripper timed out :(")
            return False

        self.get_logger().info(f"gripper reached position: {position}!")
        return True

    def convert_to_pose(self, centroid, waypoint):

        if self.wrist_image is None or self.wrist_image_header is None:
            self.get_logger().error("no camera image")
            return None

        x, y, z, qx, qy, qz, qw = waypoint

        image_height, image_width = self.wrist_image.shape[:2]

        self.image_center_x = image_width / 2.0
        self.image_center_y = image_height / 2.0

        tag_x, tag_y = centroid

        #get difference: 
        error_x = tag_x - self.image_center_x
        error_y = tag_y - self.image_center_y

        #converting: eq from class: 
        camera_x = (error_x * self.camera_depth) / self.camera_focal_length_x
        camera_y = (error_y * self.camera_depth) / self.camera_focal_length_y
        camera_z = self.camera_depth

        #worried tf is not updating properly: 
        if not self.tf_buffer.can_transform(
            base_frame,
            self.camera_frame,
            Time(),
            timeout=Duration(seconds=3.0)
        ):
            self.get_logger().error("Transform unavailable!")
            return None

        #transform to robot bsae link: 
        try:
            transform = self.tf_buffer.lookup_transform(base_frame, self.camera_frame, Time.from_msg(self.wrist_image_header.stamp))
        except tf2_ros.TransformException as e: 
            self.get_logger().error(f"could not transofm coords: {e}")
            return None

        trans = transform.transform.translation
        rot = transform.transform.rotation

        rotation = Rotation.from_quat([rot.x, rot.y, rot.z, rot.w]).as_matrix()

        translation = np.array([trans.x, trans.y, trans.z])

        #conversion here: 
        camera_point = np.array([camera_x, camera_y, camera_z])
        base_point = rotation @ camera_point + translation 

        pose_x = base_point[0]
        pose_y = base_point[1]
        pose_z = base_point[2]


        # self.get_logger().info(f"apriltag in base frame: x = {pose_x}, y = {pose_y}, z = {pose_z}")

        #return as quat: (keeping z as what it was originally at)


        return pose_x, pose_y, z, qx, qy, qz, qw
    
    def grasp_scooper(self, location):
        #take teh curr location & add an offset to grab the scooper! 

        x, y, z, qx, qy, qz, qw = location

        translation_x = -0.065
        translation_y = 0

        grasping_x = x + translation_x
        grasping_y = y + translation_y 

        return grasping_x, grasping_y, z, qx, qy, qz, qw

    def hold_scooper(self, location):
#       take the curr location & add an offset to grab the scooper! 

        x, y, z, qx, qy, qz, qw = location

        translation_z = -0.06

        grasping_z = z + translation_z

        return x, y, grasping_z, qx, qy, qz, qw

    def enter_bucket(self, location):
        #take teh curr location & add an offset to get into the bucket! 
        x, y, z, qx, qy, qz, qw = location

        yaw = 1.57
        
        # translation_x = 0.15 * np.cos(yaw)
        # translation_y = 0.2 * np.sin(yaw)

        # temp:
        translation_x = (0.1 * np.cos(yaw)) + (0.1 * np.sin(yaw))
        translation_y = - (0.1 * np.sin(yaw)) + (0.1 * np.cos(yaw))
        # rotation_x = 

        entering_x = x + translation_x 
        entering_y = y + translation_y 

        # rotation_x = qx - rotation_x

        return entering_x, entering_y, z, qx, qy, qz, qw

    def scoop(self, location):

        #current location: 
        entering_x, entering_y, entering_z, qx, qy, qz, qw = location

        #move downwards into the bucket: 
        drop_location = 0.15

        scoop_z = entering_z + drop_location

        entering_pose = entering_x, entering_y, scoop_z, qx, qy, qz, qw

        self.arm_mover.move_to(entering_pose, motion = "cartesian", cartesian_fallback=False)

        #move translationally through the bucket: 
        x_translational = 0.15

        scoop_x = entering_x + x_translational

        scoop_pose = scoop_x, entering_y, scoop_z, qx, qy, qz, qw

        self.arm_mover.move_to(scoop_pose, motion = "cartesian", cartesian_fallback=False)

        #move up 
        move_up_pose = scoop_x, entering_y, entering_z, qx, qy, qz, qw

        self.arm_mover.move_to(move_up_pose, motion = "cartesian", cartesian_fallback=False)
        
        #go to sorting location



        # return , grasping_y, z, qx, qy, qz, qw

    #modelled off old hw5? 
    def control_loop(self):

        #start position 
        self.gripper(0.0)

        self.apriltag_scanning = True

        # if not self.arm_mover.move_to(waypoints["home"]):
        #     self.get_logger().error(f"could not home")
        #     return

        self.get_logger().info("phase 1 homing complete!")

        # #go to estimate location of the scooper: 
        scooper_approach = waypoints["scooper_approach"]

        # if not self.arm_mover.move_to(scooper_approach, motion="rrtconnect"):
        #     self.get_logger().error(f"could not naviate to scooper approach")
        #     return

        self.get_logger().info(f"made it to scooper")

        #5 s to find the AT
        timeout = time.time() + 5.0

        #if found, just break out dont have to wait the 5s
        while time.time() < timeout: 
            if self.apriltag_centroid is not None: 
                break
            
            time.sleep(0.1)

        # if self.apriltag_centroid is None: 
        #     self.get_logger().warning("apriltag not detected here!")
        #     #write something to do here if april tag not detected? 
        #     #maybe do a little jiggle around like in hw 7 
        #     return

        #CENTER 
        #convert april tag centroid to robot pose:
        # scooper_center_pose = self.convert_to_pose(self.apriltag_centroid, scooper_approach)
        # self.get_logger().info(f"Scooper approach pose: {scooper_approach}")

        # if scooper_center_pose is None: 
        #     self.get_logger().error(f"could not calculate scooper center pose")
        #     return

        # self.get_logger().info(f"Scooper center pose: {scooper_center_pose}")

        #move to centroid along x,y, keep z up to center along the scooper
        # if not self.arm_mover.move_to(scooper_center_pose, motion = "cartesian", cartesian_fallback=False):
        #     self.get_logger().info(f"could not make it to scoop center location :/ ")
        #     return 

        # self.get_logger().info(f"made it to scooper center pose!")


        #GRASP
        #scooper grasp pose: 
        # scooper_grasp_pose = self.grasp_scooper(scooper_center_pose)
        self.get_logger().info(f"Scooper grasp pose!")

        # if scooper_grasp_pose is None: 
        #     self.get_logger().error(f"could not calculate scooper grasp pose")
        #     return

        # self.get_logger().info(f"Scooper grasp pose: {scooper_grasp_pose}")

        #move to centroid along x,y, keep z up to hover over the handle
        # if not self.arm_mover.move_to(scooper_grasp_pose, motion = "cartesian", cartesian_fallback=False):
        #     self.get_logger().info(f"could not make it to scoop grasp location :/ ")
        #     return 

        self.get_logger().info(f"made it to scooper grasp pose!")


        #HOLD
        #scooper hold pose: 
        # scooper_hold_pose = self.hold_scooper(scooper_grasp_pose)

        # if scooper_hold_pose is None: 
        #     self.get_logger().error(f"could not calculate scooper hold pose")
        #     return

        # self.get_logger().info(f"Scooper hold pose: {scooper_hold_pose}")

        #move to centroid along z  to grasp the handle
        # if not self.arm_mover.move_to(scooper_hold_pose, motion = "cartesian", cartesian_fallback=False):
        #     self.get_logger().info(f"could not make it to scoop hold location :/ ")
        #     return 

        self.get_logger().info(f"made it to scoop hold pose!")


        #close that scoop! 
        # self.gripper(1.0)

        #move back up 
        # if not self.arm_mover.move_to(scooper_grasp_pose, motion = "cartesian", cartesian_fallback=False):
        #     self.get_logger().info(f"could not make it to scoop grasp location :/ ")
        #     return 

        self.get_logger().info(f"made it back to scoop hold pose!")

        #move to above the bucket
        if not self.arm_mover.move_to(waypoints["bucket_approach"]):
            self.get_logger().error(f"could not make it to bucket approach")
            return
        
        self.get_logger().info(f"made it to bucket approach location")

        #detecting tag: 
        while time.time() < timeout: 
            if self.apriltag_centroid is not None: 
                break
            
            time.sleep(0.1)

        if self.bucket_tag_centroid is None: 
            self.get_logger().warning("apriltag not detected here!")
        #   #write something to do here if april tag not detected? 
            #maybe do a little jiggle around like in hw 7 
            return

        self.get_logger().info(f"found the bucket at {self.bucket_tag_centroid}!")

        bucket_approach = waypoints["bucket_approach"]

        #CENTER 
        #convert april tag centroid to robot pose:
        bucket_center_pose = self.convert_to_pose(self.bucket_tag_centroid, bucket_approach)
        self.get_logger().info(f"bucket approach pose: {bucket_center_pose}")

        if bucket_center_pose is None: 
            self.get_logger().error(f"could not calculate bucket center pose")
            return

        # self.get_logger().info(f"Scooper center pose: {bucket_center_pose}")

        bucket_center_pose = self.enter_bucket(bucket_center_pose)

        #move along x,y, keep z up to center along the scooper
        if not self.arm_mover.move_to(bucket_center_pose, motion = "cartesian", cartesian_fallback=False):
            self.get_logger().info(f"could not make it to bucket enter location :/ ")
            return 

        self.get_logger().info(f"made it to bucket center pose!")

        #call function to scoop the bucket: 

        if not self.scoop(bucket_center_pose):
            self.get_logger().info(f"not able to scoop!!")
            return

        self.get_logger().info(f"scooped the bucket!!")







            


def main(args=None):
    rclpy.init(args=args)

    node = Phase_1()

    #from phase2 sim: 

    executor = MultiThreadedExecutor(num_threads=2)

    executor.add_node(node)

    spin_thread = threading.Thread(target=executor.spin, daemon=True)

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