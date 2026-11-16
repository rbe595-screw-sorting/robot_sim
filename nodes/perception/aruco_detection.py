#!/usr/bin/env python3

#!/usr/bin/env python3

import rclpy
import cv2
import numpy as np
import tf2_ros

from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data

from cv_bridge import CvBridge
from sensor_msgs.msg import Image, CameraInfo
from geometry_msgs.msg import PoseStamped

from tf2_ros import TransformException
from tf2_geometry_msgs import do_transform_pose_stamped

from scipy.spatial.transform import Rotation






class ArucoDetection(Node):

    def __init__(self):
        super().__init__('aruco_detection')

        self.last_locations = {"Bin":None, "Bucket":None, "Scoop":None}

        self.tag_params = {
            0: (0.05, "Bin"),
            1: (0.05, "Bucket"),
            2: (0.05, "Scoop"),
        }

        self.aruco_dict = cv2.aruco.getPredefinedDictionary(
            cv2.aruco.DICT_APRILTAG_36h11
        )

        self.declare_parameter('camera_topic', '/camera/camera/color/image_raw')
        self.declare_parameter('camera_intrinsics_topic', '/camera/camera/color/camera_info')
        self.camera_topic = self.get_parameter('camera_topic').value
        self.camera_intrinsics_topic = self.get_parameter('camera_intrinsics_topic').value
        
        self.camera_matrix = None
        self.distortion_coefficients = None

        self.camera_info_sub = self.create_subscription(
            CameraInfo,
            self.camera_intrinsics_topic,
            self.camera_info_callback,
            10
        )

        self.detector_params = cv2.aruco.DetectorParameters_create()


        self.tf_buffer = tf2_ros.Buffer(
            cache_time=rclpy.duration.Duration(seconds=10.0)
        )
        self.tf_listener = tf2_ros.TransformListener(
            self.tf_buffer,
            self
        )

        self.world_frame = "base_link"


        self.declare_parameter('bucket_location_topic', 'perception/bucket_location')
        self.declare_parameter('scoop_location_topic', 'perception/scoop_location')
        self.declare_parameter('bin_location_topic', 'perception/bin_location')
        self.bucket_location_topic = self.get_parameter('bucket_location_topic').value
        self.scoop_location_topic = self.get_parameter('scoop_location_topic').value
        self.bin_location_topic = self.get_parameter('bin_location_topic').value


        # self.declare_parameter('debug', False)
        # self.debug = self.get_parameter('debug').value

        self.bridge = CvBridge()
        self.wrist_image = None
        self.wrist_image_msg = None
        self.wrist_image_subscriber = self.create_subscription(
            Image,
            self.camera_topic,
            self.wrist_image_callback,
            qos_profile_sensor_data,
        )

        self.bucket_pub = self.create_publisher(
            PoseStamped,
            self.bucket_location_topic,
            10
        )
        self.scoop_pub = self.create_publisher(
            PoseStamped,
            self.scoop_location_topic,
            10
        )
        self.bin_pub = self.create_publisher(
            PoseStamped,
            self.bin_location_topic,
            10
        )

        self.location_publishers = {
            "Bin": self.bin_pub,
            "Bucket": self.bucket_pub,
            "Scoop": self.scoop_pub
        }

        self.control_timer = self.create_timer(
            0.1,  # 10 Hz
            self.control_loop
        )

        
        self.get_logger().info('ArucoDetection node initialized!')

    def wrist_image_callback(self, msg):
        self.wrist_image_msg = msg
        self.wrist_image = self.bridge.imgmsg_to_cv2(
            msg,
            desired_encoding='bgr8'
        )
    def camera_info_callback(self, msg):
        """Populate camera calibration parameters from CameraInfo."""

        self.camera_matrix = np.array(
            msg.k,
            dtype=np.float64
        ).reshape(3, 3)

        self.distortion_coefficients = np.array(
            msg.d,
            dtype=np.float64
        )

        # self.get_logger().info(
        #     f"Received camera calibration:\n{self.camera_matrix}\n"
        #     f"Distortion: {self.distortion_coefficients}"
        # )

        # Only need the calibration once
        self.destroy_subscription(self.camera_info_sub)

    def control_loop(self):
        if self.camera_matrix is None or self.distortion_coefficients is None:
            self.get_logger().warn(
                "Waiting for camera calibration...",
                throttle_duration_sec=5.0
            )
            return
        image = self.wrist_image
        image_msg = self.wrist_image_msg

        if image is None or image_msg is None:
            return

        corners, ids, rejected = cv2.aruco.detectMarkers(
            image,
            self.aruco_dict,
            parameters=self.detector_params
        )

        if ids is not None:

            for marker_corners, marker_id in zip(corners, ids.flatten()):

                if marker_id not in self.tag_params:
                    continue

                marker_length, object_type = self.tag_params[marker_id]

                object_points = np.array([
                    [-marker_length/2,  marker_length/2, 0],
                    [ marker_length/2,  marker_length/2, 0],
                    [ marker_length/2, -marker_length/2, 0],
                    [-marker_length/2, -marker_length/2, 0]
                ], dtype=np.float32)

                image_points = marker_corners.reshape(4, 2)

                success, rvec, tvec = cv2.solvePnP(
                    object_points,
                    image_points,
                    self.camera_matrix,
                    self.distortion_coefficients
                )

                if not success:
                    continue

                rotation = Rotation.from_rotvec(rvec.flatten())
                quaternion = rotation.as_quat()

                pose = PoseStamped()

                pose.header.stamp = image_msg.header.stamp
                pose.header.frame_id = image_msg.header.frame_id

                pose.pose.position.x = float(tvec[0][0])
                pose.pose.position.y = float(tvec[1][0])
                pose.pose.position.z = float(tvec[2][0])

                pose.pose.orientation.x = float(quaternion[0])
                pose.pose.orientation.y = float(quaternion[1])
                pose.pose.orientation.z = float(quaternion[2])
                pose.pose.orientation.w = float(quaternion[3])

                try:
                    transform = self.tf_buffer.lookup_transform(
                        self.world_frame,
                        image_msg.header.frame_id,
                        rclpy.time.Time.from_msg(image_msg.header.stamp),
                        timeout=rclpy.duration.Duration(seconds=0.1)
                    )

                    world_pose = do_transform_pose_stamped(
                        pose,
                        transform
                    )

                except TransformException as ex:
                    self.get_logger().warning(
                        f"Could not transform AprilTag {marker_id}: {ex}"
                    )
                    continue

                self.last_locations[object_type] = world_pose

        for object_type, pose in self.last_locations.items():
            if pose is not None:
                self.location_publishers[object_type].publish(pose)


def main(args=None):
    rclpy.init(args=args)

    node = ArucoDetection()

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()