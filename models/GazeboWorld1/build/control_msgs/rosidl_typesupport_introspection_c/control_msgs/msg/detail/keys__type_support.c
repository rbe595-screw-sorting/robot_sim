// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from control_msgs:msg/Keys.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "control_msgs/msg/detail/keys__rosidl_typesupport_introspection_c.h"
#include "control_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "control_msgs/msg/detail/keys__functions.h"
#include "control_msgs/msg/detail/keys__struct.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/header.h"
// Member `header`
#include "std_msgs/msg/detail/header__rosidl_typesupport_introspection_c.h"
// Member `keys`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  control_msgs__msg__Keys__init(message_memory);
}

void control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_fini_function(void * message_memory)
{
  control_msgs__msg__Keys__fini(message_memory);
}

size_t control_msgs__msg__Keys__rosidl_typesupport_introspection_c__size_function__Keys__keys(
  const void * untyped_member)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return member->size;
}

const void * control_msgs__msg__Keys__rosidl_typesupport_introspection_c__get_const_function__Keys__keys(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__String__Sequence * member =
    (const rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void * control_msgs__msg__Keys__rosidl_typesupport_introspection_c__get_function__Keys__keys(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  return &member->data[index];
}

void control_msgs__msg__Keys__rosidl_typesupport_introspection_c__fetch_function__Keys__keys(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const rosidl_runtime_c__String * item =
    ((const rosidl_runtime_c__String *)
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__get_const_function__Keys__keys(untyped_member, index));
  rosidl_runtime_c__String * value =
    (rosidl_runtime_c__String *)(untyped_value);
  *value = *item;
}

void control_msgs__msg__Keys__rosidl_typesupport_introspection_c__assign_function__Keys__keys(
  void * untyped_member, size_t index, const void * untyped_value)
{
  rosidl_runtime_c__String * item =
    ((rosidl_runtime_c__String *)
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__get_function__Keys__keys(untyped_member, index));
  const rosidl_runtime_c__String * value =
    (const rosidl_runtime_c__String *)(untyped_value);
  *item = *value;
}

bool control_msgs__msg__Keys__rosidl_typesupport_introspection_c__resize_function__Keys__keys(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__String__Sequence * member =
    (rosidl_runtime_c__String__Sequence *)(untyped_member);
  rosidl_runtime_c__String__Sequence__fini(member);
  return rosidl_runtime_c__String__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_member_array[2] = {
  {
    "header",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(control_msgs__msg__Keys, header),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "keys",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is key
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(control_msgs__msg__Keys, keys),  // bytes offset in struct
    NULL,  // default value
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__size_function__Keys__keys,  // size() function pointer
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__get_const_function__Keys__keys,  // get_const(index) function pointer
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__get_function__Keys__keys,  // get(index) function pointer
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__fetch_function__Keys__keys,  // fetch(index, &value) function pointer
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__assign_function__Keys__keys,  // assign(index, value) function pointer
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__resize_function__Keys__keys  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_members = {
  "control_msgs__msg",  // message namespace
  "Keys",  // message name
  2,  // number of fields
  sizeof(control_msgs__msg__Keys),
  false,  // has_any_key_member_
  control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_member_array,  // message members
  control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_init_function,  // function to initialize message memory (memory has to be allocated)
  control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_type_support_handle = {
  0,
  &control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_members,
  get_message_typesupport_handle_function,
  &control_msgs__msg__Keys__get_type_hash,
  &control_msgs__msg__Keys__get_type_description,
  &control_msgs__msg__Keys__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_control_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, control_msgs, msg, Keys)() {
  control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, std_msgs, msg, Header)();
  if (!control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_type_support_handle.typesupport_identifier) {
    control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &control_msgs__msg__Keys__rosidl_typesupport_introspection_c__Keys_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
