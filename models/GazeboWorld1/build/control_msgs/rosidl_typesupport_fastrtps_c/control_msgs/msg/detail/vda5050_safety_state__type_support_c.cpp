// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from control_msgs:msg/VDA5050SafetyState.idl
// generated code does not contain a copyright notice
#include "control_msgs/msg/detail/vda5050_safety_state__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <cstddef>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/serialization_helpers.hpp"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "control_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "control_msgs/msg/detail/vda5050_safety_state__struct.h"
#include "control_msgs/msg/detail/vda5050_safety_state__functions.h"
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

#include "rosidl_runtime_c/string.h"  // e_stop
#include "rosidl_runtime_c/string_functions.h"  // e_stop

// forward declare type support functions


using _VDA5050SafetyState__ros_msg_type = control_msgs__msg__VDA5050SafetyState;


ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_control_msgs
bool cdr_serialize_control_msgs__msg__VDA5050SafetyState(
  const control_msgs__msg__VDA5050SafetyState * ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Field name: e_stop
  {
    const rosidl_runtime_c__String * str = &ros_message->e_stop;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  // Field name: field_violation
  {
    cdr << (ros_message->field_violation ? true : false);
  }

  return true;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_control_msgs
bool cdr_deserialize_control_msgs__msg__VDA5050SafetyState(
  eprosima::fastcdr::Cdr & cdr,
  control_msgs__msg__VDA5050SafetyState * ros_message)
{
  // Field name: e_stop
  {
    std::string tmp;
    cdr >> tmp;
    if (!ros_message->e_stop.data) {
      rosidl_runtime_c__String__init(&ros_message->e_stop);
    }
    bool succeeded = rosidl_runtime_c__String__assign(
      &ros_message->e_stop,
      tmp.c_str());
    if (!succeeded) {
      fprintf(stderr, "failed to assign string into field 'e_stop'\n");
      return false;
    }
  }

  // Field name: field_violation
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->field_violation = tmp ? true : false;
  }

  return true;
}  // NOLINT(readability/fn_size)


ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_control_msgs
size_t get_serialized_size_control_msgs__msg__VDA5050SafetyState(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _VDA5050SafetyState__ros_msg_type * ros_message = static_cast<const _VDA5050SafetyState__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Field name: e_stop
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->e_stop.size + 1);

  // Field name: field_violation
  {
    size_t item_size = sizeof(ros_message->field_violation);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}


ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_control_msgs
size_t max_serialized_size_control_msgs__msg__VDA5050SafetyState(
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

  // Field name: e_stop
  {
    size_t array_size = 1;
    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }

  // Field name: field_violation
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
    using DataType = control_msgs__msg__VDA5050SafetyState;
    is_plain =
      (
      offsetof(DataType, field_violation) +
      last_member_size
      ) == ret_val;
  }
  return ret_val;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_control_msgs
bool cdr_serialize_key_control_msgs__msg__VDA5050SafetyState(
  const control_msgs__msg__VDA5050SafetyState * ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Field name: e_stop
  {
    const rosidl_runtime_c__String * str = &ros_message->e_stop;
    if (str->capacity == 0 || str->capacity <= str->size) {
      fprintf(stderr, "string capacity not greater than size\n");
      return false;
    }
    if (str->data[str->size] != '\0') {
      fprintf(stderr, "string not null-terminated\n");
      return false;
    }
    cdr << str->data;
  }

  // Field name: field_violation
  {
    cdr << (ros_message->field_violation ? true : false);
  }

  return true;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_control_msgs
size_t get_serialized_size_key_control_msgs__msg__VDA5050SafetyState(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _VDA5050SafetyState__ros_msg_type * ros_message = static_cast<const _VDA5050SafetyState__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;

  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Field name: e_stop
  current_alignment += padding +
    eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
    (ros_message->e_stop.size + 1);

  // Field name: field_violation
  {
    size_t item_size = sizeof(ros_message->field_violation);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_control_msgs
size_t max_serialized_size_key_control_msgs__msg__VDA5050SafetyState(
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
  // Field name: e_stop
  {
    size_t array_size = 1;
    full_bounded = false;
    is_plain = false;
    for (size_t index = 0; index < array_size; ++index) {
      current_alignment += padding +
        eprosima::fastcdr::Cdr::alignment(current_alignment, padding) +
        1;
    }
  }

  // Field name: field_violation
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
    using DataType = control_msgs__msg__VDA5050SafetyState;
    is_plain =
      (
      offsetof(DataType, field_violation) +
      last_member_size
      ) == ret_val;
  }
  return ret_val;
}


static bool _VDA5050SafetyState__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const control_msgs__msg__VDA5050SafetyState * ros_message = static_cast<const control_msgs__msg__VDA5050SafetyState *>(untyped_ros_message);
  (void)ros_message;
  return cdr_serialize_control_msgs__msg__VDA5050SafetyState(ros_message, cdr);
}

static bool _VDA5050SafetyState__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  control_msgs__msg__VDA5050SafetyState * ros_message = static_cast<control_msgs__msg__VDA5050SafetyState *>(untyped_ros_message);
  (void)ros_message;
  return cdr_deserialize_control_msgs__msg__VDA5050SafetyState(cdr, ros_message);
}

static uint32_t _VDA5050SafetyState__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_control_msgs__msg__VDA5050SafetyState(
      untyped_ros_message, 0));
}

static size_t _VDA5050SafetyState__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_control_msgs__msg__VDA5050SafetyState(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_VDA5050SafetyState = {
  "control_msgs::msg",
  "VDA5050SafetyState",
  _VDA5050SafetyState__cdr_serialize,
  _VDA5050SafetyState__cdr_deserialize,
  _VDA5050SafetyState__get_serialized_size,
  _VDA5050SafetyState__max_serialized_size,
  nullptr
};

static rosidl_message_type_support_t _VDA5050SafetyState__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_VDA5050SafetyState,
  get_message_typesupport_handle_function,
  &control_msgs__msg__VDA5050SafetyState__get_type_hash,
  &control_msgs__msg__VDA5050SafetyState__get_type_description,
  &control_msgs__msg__VDA5050SafetyState__get_type_description_sources,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, control_msgs, msg, VDA5050SafetyState)() {
  return &_VDA5050SafetyState__type_support;
}

#if defined(__cplusplus)
}
#endif
