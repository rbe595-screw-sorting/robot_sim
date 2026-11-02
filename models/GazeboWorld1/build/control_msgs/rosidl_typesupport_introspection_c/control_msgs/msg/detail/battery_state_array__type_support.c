// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from control_msgs:msg/BatteryStateArray.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "control_msgs/msg/detail/battery_state_array__rosidl_typesupport_introspection_c.h"
#include "control_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "control_msgs/msg/detail/battery_state_array__functions.h"
#include "control_msgs/msg/detail/battery_state_array__struct.h"


// Include directives for member types
// Member `battery_states`
#include "sensor_msgs/msg/battery_state.h"
// Member `battery_states`
#include "sensor_msgs/msg/detail/battery_state__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  control_msgs__msg__BatteryStateArray__init(message_memory);
}

void control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_fini_function(void * message_memory)
{
  control_msgs__msg__BatteryStateArray__fini(message_memory);
}

size_t control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__size_function__BatteryStateArray__battery_states(
  const void * untyped_member)
{
  const sensor_msgs__msg__BatteryState__Sequence * member =
    (const sensor_msgs__msg__BatteryState__Sequence *)(untyped_member);
  return member->size;
}

const void * control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__get_const_function__BatteryStateArray__battery_states(
  const void * untyped_member, size_t index)
{
  const sensor_msgs__msg__BatteryState__Sequence * member =
    (const sensor_msgs__msg__BatteryState__Sequence *)(untyped_member);
  return &member->data[index];
}

void * control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__get_function__BatteryStateArray__battery_states(
  void * untyped_member, size_t index)
{
  sensor_msgs__msg__BatteryState__Sequence * member =
    (sensor_msgs__msg__BatteryState__Sequence *)(untyped_member);
  return &member->data[index];
}

void control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__fetch_function__BatteryStateArray__battery_states(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const sensor_msgs__msg__BatteryState * item =
    ((const sensor_msgs__msg__BatteryState *)
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__get_const_function__BatteryStateArray__battery_states(untyped_member, index));
  sensor_msgs__msg__BatteryState * value =
    (sensor_msgs__msg__BatteryState *)(untyped_value);
  *value = *item;
}

void control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__assign_function__BatteryStateArray__battery_states(
  void * untyped_member, size_t index, const void * untyped_value)
{
  sensor_msgs__msg__BatteryState * item =
    ((sensor_msgs__msg__BatteryState *)
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__get_function__BatteryStateArray__battery_states(untyped_member, index));
  const sensor_msgs__msg__BatteryState * value =
    (const sensor_msgs__msg__BatteryState *)(untyped_value);
  *item = *value;
}

bool control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__resize_function__BatteryStateArray__battery_states(
  void * untyped_member, size_t size)
{
  sensor_msgs__msg__BatteryState__Sequence * member =
    (sensor_msgs__msg__BatteryState__Sequence *)(untyped_member);
  sensor_msgs__msg__BatteryState__Sequence__fini(member);
  return sensor_msgs__msg__BatteryState__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_member_array[1] = {
  {
    "battery_states",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(control_msgs__msg__BatteryStateArray, battery_states),  // bytes offset in struct
    NULL,  // default value
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__size_function__BatteryStateArray__battery_states,  // size() function pointer
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__get_const_function__BatteryStateArray__battery_states,  // get_const(index) function pointer
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__get_function__BatteryStateArray__battery_states,  // get(index) function pointer
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__fetch_function__BatteryStateArray__battery_states,  // fetch(index, &value) function pointer
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__assign_function__BatteryStateArray__battery_states,  // assign(index, value) function pointer
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__resize_function__BatteryStateArray__battery_states  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_members = {
  "control_msgs__msg",  // message namespace
  "BatteryStateArray",  // message name
  1,  // number of fields
  sizeof(control_msgs__msg__BatteryStateArray),
  false,  // has_any_key_member_
  control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_member_array,  // message members
  control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_init_function,  // function to initialize message memory (memory has to be allocated)
  control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_type_support_handle = {
  0,
  &control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_members,
  get_message_typesupport_handle_function,
  &control_msgs__msg__BatteryStateArray__get_type_hash,
  &control_msgs__msg__BatteryStateArray__get_type_description,
  &control_msgs__msg__BatteryStateArray__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_control_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, control_msgs, msg, BatteryStateArray)() {
  control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, sensor_msgs, msg, BatteryState)();
  if (!control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_type_support_handle.typesupport_identifier) {
    control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &control_msgs__msg__BatteryStateArray__rosidl_typesupport_introspection_c__BatteryStateArray_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
