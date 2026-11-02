# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target control_msgs::control_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${control_msgs_TARGETS}.
if(control_msgs_TARGETS AND NOT TARGET control_msgs::control_msgs)
  add_library(control_msgs::control_msgs INTERFACE IMPORTED)
  set_target_properties(control_msgs::control_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${control_msgs_TARGETS}")
endif()
