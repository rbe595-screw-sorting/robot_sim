// generated from rosidl_typesupport_introspection_cpp/resource/idl__type_support.cpp.em
// with input from control_msgs:msg/VDA5050SafetyState.idl
// generated code does not contain a copyright notice

#include "array"
#include "cstddef"
#include "string"
#include "vector"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_interface/macros.h"
#include "control_msgs/msg/detail/vda5050_safety_state__functions.h"
#include "control_msgs/msg/detail/vda5050_safety_state__struct.hpp"
#include "rosidl_typesupport_introspection_cpp/field_types.hpp"
#include "rosidl_typesupport_introspection_cpp/identifier.hpp"
#include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
#include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace control_msgs
{

namespace msg
{

namespace rosidl_typesupport_introspection_cpp
{

void VDA5050SafetyState_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) control_msgs::msg::VDA5050SafetyState(_init);
}

void VDA5050SafetyState_fini_function(void * message_memory)
{
  auto typed_message = static_cast<control_msgs::msg::VDA5050SafetyState *>(message_memory);
  typed_message->~VDA5050SafetyState();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember VDA5050SafetyState_message_member_array[2] = {
  {
    "e_stop",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(control_msgs::msg::VDA5050SafetyState, e_stop),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr  // resize(index) function pointer
  },
  {
    "field_violation",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(control_msgs::msg::VDA5050SafetyState, field_violation),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr  // resize(index) function pointer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers VDA5050SafetyState_message_members = {
  "control_msgs::msg",  // message namespace
  "VDA5050SafetyState",  // message name
  2,  // number of fields
  sizeof(control_msgs::msg::VDA5050SafetyState),
  false,  // has_any_key_member_
  VDA5050SafetyState_message_member_array,  // message members
  VDA5050SafetyState_init_function,  // function to initialize message memory (memory has to be allocated)
  VDA5050SafetyState_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t VDA5050SafetyState_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &VDA5050SafetyState_message_members,
  get_message_typesupport_handle_function,
  &control_msgs__msg__VDA5050SafetyState__get_type_hash,
  &control_msgs__msg__VDA5050SafetyState__get_type_description,
  &control_msgs__msg__VDA5050SafetyState__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace msg

}  // namespace control_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<control_msgs::msg::VDA5050SafetyState>()
{
  return &::control_msgs::msg::rosidl_typesupport_introspection_cpp::VDA5050SafetyState_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, control_msgs, msg, VDA5050SafetyState)() {
  return &::control_msgs::msg::rosidl_typesupport_introspection_cpp::VDA5050SafetyState_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif
