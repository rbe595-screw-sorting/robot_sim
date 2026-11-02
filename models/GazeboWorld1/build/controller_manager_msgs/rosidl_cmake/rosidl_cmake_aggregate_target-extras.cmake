# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target controller_manager_msgs::controller_manager_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${controller_manager_msgs_TARGETS}.
if(controller_manager_msgs_TARGETS AND NOT TARGET controller_manager_msgs::controller_manager_msgs)
  add_library(controller_manager_msgs::controller_manager_msgs INTERFACE IMPORTED)
  set_target_properties(controller_manager_msgs::controller_manager_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${controller_manager_msgs_TARGETS}")
endif()
