// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ros_gz_interfaces:msg/AirSpeed.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "ros_gz_interfaces/msg/air_speed.h"


#ifndef ROS_GZ_INTERFACES__MSG__DETAIL__AIR_SPEED__STRUCT_H_
#define ROS_GZ_INTERFACES__MSG__DETAIL__AIR_SPEED__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Constants defined in the message

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.h"

/// Struct defined in msg/AirSpeed in the package ros_gz_interfaces.
/**
  * A message for AirSpeed readings.
 */
typedef struct ros_gz_interfaces__msg__AirSpeed
{
  /// Optional header data.
  std_msgs__msg__Header header;
  /// Differential pressure data, in hectopascals (hPa), matching gz.msgs.AirSpeed.
  double diff_pressure;
  /// Temperature data, in kelvin.
  double temperature;
} ros_gz_interfaces__msg__AirSpeed;

// Struct for a sequence of ros_gz_interfaces__msg__AirSpeed.
typedef struct ros_gz_interfaces__msg__AirSpeed__Sequence
{
  ros_gz_interfaces__msg__AirSpeed * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ros_gz_interfaces__msg__AirSpeed__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ROS_GZ_INTERFACES__MSG__DETAIL__AIR_SPEED__STRUCT_H_
