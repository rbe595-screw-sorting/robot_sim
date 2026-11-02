// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from control_msgs:msg/VDA5050SafetyState.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/vda5050_safety_state.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__STRUCT_HPP_
#define CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__control_msgs__msg__VDA5050SafetyState __attribute__((deprecated))
#else
# define DEPRECATED__control_msgs__msg__VDA5050SafetyState __declspec(deprecated)
#endif

namespace control_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct VDA5050SafetyState_
{
  using Type = VDA5050SafetyState_<ContainerAllocator>;

  explicit VDA5050SafetyState_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->e_stop = "";
      this->field_violation = false;
    }
  }

  explicit VDA5050SafetyState_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : e_stop(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->e_stop = "";
      this->field_violation = false;
    }
  }

  // field types and members
  using _e_stop_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _e_stop_type e_stop;
  using _field_violation_type =
    bool;
  _field_violation_type field_violation;

  // setters for named parameter idiom
  Type & set__e_stop(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->e_stop = _arg;
    return *this;
  }
  Type & set__field_violation(
    const bool & _arg)
  {
    this->field_violation = _arg;
    return *this;
  }

  // constant declarations
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> AUTO_ACK;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> MANUAL;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> REMOTE;
  static const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> NONE;

  // pointer types
  using RawPtr =
    control_msgs::msg::VDA5050SafetyState_<ContainerAllocator> *;
  using ConstRawPtr =
    const control_msgs::msg::VDA5050SafetyState_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      control_msgs::msg::VDA5050SafetyState_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      control_msgs::msg::VDA5050SafetyState_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__control_msgs__msg__VDA5050SafetyState
    std::shared_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__control_msgs__msg__VDA5050SafetyState
    std::shared_ptr<control_msgs::msg::VDA5050SafetyState_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const VDA5050SafetyState_ & other) const
  {
    if (this->e_stop != other.e_stop) {
      return false;
    }
    if (this->field_violation != other.field_violation) {
      return false;
    }
    return true;
  }
  bool operator!=(const VDA5050SafetyState_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct VDA5050SafetyState_

// alias to use template instance with default allocator
using VDA5050SafetyState =
  control_msgs::msg::VDA5050SafetyState_<std::allocator<void>>;

// constant definitions
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
VDA5050SafetyState_<ContainerAllocator>::AUTO_ACK = "autoAck";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
VDA5050SafetyState_<ContainerAllocator>::MANUAL = "manual";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
VDA5050SafetyState_<ContainerAllocator>::REMOTE = "remote";
template<typename ContainerAllocator>
const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>
VDA5050SafetyState_<ContainerAllocator>::NONE = "none";

}  // namespace msg

}  // namespace control_msgs

#endif  // CONTROL_MSGS__MSG__DETAIL__VDA5050_SAFETY_STATE__STRUCT_HPP_
