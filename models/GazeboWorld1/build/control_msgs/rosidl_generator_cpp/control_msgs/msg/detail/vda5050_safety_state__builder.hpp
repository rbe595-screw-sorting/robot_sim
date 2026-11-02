// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from control_msgs:msg/VDA5050SafetyState.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/vda5050_safety_state.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__BUILDER_HPP_
#define CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "control_msgs/msg/detail/vda5050_safety_state__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace control_msgs
{

namespace msg
{

namespace builder
{

class Init_VDA5050SafetyState_field_violation
{
public:
  explicit Init_VDA5050SafetyState_field_violation(::control_msgs::msg::VDA5050SafetyState & msg)
  : msg_(msg)
  {}
  ::control_msgs::msg::VDA5050SafetyState field_violation(::control_msgs::msg::VDA5050SafetyState::_field_violation_type arg)
  {
    msg_.field_violation = std::move(arg);
    return std::move(msg_);
  }

private:
  ::control_msgs::msg::VDA5050SafetyState msg_;
};

class Init_VDA5050SafetyState_e_stop
{
public:
  Init_VDA5050SafetyState_e_stop()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_VDA5050SafetyState_field_violation e_stop(::control_msgs::msg::VDA5050SafetyState::_e_stop_type arg)
  {
    msg_.e_stop = std::move(arg);
    return Init_VDA5050SafetyState_field_violation(msg_);
  }

private:
  ::control_msgs::msg::VDA5050SafetyState msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::control_msgs::msg::VDA5050SafetyState>()
{
  return control_msgs::msg::builder::Init_VDA5050SafetyState_e_stop();
}

}  // namespace control_msgs

#endif  // CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__BUILDER_HPP_
