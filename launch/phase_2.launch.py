from launch import LaunchDescription
from launch_ros.actions import Node
import os
from ament_index_python.packages import get_package_share_directory


def generate_launch_description():
    pkg_path = get_package_share_directory('robot_sim')
    config_file = os.path.join(
        pkg_path,
        'config',
        'phase_2.yaml'
    )
    return LaunchDescription([

        # State machine
        Node(
            package='robot_sim',
            executable='phase_2',
            name='phase_2',
            output='screen',
            parameters=[config_file]
        ),

        # Perception
        Node(
            package='robot_sim',
            executable='detection',
            name='detection',
            output='screen',
            parameters=[config_file]
        ),
        Node(
            package='robot_sim',
            executable='aruco_detection',
            name='aruco_detection',
            output='screen',
            parameters=[config_file]
        ),
        Node(
            package='robot_sim',
            executable='localization',
            name='localization',
            output='screen',
            parameters=[config_file]
        ),
        Node(
            package='robot_sim',
            executable='static_transforms',
            name='static_transforms',
            output='screen',
            parameters=[config_file]
        ),

        # Robot motion
        Node(
            package='robot_sim',
            executable='arm_services_node',
            name='arm_services_node',
            output='screen',
            parameters=[config_file]
        ),
        # Node(
        #     package='robot_sim',
        #     executable='gripper_executor',
        #     name='gripper_executor',
        #     output='screen',
        #     parameters=[config_file]
        # ),

    ])