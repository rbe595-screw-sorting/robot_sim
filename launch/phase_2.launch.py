from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([

        # State machine
        Node(
            package='robot_sim',
            executable='phase_2',
            name='phase_2',
            output='screen'
        ),

        # Perception
        Node(
            package='robot_sim',
            executable='screw_detection',
            name='screw_detection',
            output='screen'
        ),
        Node(
            package='robot_sim',
            executable='aruco_detection',
            name='aruco_detection',
            output='screen'
        ),
        Node(
            package='robot_sim',
            executable='localization',
            name='localization',
            output='screen'
        ),

        # Robot motion
        Node(
            package='robot_sim',
            executable='arm_services_node',
            name='arm_services_node',
            output='screen'
        ),
        Node(
            package='robot_sim',
            executable='gripper_executor',
            name='gripper_executor',
            output='screen'
        ),

    ])