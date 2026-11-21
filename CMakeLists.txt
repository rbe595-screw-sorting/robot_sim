cmake_minimum_required(VERSION 3.8)
project(robot_sim)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(ament_cmake_python REQUIRED)
# uncomment the following section in order to fill in
# further dependencies manually.
# find_package(<dependency> REQUIRED)
ament_python_install_package(helpers)
install(
  FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/pcl_python_merlab.cpython-312-x86_64-linux-gnu.so
  DESTINATION lib/${PROJECT_NAME}
)

install(
  DIRECTORY     
    launch
    worlds
    models
    robots
    arms
    grippers
    config
    helpers
  DESTINATION share/${PROJECT_NAME}
)

install(
  PROGRAMS helpers/finger_relay.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME finger_relay
)


# Manipulation
install(
  PROGRAMS helpers/arm_services_node.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME arm_services_node
)

install(
  PROGRAMS nodes/manipulation/gripper_executor.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME gripper_executor
)

# Perception
install(
  PROGRAMS nodes/perception/screw_detection.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME screw_detection
)

install(
  PROGRAMS nodes/perception/cloud_manager.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME cloud_manager
)

install(
  PROGRAMS nodes/perception/static_transforms.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME static_transforms
)

install(
  PROGRAMS nodes/perception/point_cloud_processing.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME point_cloud_processing
)


install(
  PROGRAMS nodes/perception/aruco_detection.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME aruco_detection
)

install(
  PROGRAMS nodes/perception/localization.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME localization
)

# State machine
install(
  PROGRAMS nodes/state_machine/phase_2.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME phase_2
)

# Utilities
install(
  PROGRAMS nodes/utilities/visualization.py
  DESTINATION lib/${PROJECT_NAME}
  RENAME visualization
)
if(BUILD_TESTING)
  find_package(ament_lint_auto REQUIRED)
  # the following line skips the linter which checks for copyrights
  # comment the line when a copyright and license is added to all source files
  set(ament_cmake_copyright_FOUND TRUE)
  # the following line skips cpplint (only works in a git repo)
  # comment the line when this package is in a git repo and when
  # a copyright and license is added to all source files
  set(ament_cmake_cpplint_FOUND TRUE)
  ament_lint_auto_find_test_dependencies()
endif()

ament_package()