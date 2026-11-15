#!/usr/bin/env python3

import rclpy
from rclpy.node import Node


class Localization(Node):

    def __init__(self):
        super().__init__('localization')

        self.get_logger().info('NODE_NAME initialized!')

        # TODO: Initialize publishers, subscribers, services, etc.


def main(args=None):
    rclpy.init(args=args)

    node = Localization()

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()