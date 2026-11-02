// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from ros_gz_interfaces:msg/WorldStatistics.idl
// generated code does not contain a copyright notice
#include "ros_gz_interfaces/msg/detail/world_statistics__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <cstddef>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/serialization_helpers.hpp"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "ros_gz_interfaces/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "ros_gz_interfaces/msg/detail/world_statistics__struct.h"
#include "ros_gz_interfaces/msg/detail/world_statistics__functions.h"
#include "fastcdr/Cdr.h"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif

#include "builtin_interfaces/msg/detail/time__functions.h"  // pause_time, real_time, sim_time, step_size
#include "ros_gz_interfaces/msg/detail/log_playback_statistics__functions.h"  // log_playback_statistics
#include "std_msgs/msg/detail/header__functions.h"  // header

// forward declare type support functions

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
bool cdr_serialize_builtin_interfaces__msg__Time(
  const builtin_interfaces__msg__Time * ros_message,
  eprosima::fastcdr::Cdr & cdr);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
bool cdr_deserialize_builtin_interfaces__msg__Time(
  eprosima::fastcdr::Cdr & cdr,
  builtin_interfaces__msg__Time * ros_message);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t get_serialized_size_builtin_interfaces__msg__Time(
  const void * untyped_ros_message,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t max_serialized_size_builtin_interfaces__msg__Time(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
bool cdr_serialize_key_builtin_interfaces__msg__Time(
  const builtin_interfaces__msg__Time * ros_message,
  eprosima::fastcdr::Cdr & cdr);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t get_serialized_size_key_builtin_interfaces__msg__Time(
  const void * untyped_ros_message,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t max_serialized_size_key_builtin_interfaces__msg__Time(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, builtin_interfaces, msg, Time)();

bool cdr_serialize_ros_gz_interfaces__msg__LogPlaybackStatistics(
  const ros_gz_interfaces__msg__LogPlaybackStatistics * ros_message,
  eprosima::fastcdr::Cdr & cdr);

bool cdr_deserialize_ros_gz_interfaces__msg__LogPlaybackStatistics(
  eprosima::fastcdr::Cdr & cdr,
  ros_gz_interfaces__msg__LogPlaybackStatistics * ros_message);

size_t get_serialized_size_ros_gz_interfaces__msg__LogPlaybackStatistics(
  const void * untyped_ros_message,
  size_t current_alignment);

size_t max_serialized_size_ros_gz_interfaces__msg__LogPlaybackStatistics(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

bool cdr_serialize_key_ros_gz_interfaces__msg__LogPlaybackStatistics(
  const ros_gz_interfaces__msg__LogPlaybackStatistics * ros_message,
  eprosima::fastcdr::Cdr & cdr);

size_t get_serialized_size_key_ros_gz_interfaces__msg__LogPlaybackStatistics(
  const void * untyped_ros_message,
  size_t current_alignment);

size_t max_serialized_size_key_ros_gz_interfaces__msg__LogPlaybackStatistics(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, ros_gz_interfaces, msg, LogPlaybackStatistics)();

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
bool cdr_serialize_std_msgs__msg__Header(
  const std_msgs__msg__Header * ros_message,
  eprosima::fastcdr::Cdr & cdr);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
bool cdr_deserialize_std_msgs__msg__Header(
  eprosima::fastcdr::Cdr & cdr,
  std_msgs__msg__Header * ros_message);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t get_serialized_size_std_msgs__msg__Header(
  const void * untyped_ros_message,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t max_serialized_size_std_msgs__msg__Header(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
bool cdr_serialize_key_std_msgs__msg__Header(
  const std_msgs__msg__Header * ros_message,
  eprosima::fastcdr::Cdr & cdr);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t get_serialized_size_key_std_msgs__msg__Header(
  const void * untyped_ros_message,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
size_t max_serialized_size_key_std_msgs__msg__Header(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

ROSIDL_TYPESUPPORT_FASTRTPS_C_IMPORT_ros_gz_interfaces
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, std_msgs, msg, Header)();


using _WorldStatistics__ros_msg_type = ros_gz_interfaces__msg__WorldStatistics;


ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_ros_gz_interfaces
bool cdr_serialize_ros_gz_interfaces__msg__WorldStatistics(
  const ros_gz_interfaces__msg__WorldStatistics * ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Field name: header
  {
    cdr_serialize_std_msgs__msg__Header(
      &ros_message->header, cdr);
  }

  // Field name: sim_time
  {
    cdr_serialize_builtin_interfaces__msg__Time(
      &ros_message->sim_time, cdr);
  }

  // Field name: pause_time
  {
    cdr_serialize_builtin_interfaces__msg__Time(
      &ros_message->pause_time, cdr);
  }

  // Field name: real_time
  {
    cdr_serialize_builtin_interfaces__msg__Time(
      &ros_message->real_time, cdr);
  }

  // Field name: paused
  {
    cdr << (ros_message->paused ? true : false);
  }

  // Field name: iterations
  {
    cdr << ros_message->iterations;
  }

  // Field name: model_count
  {
    cdr << ros_message->model_count;
  }

  // Field name: log_playback_statistics
  {
    cdr_serialize_ros_gz_interfaces__msg__LogPlaybackStatistics(
      &ros_message->log_playback_statistics, cdr);
  }

  // Field name: real_time_factor
  {
    cdr << ros_message->real_time_factor;
  }

  // Field name: step_size
  {
    cdr_serialize_builtin_interfaces__msg__Time(
      &ros_message->step_size, cdr);
  }

  // Field name: stepping
  {
    cdr << (ros_message->stepping ? true : false);
  }

  return true;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_ros_gz_interfaces
bool cdr_deserialize_ros_gz_interfaces__msg__WorldStatistics(
  eprosima::fastcdr::Cdr & cdr,
  ros_gz_interfaces__msg__WorldStatistics * ros_message)
{
  // Field name: header
  {
    cdr_deserialize_std_msgs__msg__Header(cdr, &ros_message->header);
  }

  // Field name: sim_time
  {
    cdr_deserialize_builtin_interfaces__msg__Time(cdr, &ros_message->sim_time);
  }

  // Field name: pause_time
  {
    cdr_deserialize_builtin_interfaces__msg__Time(cdr, &ros_message->pause_time);
  }

  // Field name: real_time
  {
    cdr_deserialize_builtin_interfaces__msg__Time(cdr, &ros_message->real_time);
  }

  // Field name: paused
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->paused = tmp ? true : false;
  }

  // Field name: iterations
  {
    cdr >> ros_message->iterations;
  }

  // Field name: model_count
  {
    cdr >> ros_message->model_count;
  }

  // Field name: log_playback_statistics
  {
    cdr_deserialize_ros_gz_interfaces__msg__LogPlaybackStatistics(cdr, &ros_message->log_playback_statistics);
  }

  // Field name: real_time_factor
  {
    cdr >> ros_message->real_time_factor;
  }

  // Field name: step_size
  {
    cdr_deserialize_builtin_interfaces__msg__Time(cdr, &ros_message->step_size);
  }

  // Field name: stepping
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->stepping = tmp ? true : false;
  }

  return true;
}  // NOLINT(readability/fn_size)


ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_ros_gz_interfaces
size_t get_serialized_size_ros_gz_interfaces__msg__WorldStatistics(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _WorldStatistics__ros_msg_type * ros_message = static_cast<const _WorldStatistics__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Field name: header
  current_alignment += get_serialized_size_std_msgs__msg__Header(
    &(ros_message->header), current_alignment);

  // Field name: sim_time
  current_alignment += get_serialized_size_builtin_interfaces__msg__Time(
    &(ros_message->sim_time), current_alignment);

  // Field name: pause_time
  current_alignment += get_serialized_size_builtin_interfaces__msg__Time(
    &(ros_message->pause_time), current_alignment);

  // Field name: real_time
  current_alignment += get_serialized_size_builtin_interfaces__msg__Time(
    &(ros_message->real_time), current_alignment);

  // Field name: paused
  {
    size_t item_size = sizeof(ros_message->paused);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: iterations
  {
    size_t item_size = sizeof(ros_message->iterations);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: model_count
  {
    size_t item_size = sizeof(ros_message->model_count);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: log_playback_statistics
  current_alignment += get_serialized_size_ros_gz_interfaces__msg__LogPlaybackStatistics(
    &(ros_message->log_playback_statistics), current_alignment);

  // Field name: real_time_factor
  {
    size_t item_size = sizeof(ros_message->real_time_factor);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: step_size
  current_alignment += get_serialized_size_builtin_interfaces__msg__Time(
    &(ros_message->step_size), current_alignment);

  // Field name: stepping
  {
    size_t item_size = sizeof(ros_message->stepping);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}


ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_ros_gz_interfaces
size_t max_serialized_size_ros_gz_interfaces__msg__WorldStatistics(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // Field name: header
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_std_msgs__msg__Header(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: sim_time
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: pause_time
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: real_time
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: paused
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Field name: iterations
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Field name: model_count
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  // Field name: log_playback_statistics
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_ros_gz_interfaces__msg__LogPlaybackStatistics(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: real_time_factor
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Field name: step_size
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: stepping
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }


  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = ros_gz_interfaces__msg__WorldStatistics;
    is_plain =
      (
      offsetof(DataType, stepping) +
      last_member_size
      ) == ret_val;
  }
  return ret_val;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_ros_gz_interfaces
bool cdr_serialize_key_ros_gz_interfaces__msg__WorldStatistics(
  const ros_gz_interfaces__msg__WorldStatistics * ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Field name: header
  {
    cdr_serialize_key_std_msgs__msg__Header(
      &ros_message->header, cdr);
  }

  // Field name: sim_time
  {
    cdr_serialize_key_builtin_interfaces__msg__Time(
      &ros_message->sim_time, cdr);
  }

  // Field name: pause_time
  {
    cdr_serialize_key_builtin_interfaces__msg__Time(
      &ros_message->pause_time, cdr);
  }

  // Field name: real_time
  {
    cdr_serialize_key_builtin_interfaces__msg__Time(
      &ros_message->real_time, cdr);
  }

  // Field name: paused
  {
    cdr << (ros_message->paused ? true : false);
  }

  // Field name: iterations
  {
    cdr << ros_message->iterations;
  }

  // Field name: model_count
  {
    cdr << ros_message->model_count;
  }

  // Field name: log_playback_statistics
  {
    cdr_serialize_key_ros_gz_interfaces__msg__LogPlaybackStatistics(
      &ros_message->log_playback_statistics, cdr);
  }

  // Field name: real_time_factor
  {
    cdr << ros_message->real_time_factor;
  }

  // Field name: step_size
  {
    cdr_serialize_key_builtin_interfaces__msg__Time(
      &ros_message->step_size, cdr);
  }

  // Field name: stepping
  {
    cdr << (ros_message->stepping ? true : false);
  }

  return true;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_ros_gz_interfaces
size_t get_serialized_size_key_ros_gz_interfaces__msg__WorldStatistics(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _WorldStatistics__ros_msg_type * ros_message = static_cast<const _WorldStatistics__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;

  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Field name: header
  current_alignment += get_serialized_size_key_std_msgs__msg__Header(
    &(ros_message->header), current_alignment);

  // Field name: sim_time
  current_alignment += get_serialized_size_key_builtin_interfaces__msg__Time(
    &(ros_message->sim_time), current_alignment);

  // Field name: pause_time
  current_alignment += get_serialized_size_key_builtin_interfaces__msg__Time(
    &(ros_message->pause_time), current_alignment);

  // Field name: real_time
  current_alignment += get_serialized_size_key_builtin_interfaces__msg__Time(
    &(ros_message->real_time), current_alignment);

  // Field name: paused
  {
    size_t item_size = sizeof(ros_message->paused);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: iterations
  {
    size_t item_size = sizeof(ros_message->iterations);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: model_count
  {
    size_t item_size = sizeof(ros_message->model_count);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: log_playback_statistics
  current_alignment += get_serialized_size_key_ros_gz_interfaces__msg__LogPlaybackStatistics(
    &(ros_message->log_playback_statistics), current_alignment);

  // Field name: real_time_factor
  {
    size_t item_size = sizeof(ros_message->real_time_factor);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  // Field name: step_size
  current_alignment += get_serialized_size_key_builtin_interfaces__msg__Time(
    &(ros_message->step_size), current_alignment);

  // Field name: stepping
  {
    size_t item_size = sizeof(ros_message->stepping);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_ros_gz_interfaces
size_t max_serialized_size_key_ros_gz_interfaces__msg__WorldStatistics(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;
  // Field name: header
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_key_std_msgs__msg__Header(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: sim_time
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_key_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: pause_time
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_key_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: real_time
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_key_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: paused
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Field name: iterations
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Field name: model_count
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  // Field name: log_playback_statistics
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_key_ros_gz_interfaces__msg__LogPlaybackStatistics(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: real_time_factor
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Field name: step_size
  {
    size_t array_size = 1;
    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_key_builtin_interfaces__msg__Time(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Field name: stepping
  {
    size_t array_size = 1;
    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = ros_gz_interfaces__msg__WorldStatistics;
    is_plain =
      (
      offsetof(DataType, stepping) +
      last_member_size
      ) == ret_val;
  }
  return ret_val;
}


static bool _WorldStatistics__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const ros_gz_interfaces__msg__WorldStatistics * ros_message = static_cast<const ros_gz_interfaces__msg__WorldStatistics *>(untyped_ros_message);
  (void)ros_message;
  return cdr_serialize_ros_gz_interfaces__msg__WorldStatistics(ros_message, cdr);
}

static bool _WorldStatistics__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  ros_gz_interfaces__msg__WorldStatistics * ros_message = static_cast<ros_gz_interfaces__msg__WorldStatistics *>(untyped_ros_message);
  (void)ros_message;
  return cdr_deserialize_ros_gz_interfaces__msg__WorldStatistics(cdr, ros_message);
}

static uint32_t _WorldStatistics__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_ros_gz_interfaces__msg__WorldStatistics(
      untyped_ros_message, 0));
}

static size_t _WorldStatistics__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_ros_gz_interfaces__msg__WorldStatistics(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_WorldStatistics = {
  "ros_gz_interfaces::msg",
  "WorldStatistics",
  _WorldStatistics__cdr_serialize,
  _WorldStatistics__cdr_deserialize,
  _WorldStatistics__get_serialized_size,
  _WorldStatistics__max_serialized_size,
  nullptr
};

static rosidl_message_type_support_t _WorldStatistics__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_WorldStatistics,
  get_message_typesupport_handle_function,
  &ros_gz_interfaces__msg__WorldStatistics__get_type_hash,
  &ros_gz_interfaces__msg__WorldStatistics__get_type_description,
  &ros_gz_interfaces__msg__WorldStatistics__get_type_description_sources,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, ros_gz_interfaces, msg, WorldStatistics)() {
  return &_WorldStatistics__type_support;
}

#if defined(__cplusplus)
}
#endif
