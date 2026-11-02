// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from control_msgs:msg/Float64Values.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/float64_values.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__FLOAT64_VALUES__BUILDER_HPP_
#define CONTROL_MSGS__MSG__DETAIL__FLOAT64_VALUES__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "control_msgs/msg/detail/float64_values__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace control_msgs
{

namespace msg
{

namespace builder
{

class Init_Float64Values_values
{
public:
  explicit Init_Float64Values_values(::control_msgs::msg::Float64Values & msg)
  : msg_(msg)
  {}
  ::control_msgs::msg::Float64Values values(::control_msgs::msg::Float64Values::_values_type arg)
  {
    msg_.values = std::move(arg);
    return std::move(msg_);
  }

private:
  ::control_msgs::msg::Float64Values msg_;
};

class Init_Float64Values_header
{
public:
  Init_Float64Values_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Float64Values_values header(::control_msgs::msg::Float64Values::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_Float64Values_values(msg_);
  }

private:
  ::control_msgs::msg::Float64Values msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::control_msgs::msg::Float64Values>()
{
  return control_msgs::msg::builder::Init_Float64Values_header();
}

}  // namespace control_msgs

#endif  // CONTROL_MSGS__MSG__DETAIL__FLOAT64_VALUES__BUILDER_HPP_
