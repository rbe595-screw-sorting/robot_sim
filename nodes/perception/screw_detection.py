#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
import cv2
import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.executors import MultiThreadedExecutor
from cv_bridge import CvBridge
from sensor_msgs.msg import Image
from rclpy.qos import qos_profile_sensor_data
import numpy as np
from helpers.robot_motion import ArmMover, wait_for
import time
from screw_sorting_interfaces.msg import Screw


class ScrewDetection(Node):
    def __init__(self):
        super().__init__('localization')

        self.get_logger().info('NODE_NAME initialized!')

        # TODO: Initialize publishers, subscribers, services, etc.
    # def __init__(self):
    #     super().__init__('screw_detection')
    #     self.last_screw = None

    #     self.declare_parameter('camera_topic', '/camera/camera/color/image_raw')
    #     self.declare_parameter('screw_location_topic', '/perception/screw_location')

    #     self.declare_parameter('debug', False)
    #     self.debug = self.get_parameter('debug').value
    #     if self.debug:
    #         self.declare_parameter('debug_screw_box_topic', '/debug/annotated_screw_image')
    #         self.annotated_screw_image = self.get_parameter('debug_screw_box_topic').value

    #         self.image_pub = self.create_publisher(
    #         Image,
    #         self.annotated_screw_image,
    #         10
    #     )

    #     self.camera_topic = self.get_parameter('camera_topic').value
    #     self.screw_location_topic = self.get_parameter('screw_location_topic').value

    #     self.bridge = CvBridge()
    #     self.wrist_image = None  # Latest OpenCV image; None until one arrives.
    #     self.wrist_image_subscriber = self.create_subscription(
    #         Image,
    #         self.camera_topic,
    #         self.wrist_image_callback,
    #         qos_profile_sensor_data,
    #     )

    #     self.screw_pub = self.create_publisher(
    #         Screw,
    #         self.screw_location_topic,
    #         10
    #     )

    #     self.control_timer = self.create_timer(
    #         0.1,  # 10 Hz
    #         self.control_loop
    #     )


    #     self.get_logger().info('ScrewDetection initialized!')



    # def wrist_image_callback(self, msg):
    #         """Convert the ROS image to an OpenCV image (BGR NumPy array)."""
    #         self.wrist_image = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')
    
    # def detect_edges(
    #     self,
    #     image,
    #     lower_threshold=50,
    #     upper_threshold=150,
    #     blur_kernel=(5, 5)
    # ):
    #     """
    #     Apply Gaussian blur and Canny edge detection.

    #     Args:
    #         image: Input BGR or grayscale image.
    #         lower_threshold: Canny lower threshold.
    #         upper_threshold: Canny upper threshold.
    #         blur_kernel: Gaussian blur kernel size.

    #     Returns:
    #         edges: Binary edge image (uint8).
    #     """

    #     # Convert to grayscale if necessary
    #     if len(image.shape) == 3:
    #         gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    #     else:
    #         gray = image.copy()

    #     # Reduce noise
    #     blur = cv2.GaussianBlur(gray, blur_kernel, 0)

    #     # Canny edge detection
    #     edges = cv2.Canny(
    #         blur,
    #         lower_threshold,
    #         upper_threshold
    #     )

    #     return edges
    
    # def find_oriented_bounding_boxes(
    #     self,
    #     edges,
    #     min_area=100,
    #     max_area=None
    # ):
    #     """
    #     Extract contours and calculate oriented bounding boxes.

    #     Args:
    #         edges: Binary edge image from detect_edges().
    #         min_area: Minimum contour area to retain.
    #         max_area: Maximum contour area (None disables limit).

    #     Returns:
    #         List of dictionaries containing:
    #             center: (cx, cy)
    #             size: (width, height)
    #             angle: Rotation in degrees
    #             corners: Four bounding box corner coordinates
    #             contour: Original contour
    #     """

    #     contours, hierarchy = cv2.findContours(
    #         edges,
    #         cv2.RETR_EXTERNAL,
    #         cv2.CHAIN_APPROX_SIMPLE
    #     )

    #     bounding_boxes = []

    #     for contour in contours:

    #         # Filter by contour area
    #         area = cv2.contourArea(contour)

    #         if area < min_area:
    #             continue

    #         if max_area is not None and area > max_area:
    #             continue

    #         # Calculate minimum-area rotated rectangle
    #         rect = cv2.minAreaRect(contour)

    #         (cx, cy), (width, height), angle = rect

    #         # Convert rectangle to four corner coordinates
    #         corners = cv2.boxPoints(rect)
    #         corners = np.intp(corners)

    #         bounding_boxes.append({
    #             "center": (cx, cy),
    #             "size": (width, height),
    #             "angle": angle,
    #             "corners": corners,
    #             "contour": contour
    #         })
    #     return bounding_boxes 

    # def control_loop(self):
    #     image = self.wrist_image

    #     # Wait until the first camera frame arrives
    #     if image is None:
    #         return

    #     edges = self.detect_edges(image)
    #     boxes = self.find_oriented_bounding_boxes(edges)

    #     if boxes:
    #         box = boxes[0]

    #         screw_msg = Screw()
    #         screw_msg.screw_type = "unknown"

    #         screw_msg.centroid_position.x = float(box["center"][0])
    #         screw_msg.centroid_position.y = float(box["center"][1])
    #         screw_msg.centroid_position.z = 0.0

    #         screw_msg.length = float(max(box["size"]))
    #         screw_msg.width = float(min(box["size"]))
    #         screw_msg.angle = float(np.deg2rad(box["angle"]))            

    #         self.last_screw = screw_msg

    #     # Publish the most recently detected screw
    #     if self.last_screw is not None:
    #         self.screw_pub.publish(self.last_screw)

    #     if self.debug:
    #         output = image.copy()

    #         for box in boxes:
    #             corners = box["corners"]

    #             cv2.drawContours(
    #                 output,
    #                 [corners],
    #                 -1,
    #                 (0, 255, 0),
    #                 2
    #             )

    #             cx, cy = map(int, box["center"])

    #             cv2.circle(
    #                 output,
    #                 (cx, cy),
    #                 4,
    #                 (0, 0, 255),
    #                 -1
    #             )

    #         ros_image = self.bridge.cv2_to_imgmsg(
    #             output,
    #             encoding="bgr8"
    #         )

    #         self.image_pub.publish(ros_image)

def main(args=None):
    rclpy.init(args=args)

    node = ScrewDetection()

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()