// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from control_msgs:msg/BatteryStateArray.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/battery_state_array.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__BATTERY_STATE_ARRAY__TRAITS_HPP_
#define CONTROL_MSGS__MSG__DETAIL__BATTERY_STATE_ARRAY__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "control_msgs/msg/detail/battery_state_array__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'battery_states'
#include "sensor_msgs/msg/detail/battery_state__traits.hpp"

namespace control_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const BatteryStateArray & msg,
  std::ostream & out)
{
  out << "{";
  // member: battery_states
  {
    if (msg.battery_states.size() == 0) {
      out << "battery_states: []";
    } else {
      out << "battery_states: [";
      size_t pending_items = msg.battery_states.size();
      for (auto item : msg.battery_states) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BatteryStateArray & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: battery_states
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.battery_states.size() == 0) {
      out << "battery_states: []\n";
    } else {
      out << "battery_states:\n";
      for (auto item : msg.battery_states) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BatteryStateArray & msg, bool use_flow_style = false)
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
  const control_msgs::msg::BatteryStateArray & msg,
  std::ostream & out, size_t indentation = 0)
{
  control_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use control_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const control_msgs::msg::BatteryStateArray & msg)
{
  return control_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<control_msgs::msg::BatteryStateArray>()
{
  return "control_msgs::msg::BatteryStateArray";
}

template<>
inline const char * name<control_msgs::msg::BatteryStateArray>()
{
  return "control_msgs/msg/BatteryStateArray";
}

template<>
struct has_fixed_size<control_msgs::msg::BatteryStateArray>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<control_msgs::msg::BatteryStateArray>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<control_msgs::msg::BatteryStateArray>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // CONTROL_MSGS__MSG__DETAIL__BATTERY_STATE_ARRAY__TRAITS_HPP_
