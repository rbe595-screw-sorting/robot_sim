// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from control_msgs:msg/VDA5050SafetyState.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/vda5050_safety_state.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__TRAITS_HPP_
#define CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "control_msgs/msg/detail/vda5050_safety_state__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace control_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const VDA5050SafetyState & msg,
  std::ostream & out)
{
  out << "{";
  // member: e_stop
  {
    out << "e_stop: ";
    rosidl_generator_traits::value_to_yaml(msg.e_stop, out);
    out << ", ";
  }

  // member: field_violation
  {
    out << "field_violation: ";
    rosidl_generator_traits::value_to_yaml(msg.field_violation, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const VDA5050SafetyState & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: e_stop
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "e_stop: ";
    rosidl_generator_traits::value_to_yaml(msg.e_stop, out);
    out << "\n";
  }

  // member: field_violation
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "field_violation: ";
    rosidl_generator_traits::value_to_yaml(msg.field_violation, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const VDA5050SafetyState & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace control_msgs

namespace rosidl_generator_traits
{

[[deprecated("use control_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const control_msgs::msg::VDA5050SafetyState & msg,
  std::ostream & out, size_t indentation = 0)
{
  control_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use control_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const control_msgs::msg::VDA5050SafetyState & msg)
{
  return control_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<control_msgs::msg::VDA5050SafetyState>()
{
  return "control_msgs::msg::VDA5050SafetyState";
}

template<>
inline const char * name<control_msgs::msg::VDA5050SafetyState>()
{
  return "control_msgs/msg/VDA5050SafetyState";
}

template<>
struct has_fixed_size<control_msgs::msg::VDA5050SafetyState>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<control_msgs::msg::VDA5050SafetyState>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<control_msgs::msg::VDA5050SafetyState>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__TRAITS_HPP_
