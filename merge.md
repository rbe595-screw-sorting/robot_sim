# Merge Notes
Things that were updated in merging both phases

1. `sim_gazebo` xacro argument is kept in `grippers/gen3_lite_2f/urdf/gen3_lite_2f_macro.xacro` seems like it might effect phase 1 launch files for sim.
2. camera frame in sim (`robots/kinova.urdf.xacro`) is renamed to have the phase 2 branch frame name `camera_color_frame`
3. removed build artifacts from repo (build, log, and install folders)
4. `grippers/gen3_lite_2f/urdf/gen3_lite_2f.ros2_control.xacro` now uses `GazeboSimSystem` instead of `GzSimSystem`. I decided this by checking the [ros2 control docs](https://control.ros.org/jazzy/doc/gz_ros2_control/doc/index.html#add-ros2-control-tag-to-a-urdf-or-sdf)
5. `arms/gen3_lite/6dof/config/ros2_controllers.yaml` now contains the `finger_follower_controller`. Also contains configuration params at the end of the file.
6. `grippers/gen3_lite_2f/urdf/gen3_lite_2f.ros2_control.xacro` ended up favoring phase 2's setup. 
7. `scripts/phase_2.py` phase 2 was prioritized, kept those changes and discarded phase 1 differences.
8. `scripts/robot_motion.py` contains the version found on the phase 2 branch. `scripts/robot_motion_1.py` contains the phase 1 version.

## Questions I have:
- why does this exist? `models/bingrid/launch/rbe595_scoopat.launch`
- what can be removed/what is dead code?

## Things to fix
```
Unmerged paths:
  (use "git add/rm <file>..." as appropriate to mark resolution)
	both added:      helpers/arm_services_node.py
	both added:      helpers/robot_motion.py
```