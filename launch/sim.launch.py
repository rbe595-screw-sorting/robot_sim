import os

from ament_index_python import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription, SetEnvironmentVariable, TimerAction
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description():
    # base file path for the package
    pkg_path = os.path.join(get_package_share_directory('robot_sim'))

    # secondary file paths for locating resources
    models_path = os.path.join(pkg_path, 'models')
    worlds_path = os.path.join(pkg_path, 'worlds')

    camera_bridge = Node(
        package='ros_gz_bridge',
        executable='parameter_bridge',
        arguments=[
            '/wrist_mounted_camera/image@sensor_msgs/msg/Image@gz.msgs.Image'
        ],
        remappings=[
            ('/wrist_mounted_camera/image', '/camera/camera/color/image_raw')
        ],
        output='screen',
        parameters=[{'use_sim_time': True}]
    )

    def get_available_worlds():
        try:
            world_list = [f[:-4] for f in os.listdir(worlds_path) if f.endswith('.sdf')]
            if not world_list:
                return "directory empty"
            return "Gz Sim Worlds: " + ", ".join(world_list)
        except Exception as e:
            return f"ERROR: {e}"
        
    worlds = get_available_worlds()

    # set gz sim resource path
    gz_sim_resource = SetEnvironmentVariable(
        name='GZ_SIM_RESOURCE_PATH',
        value=[
            pkg_path,
            ':',
            models_path,
            ':',
            os.environ.get('GZ_SIM_RESOURCE_PATH', '')
        ]
    )

    # arguments for gz sim
    arguments = [
        DeclareLaunchArgument('world', default_value='empty', description=worlds),
    ]

    # actually run gazebo
    ros_gz_sim = get_package_share_directory('ros_gz_sim')
    ros_gz_sim_src = PythonLaunchDescriptionSource(
        os.path.join(ros_gz_sim, 'launch', 'gz_sim.launch.py')
    )

    gzserver_cmd = IncludeLaunchDescription(
        ros_gz_sim_src,
        launch_arguments=[
            ('gz_args', [LaunchConfiguration('world'), '.sdf -v 4 -r -s']),
            ('on_exit_shutdown', 'true')
        ]
    )

    gzclient_cmd = IncludeLaunchDescription(
        ros_gz_sim_src,
        launch_arguments=[
            ('gz_args', ['-v 4 -g']),
            ('on_exit_shutdown', 'true')
        ]
    )

    #launch the robot: 
    kinova_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(pkg_path, 'launch', 'kortex_sim_control.launch.py')
        ),
        launch_arguments={
            'sim_gazebo': 'true',
            'start_gazebo': 'false',
            'robot_type': 'gen3_lite',
            'robot_controller': 'joint_trajectory_controller',
            'robot_pos_controller': 'twist_controller',
            'dof': '6',
            # 'description_file': 'gen3_lite_gen3_lite_2f.xacro',
            'description_file' : 'kinova.urdf.xacro',
            'robot_name': 'gen3_lite',
            'gripper': 'gen3_lite_2f',
            'launch_rviz': 'false',
            'use_sim_time': 'true',
            'spawn_x': LaunchConfiguration('robot_x'),
            'spawn_y': LaunchConfiguration('robot_y'),
            'spawn_z': LaunchConfiguration('robot_z'),
            'vision' : 'true'
        }.items()
    )

    deployed_kinova_launch = TimerAction(
        period = 8.0,
        actions = [kinova_launch]
    )

    # launch each item defined above by returning the variable
    return LaunchDescription([
        gz_sim_resource,
        *arguments,

        DeclareLaunchArgument('robot_x', default_value='0.0'),
        DeclareLaunchArgument('robot_y', default_value='0.0'),
        DeclareLaunchArgument('robot_z', default_value='0.0'),

        DeclareLaunchArgument('launch_rviz', default_value='false'),

        gzserver_cmd,
        gzclient_cmd,

        deployed_kinova_launch,

        camera_bridge,
    ])