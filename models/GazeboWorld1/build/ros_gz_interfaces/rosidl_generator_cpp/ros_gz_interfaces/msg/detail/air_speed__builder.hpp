// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ros_gz_interfaces:msg/AirSpeed.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "ros_gz_interfaces/msg/air_speed.hpp"


#ifndef ROS_GZ_INTERFACES__MSG__DETAIL__AIR_SPEED__BUILDER_HPP_
#define ROS_GZ_INTERFACES__MSG__DETAIL__AIR_SPEED__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ros_gz_interfaces/msg/detail/air_speed__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ros_gz_interfaces
{

namespace msg
{

namespace builder
{

class Init_AirSpeed_temperature
{
public:
  explicit Init_AirSpeed_temperature(::ros_gz_interfaces::msg::AirSpeed & msg)
  : msg_(msg)
  {}
  ::ros_gz_interfaces::msg::AirSpeed temperature(::ros_gz_interfaces::msg::AirSpeed::_temperature_type arg)
  {
    msg_.temperature = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ros_gz_interfaces::msg::AirSpeed msg_;
};

class Init_AirSpeed_diff_pressure
{
public:
  explicit Init_AirSpeed_diff_pressure(::ros_gz_interfaces::msg::AirSpeed & msg)
  : msg_(msg)
  {}
  Init_AirSpeed_temperature diff_pressure(::ros_gz_interfaces::msg::AirSpeed::_diff_pressure_type arg)
  {
    msg_.diff_pressure = std::move(arg);
    return Init_AirSpeed_temperature(msg_);
  }

private:
  ::ros_gz_interfaces::msg::AirSpeed msg_;
};

class Init_AirSpeed_header
{
public:
  Init_AirSpeed_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AirSpeed_diff_pressure header(::ros_gz_interfaces::msg::AirSpeed::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_AirSpeed_diff_pressure(msg_);
  }

private:
  ::ros_gz_interfaces::msg::AirSpeed msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ros_gz_interfaces::msg::AirSpeed>()
{
  return ros_gz_interfaces::msg::builder::Init_AirSpeed_header();
}

}  // namespace ros_gz_interfaces

#endif  // ROS_GZ_INTERFACES__MSG__DETAIL__AIR_SPEED__BUILDER_HPP_
