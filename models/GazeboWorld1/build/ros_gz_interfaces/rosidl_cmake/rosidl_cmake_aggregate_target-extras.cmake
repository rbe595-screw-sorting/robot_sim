# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target ros_gz_interfaces::ros_gz_interfaces
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${ros_gz_interfaces_TARGETS}.
if(ros_gz_interfaces_TARGETS AND NOT TARGET ros_gz_interfaces::ros_gz_interfaces)
  add_library(ros_gz_interfaces::ros_gz_interfaces INTERFACE IMPORTED)
  set_target_properties(ros_gz_interfaces::ros_gz_interfaces PROPERTIES
    INTERFACE_LINK_LIBRARIES "${ros_gz_interfaces_TARGETS}")
endif()
