#!/usr/bin/env python3
import numpy as np
import rclpy
from geometry_msgs.msg import Pose, PoseStamped
from common_interfaces_merlab.srv import SendPose 

from pcl_python_merlab import PointCloud
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from rclpy.time import Time
from sensor_msgs.msg import PointCloud2
from sensor_msgs_py import point_cloud2
from std_msgs.msg import Header
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy


def cloud_from_msg(msg):
    xyz = point_cloud2.read_points_numpy(
        msg, field_names=('x', 'y', 'z'), skip_nans=False,
    ).reshape(-1, 3)
    return PointCloud(xyz[np.isfinite(xyz).all(axis=1)].astype(np.float32))

def rotation_to_quaternion(R):
    """3x3 rotation matrix to (x, y, z, w)."""
    t = np.trace(R)
    if t > 0.0:
        s = 2.0 * np.sqrt(t + 1.0)
        w = 0.25 * s
        x = (R[2, 1] - R[1, 2]) / s
        y = (R[0, 2] - R[2, 0]) / s
        z = (R[1, 0] - R[0, 1]) / s
    elif R[0, 0] > R[1, 1] and R[0, 0] > R[2, 2]:
        s = 2.0 * np.sqrt(1.0 + R[0, 0] - R[1, 1] - R[2, 2])
        w = (R[2, 1] - R[1, 2]) / s
        x = 0.25 * s
        y = (R[0, 1] + R[1, 0]) / s
        z = (R[0, 2] + R[2, 0]) / s
    elif R[1, 1] > R[2, 2]:
        s = 2.0 * np.sqrt(1.0 + R[1, 1] - R[0, 0] - R[2, 2])
        w = (R[0, 2] - R[2, 0]) / s
        x = (R[0, 1] + R[1, 0]) / s
        y = 0.25 * s
        z = (R[1, 2] + R[2, 1]) / s
    else:
        s = 2.0 * np.sqrt(1.0 + R[2, 2] - R[0, 0] - R[1, 1])
        w = (R[1, 0] - R[0, 1]) / s
        x = (R[0, 2] + R[2, 0]) / s
        y = (R[1, 2] + R[2, 1]) / s
        z = 0.25 * s
    q = np.array([x, y, z, w])
    return q / np.linalg.norm(q)

class PointCloudGrasping(Node):
    def __init__(self):
        super().__init__('pc_processing_template')

        self.declare_parameter('target_frame', 'base_link')
        self.declare_parameter('processing_period_sec', 0.5)
        self.declare_parameter('voxel_size', 0.001)
        self.declare_parameter('merged_cloud_topic', '/multiview/merged')

        self.target_frame = self.get_parameter('target_frame').value
        self.voxel_size = float(self.get_parameter('voxel_size').value)
        self.cloud_topic = self.get_parameter('merged_cloud_topic').value
        self.period = float(self.get_parameter('processing_period_sec').value)

        self.normals = None
        self.pair = None

        self.latest_cloud = None
        self.cloud_generation = 0

        #depends on commented out code later in the file, 
        # but currently set to publish the segmented object
        self.object_publisher = self.create_publisher(
            PointCloud2, '~/processed_cloud/segmented_object', 1,
        ) 
        self.noplane_publisher = self.create_publisher(
            PointCloud2, '~/processed_cloud/no_plane', 1,
        ) 
        self.filter_downsample_publisher = self.create_publisher(
            PointCloud2, '~/processed_cloud/filter_downsample', 1,
        ) 


        #publishes centroid, contact points, and line of approach
        self.centroid_pub = self.create_publisher(
            PointCloud2, '/centroid_cloud', 10,
        )

        #publishes the grasp pose which is subscribed to by
        # the grasp_executor node later
        self.grasp_publisher = self.create_publisher(
            PoseStamped, '~/grasp_pose', 1,
        )

        #client for arm_servcices node for motion
        self.pose_client = self.create_client(
            SendPose, '/send_pose'
        )

        cloud_qos = QoSProfile(
            history=HistoryPolicy.KEEP_LAST,
            depth=5,
            reliability=ReliabilityPolicy.RELIABLE,
        )

        self.cloud_subscription = self.create_subscription(
            PointCloud2, self.cloud_topic, self.cloud_callback1, cloud_qos,
        )

        self.timer = self.create_timer(
            self.period, self.process_latest_cloud,
        )


    def wait_for_services(self, timeout=30.0):
        """Wait for the ArmServices node to expose the required services."""
        self.get_logger().info(
            'Waiting for ArmServices...'
        )

        pose_ready = self.pose_client.wait_for_service(
            timeout_sec=timeout
        )

        if not pose_ready:
            self.get_logger().error(
                'Service /send_pose is unavailable.'
            )
            return False

        self.get_logger().info(
            '/send_pose is available.'
        )

        return True

    def move_to_pose(
        self,
        x,
        y,
        z,
        qx=1.0,
        qy=0.0,
        qz=0.0,
        qw=0.0
    ):
        """
        Move the end effector using the /send_pose service.

        Position is in meters, relative to the configured base frame.
        Orientation is a quaternion [qx, qy, qz, qw].
        """
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

            deadline = time.monotonic() + 180.0

            while rclpy.ok() and not future.done():
                if time.monotonic() >= deadline:
                    self.get_logger().error(
                        'Timed out waiting for /send_pose.'
                    )
                    return False

                time.sleep(0.01)

            if not future.done():
                return False

            response = future.result()

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


    def cloud_callback1(self, msg):
        self.latest_cloud = msg
        self.cloud_generation += 1


    def process_latest_cloud(self): 
        """Process an incoming merged cloud and estimate a grasp pose."""

        if self.latest_cloud is None:
            return

        if not hasattr(self, 'last_processed_generation'):
            self.last_processed_generation = 0

        if self.cloud_generation == self.last_processed_generation:
            return

        msg = self.latest_cloud
        self.last_processed_generation = self.cloud_generation

        try:
            cloud = cloud_from_msg(msg)

            if len(cloud) == 0:
                self.get_logger().warning('Received an empty merged cloud.')
                return

            self.get_logger().info(
                f'Received merged cloud: {len(cloud)} points.'
            )

            # Voxel filtering.
            processed = self.process_cloud(cloud)

            if len(processed) == 0:
                self.get_logger().warning('Cloud is empty after filtering.')
                return

            # Remove support plane.
            plane_indices = self.planeSegment(processed)

            no_plane = processed.extract(
                plane_indices,
                negative=True,
            )

            if len(no_plane) == 0:
                self.get_logger().warning('No points remain after plane removal.')
                return

            # object clustering.
            objects = self.clusterObjects(no_plane)

            self.get_logger().info(
                f'Found {len(objects)} object clusters.'
            )


            header = Header(
                stamp=self.get_clock().now().to_msg(),
                frame_id=self.target_frame,
            )
            self.noplane_publisher.publish(
                point_cloud2.create_cloud_xyz32(
                    header,
                    no_plane.xyz,
                )
            )

            self.filter_downsample_publisher.publish(
                point_cloud2.create_cloud_xyz32(
                    header,
                    processed.xyz,
                )
            )

            if not objects:
                self.get_logger().warning('No object clusters found.')
                return

            # Select largest object.
            obj = max(objects, key=len)

            # Publish processed cloud.
            header = Header(
                stamp=self.get_clock().now().to_msg(),
                frame_id=self.target_frame,
            )

            self.object_publisher.publish(
                point_cloud2.create_cloud_xyz32(
                    header,
                    obj.xyz,
                )
            )
            # # Estimate surface normals.
            # self.normals = self.estimateNormals(obj)

            # # Find grasp contact pair.
            # pair = self.graspPairs(
            #     obj.xyz,
            #     self.normals,
            # )
            pair = self.graspPairs(obj.xyz)

            if pair is None:
                self.get_logger().warning(
                    'No valid contact pair found.'
                )
                return

            self.pair = pair

            # Calculate object centroid and approach direction.
            centroid = self.findCentroid(obj)

            approach = self.approachVector(
                pair,
            )

            line = self.approachLinePoints(
                pair,
                approach,
            )

            self.publishPoints(
                [centroid, pair[0], pair[1]] + line
            )

            # Estimate candidate grasp pose.
            pose = self.estimate_grasp()

            if pose is None:
                self.get_logger().warning(
                    'Could not estimate a valid grasp pose.'
                )
                return

            # Publish candidate grasp pose.
            self.grasp_publisher.publish(
                PoseStamped(
                    header=header,
                    pose=pose,
                )
            )

            self.get_logger().info(
                'Candidate grasp pose published in '
                f'{self.target_frame}.'
            )

        except (ValueError, RuntimeError, np.linalg.LinAlgError) as exc:
            self.get_logger().error(
                f'Cloud processing failed: {exc}'
            )

    def publishPoints(self, points):
        """
        publish a set of points as a PointCloud2 message
        """
        header = Header(stamp=self.get_clock().now().to_msg(), frame_id=self.target_frame)
        pts = [tuple(float(v) for v in p) for p in points]
        self.centroid_pub.publish(point_cloud2.create_cloud_xyz32(header, pts))

    def filterCloud(self, cloud):
        """
        passthrough filter with hardcoded values, currently disabled
        """

        cloud = cloud.pass_through(axis="z", minimum=-0.1, maximum=0.05)
        # cloud = cloud.pass_through(axis="y", minimum=-0.1, maximum=0.3)
        # cloud = cloud.pass_through(axis="x", minimum=0.3, maximum=0.8)

        return cloud
    
    def planeSegment(self, cloud):
        """
        takes a cloud input and returns all points on the plane using PCL
        built-in .segment_plane

        distance_threshold determines how much height deviation is valid for points
            to be considered on the plane
        max_iterations is how many times it samples points to find the plane consensus
        """

        inliers, _ = cloud.segment_plane(distance_threshold=0.002, max_iterations=10000)

        return inliers

    
    def clusterObjects(self, cloud):
        """
        clusters a point cloud into individual object clouds.
        """
        clusters = cloud.euclidean_clusters(tolerance=0.005, min_size=10, max_size=250)

        objectClouds = []
        for number, indices in enumerate(clusters):
            objectClouds.append(cloud.extract(indices))

        return objectClouds
        
    def process_cloud(self, cloud):
        """
        downsamples and calls the filtering(which isnt doing anything currently)
        """
        # cloud = cloud.voxel_grid(self.voxel_size)
        cloud = self.filterCloud(cloud)

        return cloud

    def findCentroid(self, object):
        """
        takes segmented object cloud and returns centroid
        """
        centroid = np.mean(object.xyz, axis=0)
        return centroid

    def estimateNormals(self, object):
        """
        takes object and returns the normals
        """
        centroid = self.findCentroid(object)
        return object.estimate_normals(radius=0.01, viewpoint=centroid)

    def graspPairs(self, points, center_band=0.25):

        points = np.asarray(points, dtype=np.float64)

        if len(points) < 4:
            return None

        # Object centroid
        centroid = np.mean(points, axis=0)

        # PCA in XY plane
        centered = points[:, :2] - centroid[:2]

        covariance = np.cov(centered.T)
        eigenvalues, eigenvectors = np.linalg.eigh(covariance)

        # Principal (longitudinal) axis
        longitudinal = eigenvectors[:, np.argmax(eigenvalues)]
        longitudinal /= np.linalg.norm(longitudinal)

        # Deterministic eigenvector sign
        if longitudinal[0] < 0 or (
            abs(longitudinal[0]) < 1e-9 and longitudinal[1] < 0
        ):
            longitudinal = -longitudinal

        # Transverse axis: perpendicular to screw length
        transverse = np.array([
            -longitudinal[1],
            longitudinal[0]
        ])

        # Project points onto both axes
        longitudinal_proj = centered @ longitudinal
        transverse_proj = centered @ transverse

        length = np.ptp(longitudinal_proj)
        width = np.ptp(transverse_proj)

        if length < 1e-6 or width < 1e-6:
            return None

        # Restrict contacts to center of screw length
        center_limit = length * center_band / 2.0

        mask = np.abs(longitudinal_proj) < center_limit

        candidates = np.where(mask)[0]

        if len(candidates) < 2:
            return None

        # Find opposing sides across the screw width
        projections = transverse_proj[candidates]

        low_idx = candidates[np.argmin(projections)]
        high_idx = candidates[np.argmax(projections)]

        # Symmetric grasp points centered on object centroid
        half_width = width / 2.0

        p0 = centroid.copy()
        p1 = centroid.copy()

        p0[:2] -= transverse * half_width
        p1[:2] += transverse * half_width

        # Keep grasp points at the centroid's height
        p0[2] = centroid[2]
        p1[2] = centroid[2]

        self.get_logger().info(
            f'Principal axis: {longitudinal}, '
            f'Transverse axis: {transverse}, '
            f'Length: {length:.4f}, Width: {width:.4f}'
        )

        return p0, p1

    def approachVector(self, pair=None):
        return np.array([0.0, 0.0, -1.0])

    def approachLinePoints(self, pair, approach, length=0.10, spacing=0.005):
        """
        takes contact pair, approach direction and returns a list of points corresponding
        to a visual of the how the end effector will aproach the object
        """
        grasp_center = (pair[0] + pair[1]) / 2.0
        steps = int(length / spacing)
        return [grasp_center - approach * (i * spacing) for i in range(steps + 1)]

    def estimate_grasp(self):

        if self.pair is None:
            return None

        p0 = np.asarray(self.pair[0], dtype=np.float64)
        p1 = np.asarray(self.pair[1], dtype=np.float64)

        center = (p0 + p1) / 2.0

        # Gripper Z axis: vertically downward
        z_axis = np.array([0.0, 0.0, -1.0])

        # Gripper Y axis: parallel to screw principal axis
        y_axis = p1 - p0
        y_axis[2] = 0.0
        y_axis /= np.linalg.norm(y_axis)

        # Deterministic orientation
        if y_axis[0] < 0 or (
            abs(y_axis[0]) < 1e-9 and y_axis[1] < 0
        ):
            y_axis = -y_axis

        # Complete right-handed orthogonal basis
        x_axis = np.cross(y_axis, z_axis)
        x_axis /= np.linalg.norm(x_axis)

        y_axis = np.cross(z_axis, x_axis)

        R = np.column_stack([
            x_axis,
            y_axis,
            z_axis
        ])

        qx, qy, qz, qw = rotation_to_quaternion(R)

        pose = Pose()

        pose.position.x = float(center[0])
        pose.position.y = float(center[1])
        pose.position.z = float(center[2])

        pose.orientation.x = float(qx)
        pose.orientation.y = float(qy)
        pose.orientation.z = float(qz)
        pose.orientation.w = float(qw)

        return pose



def main(args=None):
    rclpy.init(args=args)
    node = PointCloudGrasping()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()