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

        # MoveIt motion planning
        self.arm_mover = ArmMover(
            self,
            group=arm_group,
            ee_link=ee_link,
            base_frame=base_frame,
            speed=0.1,
            motion="cartesian",
            cartesian_fallback=False
        )

        # Gripper action client
        self.gripper_client = ActionClient(
            self,
            GripperCommand,
            gripper_action
        )


        self.bridge = CvBridge()
        self.wrist_image = None  # Latest OpenCV image; None until one arrives.
        self.wrist_image_subscriber = self.create_subscription(
            Image,
            '/wrist_mounted_camera/image',
            self.wrist_image_callback,
            qos_profile_sensor_data,
        )
        self.image_pub = self.create_publisher(
            Image,
            '/perception/annotated_image',
            10
        )

        self.get_logger().info(
            "Phase 1 control initialized!"
        )


    # IMAGE PROCESSING
    def wrist_image_callback(self, msg):
        """Convert the ROS image to an OpenCV image (BGR NumPy array)."""
        self.wrist_image = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')

    def detect_edges(
        self,
        image,
        lower_threshold=50,
        upper_threshold=150,
        blur_kernel=(5, 5)
    ):
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

    # ARM MOVEMENT
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


    # GRIPPER CONTROL
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


    # HIGH LEVEL CONTROL SEQUENCE
    def control_loop(self):

        # Initialize gripper
        if not self.gripper(0.0):
            return

        initial_sequence = [
            "workspace_home",
        ]

        for waypoint in initial_sequence:
            if not self.move(waypoint):
                return


        image = self.wrist_image
        edges = self.detect_edges(image)
        boxes = self.find_oriented_bounding_boxes(edges)
        output = image.copy()

        for box in boxes:

            corners = box["corners"]

            cv2.drawContours(
                output,
                [corners],
                -1,
                (0, 255, 0),
                2
            )

            cx, cy = map(int, box["center"])

            cv2.circle(
                output,
                (cx, cy),
                4,
                (0, 0, 255),
                -1
            )

        # Convert OpenCV image to ROS Image message
        ros_image = self.bridge.cv2_to_imgmsg(
            output,
            encoding="bgr8"
        )

        # Publish annotated image for debugging purposes
        # while True:
        #     self.image_pub.publish(ros_image)
        #     time.sleep(0.05)

        # Close gripper
        if not self.gripper(0.5):
            return


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