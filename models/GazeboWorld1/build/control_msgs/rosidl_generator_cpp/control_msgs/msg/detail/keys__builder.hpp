// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from control_msgs:msg/Keys.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/keys.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__KEYS__BUILDER_HPP_
#define CONTROL_MSGS__MSG__DETAIL__KEYS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "control_msgs/msg/detail/keys__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace control_msgs
{

namespace msg
{

namespace builder
{

class Init_Keys_keys
{
public:
  explicit Init_Keys_keys(::control_msgs::msg::Keys & msg)
  : msg_(msg)
  {}
  ::control_msgs::msg::Keys keys(::control_msgs::msg::Keys::_keys_type arg)
  {
    msg_.keys = std::move(arg);
    return std::move(msg_);
  }

private:
  ::control_msgs::msg::Keys msg_;
};

class Init_Keys_header
{
public:
  Init_Keys_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Keys_keys header(::control_msgs::msg::Keys::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_Keys_keys(msg_);
  }

private:
  ::control_msgs::msg::Keys msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::control_msgs::msg::Keys>()
{
  return control_msgs::msg::builder::Init_Keys_header();
}

}  // namespace control_msgs

#endif  // CONTROL_MSGS__MSG__DETAIL__KEYS__BUILDER_HPP_
