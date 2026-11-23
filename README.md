# robot_sim
For edits made to prevent issues while merging, see `merge.md`. Ask Kylie if you have questions.

> Note: DO NOT BUILD IN THE REPO!

## Setting up
1. clone the following packages into your **workspace not the repo**:
- https://github.com/Kinovarobotics/ros2_kortex.git


2. build and source in the workspace root: </br>
`colcon build --symlink-install && source install/setup.bash`


terminal 1: ros2 launch robot_sim sim.launch.py world:=rbe595_phase1

terminal 2: ros2 launch robot_sim move_group_sim.launch.py

terminal 3: python3 -m scripts.phase_2

gazebo has a tendency to not close gracefully, if you are getting some strange stale behaviors:

pkill -9 -f "gz sim"; pkill -9 ruby; pkill -f parameter_bridge; sleep 3


### From Phase 2 README
terminal 1: ros2 launch kinova_gen3_lite_moveit_config robot.launch.py robot_ip:=192.168.1.10

terminal 2: ros2 run tf2_ros static_transform_publisher --x -0.05996 --y 0.01561 --z 0.04815 --qx -0.003040 --qy -0.703660 --qz -0.015836 --qw 0.710354 --frame-id end_effector_link --child-frame-id camera_link

terminal 3:     ros2 launch realsense2_camera rs_launch.py   align_depth.enable:=true   decimation_filter.enable:=true   spatial_filter.enable:=true   temporal_filter.enable:=true   hole_filling_filter.enable:=true pointcloud.enable:=true

terminal 4: python3 -m scripts.phase_2

### From Phase 1 README
-- 
Terminal 1: to launch in world: </br>
'ros2 launch robot_sim sim.launch.py world:=rbe595_phase1'


Terminal 2: to launch move it: </br> 
'ros2 launch kinova_gen3_lite_moveit_config move_group.launch.py'

Terminal 3: to launch phase 1 controller: </br>
'python3 src/robot_sim/scripts/phase_1.py'


