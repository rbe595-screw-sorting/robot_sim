#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from sensor_msgs.msg import JointState
from std_msgs.msg import Float64MultiArray

class Relay(Node):
    def __init__(self):
        super().__init__('finger_relay')
        self.pub = self.create_publisher(Float64MultiArray, '/finger_followers_controller/commands', 10)
        self.create_subscription(JointState, '/joint_states', self.cb, 10)

    def cb(self, msg):
        if 'right_finger_bottom_joint' not in msg.name:
            return
        p = msg.position[msg.name.index('right_finger_bottom_joint')]
        out = Float64MultiArray()
        # order: left_finger_bottom, right_finger_tip, left_finger_tip
        out.data = [1.0 * p + 0.0, -0.276 * p - 0.1, -0.276 * p - 0.1]
        self.pub.publish(out)

rclpy.init()
rclpy.spin(Relay())