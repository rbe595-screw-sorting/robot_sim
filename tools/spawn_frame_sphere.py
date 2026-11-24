#!/usr/bin/env python3

import os
import tempfile
import subprocess

import rclpy
from rclpy.node import Node

from tf2_ros import Buffer, TransformListener
from rclpy.time import Time


class SpawnFrameSphere(Node):

    def __init__(self):
        super().__init__('spawn_frame_sphere')

        # Parameters
        self.declare_parameter('frame_id', 'bin')
        self.declare_parameter('reference_frame', 'base_link')
        self.declare_parameter('height', 0.10)
        self.declare_parameter('radius', 0.005)
        self.declare_parameter('world', 'rbe595')
        self.declare_parameter('entity_name', 'debug_sphere6')

        self.frame_id = self.get_parameter('frame_id').value
        self.reference_frame = self.get_parameter('reference_frame').value
        self.height = self.get_parameter('height').value
        self.radius = self.get_parameter('radius').value
        self.world = self.get_parameter('world').value
        self.entity_name = self.get_parameter('entity_name').value

        # TF
        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)

        self.timer = self.create_timer(0.5, self.spawn)

        self.get_logger().info(
            f"Waiting for TF: {self.reference_frame} -> {self.frame_id}"
        )

    def spawn(self):

        try:
            transform = self.tf_buffer.lookup_transform(
                self.reference_frame,
                self.frame_id,
                Time()
            )

        except Exception as e:
            self.get_logger().warn(f"Waiting for transform: {e}")
            return

        # Position relative to the reference frame
        x = transform.transform.translation.x
        y = transform.transform.translation.y
        z = transform.transform.translation.z + self.height

        self.get_logger().info(
            f"Spawning sphere at ({x:.3f}, {y:.3f}, {z:.3f})"
        )

        # Simple static red sphere SDF
        sdf = f"""<?xml version="1.0"?>
<sdf version="1.8">
  <model name="{self.entity_name}">
    <static>true</static>
    <link name="sphere_link">
      <visual name="sphere_visual">
        <geometry>
          <sphere>
            <radius>{self.radius}</radius>
          </sphere>
        </geometry>
        <material>
          <ambient>1 0 0 1</ambient>
          <diffuse>1 0 0 1</diffuse>
          <emissive>0.3 0 0 1</emissive>
        </material>
      </visual>
    </link>
  </model>
</sdf>
"""

        # Write temporary SDF
        with tempfile.NamedTemporaryFile(
            mode='w', suffix='.sdf', delete=False
        ) as f:
            f.write(sdf)
            sdf_path = f.name

        try:
            result = subprocess.run(
                [
                    'ros2', 'run', 'ros_gz_sim', 'create',
                    '--world', self.world,
                    '--file', sdf_path,
                    '--name', self.entity_name,
                    '--x', str(x),
                    '--y', str(y),
                    '--z', str(z),
                ],
                capture_output=True,
                text=True,
                timeout=15
            )

            if result.returncode == 0:
                self.get_logger().info(
                    f"Successfully spawned {self.entity_name}"
                )
            else:
                self.get_logger().error(
                    f"Gazebo spawn failed:\n{result.stderr}"
                )

        except Exception as e:
            self.get_logger().error(f"Spawn error: {e}")

        finally:
            os.unlink(sdf_path)

        # One-shot spawn
        self.timer.cancel()


def main(args=None):

    rclpy.init(args=args)
    node = SpawnFrameSphere()

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()