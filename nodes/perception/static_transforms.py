#!/usr/bin/env python3

import yaml

import rclpy
from rclpy.node import Node

from geometry_msgs.msg import TransformStamped
from tf2_ros.static_transform_broadcaster import StaticTransformBroadcaster

from ament_index_python.packages import get_package_share_directory
import os
from tf_transformations import quaternion_from_euler



class StaticTransforms(Node):

    def __init__(self):

        super().__init__('static_transforms')

        self.broadcaster = StaticTransformBroadcaster(self)

        package_path = get_package_share_directory('robot_sim')

        config_path = os.path.join(
            package_path,
            'config',
            'static_transforms.yaml'
        )

        with open(config_path, 'r') as file:
            config = yaml.safe_load(file)

        transforms = []
        for name, grid_config in config.get('grids', {}).items():

            grid_transforms = self.generate_grid_transforms(
                grid_config
            )

            transforms.extend(grid_transforms)

        for name, data in config['static_transforms'].items():

            transform = TransformStamped()

            transform.header.stamp = self.get_clock().now().to_msg()

            transform.header.frame_id = data['parent']
            transform.child_frame_id = data['child']

            transform.transform.translation.x = data['translation'][0]
            transform.transform.translation.y = data['translation'][1]
            transform.transform.translation.z = data['translation'][2]

            # Euler angles (roll, pitch, yaw) -> quaternion
            q = quaternion_from_euler(*data['rotation'])

            transform.transform.rotation.x = q[0]
            transform.transform.rotation.y = q[1]
            transform.transform.rotation.z = q[2]
            transform.transform.rotation.w = q[3]

            transforms.append(transform)

            self.get_logger().info(
                f'Publishing static transform: '
                f'{data["parent"]} -> {data["child"]}'
            )

        self.broadcaster.sendTransform(transforms)

    def generate_grid_transforms(self, grid_config):

        transforms = []

        parent = grid_config['parent']

        rows = grid_config['rows']
        columns = grid_config['columns']

        spacing_x = grid_config['spacing']['x']
        spacing_y = grid_config['spacing']['y']

        origin = grid_config['origin']

        rotation = grid_config['rotation']

        prefix = grid_config['frame_prefix']

        # Grid orientation
        q = quaternion_from_euler(*rotation)

        # Rotation matrix
        from tf_transformations import quaternion_matrix
        R = quaternion_matrix(q)[:3, :3]

        for row in range(rows):

            for col in range(columns):

                # Position in the grid's local coordinate system
                local_offset = [
                    col * spacing_x,
                    row * spacing_y,
                    0.0
                ]

                # Rotate offset into the parent frame
                rotated_offset = R @ local_offset

                transform = TransformStamped()

                transform.header.stamp = self.get_clock().now().to_msg()
                transform.header.frame_id = parent

                transform.child_frame_id = f'{prefix}_{row}_{col}'

                transform.transform.translation.x = (
                    origin['x'] + rotated_offset[0]
                )

                transform.transform.translation.y = (
                    origin['y'] + rotated_offset[1]
                )

                transform.transform.translation.z = (
                    origin['z'] + rotated_offset[2]
                )

                transform.transform.rotation.x = q[0]
                transform.transform.rotation.y = q[1]
                transform.transform.rotation.z = q[2]
                transform.transform.rotation.w = q[3]

                transforms.append(transform)

        return transforms


def main(args=None):

    rclpy.init(args=args)

    node = StaticTransforms()

    try:
        rclpy.spin(node)

    except KeyboardInterrupt:
        pass

    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()