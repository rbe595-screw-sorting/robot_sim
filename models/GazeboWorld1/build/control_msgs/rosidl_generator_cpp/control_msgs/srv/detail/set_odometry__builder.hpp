// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from control_msgs:srv/SetOdometry.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/srv/set_odometry.hpp"


#ifndef CONTROL_MSGS__SRV__DETAIL__SET_ODOMETRY__BUILDER_HPP_
#define CONTROL_MSGS__SRV__DETAIL__SET_ODOMETRY__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "control_msgs/srv/detail/set_odometry__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace control_msgs
{

namespace srv
{

namespace builder
{

class Init_SetOdometry_Request_yaw
{
public:
  explicit Init_SetOdometry_Request_yaw(::control_msgs::srv::SetOdometry_Request & msg)
  : msg_(msg)
  {}
  ::control_msgs::srv::SetOdometry_Request yaw(::control_msgs::srv::SetOdometry_Request::_yaw_type arg)
  {
    msg_.yaw = std::move(arg);
    return std::move(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Request msg_;
};

class Init_SetOdometry_Request_pitch
{
public:
  explicit Init_SetOdometry_Request_pitch(::control_msgs::srv::SetOdometry_Request & msg)
  : msg_(msg)
  {}
  Init_SetOdometry_Request_yaw pitch(::control_msgs::srv::SetOdometry_Request::_pitch_type arg)
  {
    msg_.pitch = std::move(arg);
    return Init_SetOdometry_Request_yaw(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Request msg_;
};

class Init_SetOdometry_Request_roll
{
public:
  explicit Init_SetOdometry_Request_roll(::control_msgs::srv::SetOdometry_Request & msg)
  : msg_(msg)
  {}
  Init_SetOdometry_Request_pitch roll(::control_msgs::srv::SetOdometry_Request::_roll_type arg)
  {
    msg_.roll = std::move(arg);
    return Init_SetOdometry_Request_pitch(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Request msg_;
};

class Init_SetOdometry_Request_z
{
public:
  explicit Init_SetOdometry_Request_z(::control_msgs::srv::SetOdometry_Request & msg)
  : msg_(msg)
  {}
  Init_SetOdometry_Request_roll z(::control_msgs::srv::SetOdometry_Request::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_SetOdometry_Request_roll(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Request msg_;
};

class Init_SetOdometry_Request_y
{
public:
  explicit Init_SetOdometry_Request_y(::control_msgs::srv::SetOdometry_Request & msg)
  : msg_(msg)
  {}
  Init_SetOdometry_Request_z y(::control_msgs::srv::SetOdometry_Request::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_SetOdometry_Request_z(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Request msg_;
};

class Init_SetOdometry_Request_x
{
public:
  Init_SetOdometry_Request_x()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetOdometry_Request_y x(::control_msgs::srv::SetOdometry_Request::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_SetOdometry_Request_y(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::control_msgs::srv::SetOdometry_Request>()
{
  return control_msgs::srv::builder::Init_SetOdometry_Request_x();
}

}  // namespace control_msgs


namespace control_msgs
{

namespace srv
{

namespace builder
{

class Init_SetOdometry_Response_message
{
public:
  explicit Init_SetOdometry_Response_message(::control_msgs::srv::SetOdometry_Response & msg)
  : msg_(msg)
  {}
  ::control_msgs::srv::SetOdometry_Response message(::control_msgs::srv::SetOdometry_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Response msg_;
};

class Init_SetOdometry_Response_success
{
public:
  Init_SetOdometry_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetOdometry_Response_message success(::control_msgs::srv::SetOdometry_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetOdometry_Response_message(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::control_msgs::srv::SetOdometry_Response>()
{
  return control_msgs::srv::builder::Init_SetOdometry_Response_success();
}

}  // namespace control_msgs


namespace control_msgs
{

namespace srv
{

namespace builder
{

class Init_SetOdometry_Event_response
{
public:
  explicit Init_SetOdometry_Event_response(::control_msgs::srv::SetOdometry_Event & msg)
  : msg_(msg)
  {}
  ::control_msgs::srv::SetOdometry_Event response(::control_msgs::srv::SetOdometry_Event::_response_type arg)
  {
    msg_.response = std::move(arg);
    return std::move(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Event msg_;
};

class Init_SetOdometry_Event_request
{
public:
  explicit Init_SetOdometry_Event_request(::control_msgs::srv::SetOdometry_Event & msg)
  : msg_(msg)
  {}
  Init_SetOdometry_Event_response request(::control_msgs::srv::SetOdometry_Event::_request_type arg)
  {
    msg_.request = std::move(arg);
    return Init_SetOdometry_Event_response(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Event msg_;
};

class Init_SetOdometry_Event_info
{
public:
  Init_SetOdometry_Event_info()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetOdometry_Event_request info(::control_msgs::srv::SetOdometry_Event::_info_type arg)
  {
    msg_.info = std::move(arg);
    return Init_SetOdometry_Event_request(msg_);
  }

private:
  ::control_msgs::srv::SetOdometry_Event msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::control_msgs::srv::SetOdometry_Event>()
{
  return control_msgs::srv::builder::Init_SetOdometry_Event_info();
}

}  // namespace control_msgs

#endif  // CONTROL_MSGS__SRV__DETAIL__SET_ODOMETRY__BUILDER_HPP_
