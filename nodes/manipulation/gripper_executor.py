#!/usr/bin/env python3

import rclpy
from rclpy.node import Node


class GripperExecutor(Node):

    def __init__(self):
        super().__init__('gripper_exectuor')

        self.get_logger().info('NODE_NAME initialized!')

        # TODO: Initialize publishers, subscribers, services, etc.


def main(args=None):
    rclpy.init(args=args)

    node = GripperExecutor()

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()