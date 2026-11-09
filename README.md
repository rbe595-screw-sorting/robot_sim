# robot_sim

## Setting up
1. clone the following packages into your **workspace not the repo**:
- https://github.com/Kinovarobotics/ros2_kortex.git


2. build and source in the workspace root: </br>
`colcon build --symlink-install && source install/setup.bash`

3. to launch an empty world with the arm (no controllers yet?) </br>
`ros2 launch kortex_bringup kortex_sim_control.launch.py launch_rviz:=false description_file:=gen3_lite.urdf`

4. to launch in world: ros2 launch robot_sim sim.launch.py world:=rbe595_phase1

