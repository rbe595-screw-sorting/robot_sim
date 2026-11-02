// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from control_msgs:msg/JointCommand.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/joint_command.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__JOINT_COMMAND__BUILDER_HPP_
#define CONTROL_MSGS__MSG__DETAIL__JOINT_COMMAND__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "control_msgs/msg/detail/joint_command__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace control_msgs
{

namespace msg
{

namespace builder
{

class Init_JointCommand_values
{
public:
  explicit Init_JointCommand_values(::control_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  ::control_msgs::msg::JointCommand values(::control_msgs::msg::JointCommand::_values_type arg)
  {
    msg_.values = std::move(arg);
    return std::move(msg_);
  }

private:
  ::control_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_interface_name
{
public:
  explicit Init_JointCommand_interface_name(::control_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  Init_JointCommand_values interface_name(::control_msgs::msg::JointCommand::_interface_name_type arg)
  {
    msg_.interface_name = std::move(arg);
    return Init_JointCommand_values(msg_);
  }

private:
  ::control_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_joint_names
{
public:
  explicit Init_JointCommand_joint_names(::control_msgs::msg::JointCommand & msg)
  : msg_(msg)
  {}
  Init_JointCommand_interface_name joint_names(::control_msgs::msg::JointCommand::_joint_names_type arg)
  {
    msg_.joint_names = std::move(arg);
    return Init_JointCommand_interface_name(msg_);
  }

private:
  ::control_msgs::msg::JointCommand msg_;
};

class Init_JointCommand_header
{
public:
  Init_JointCommand_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_JointCommand_joint_names header(::control_msgs::msg::JointCommand::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_JointCommand_joint_names(msg_);
  }

private:
  ::control_msgs::msg::JointCommand msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::control_msgs::msg::JointCommand>()
{
  return control_msgs::msg::builder::Init_JointCommand_header();
}

}  // namespace control_msgs

#endif  // CONTROL_MSGS__MSG__DETAIL__JOINT_COMMAND__BUILDER_HPP_
