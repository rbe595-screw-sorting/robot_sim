// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from control_msgs:msg/BatteryStateArray.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/battery_state_array.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__BATTERY_STATE_ARRAY__STRUCT_HPP_
#define CONTROL_MSGS__MSG__DETAIL__BATTERY_STATE_ARRAY__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'battery_states'
#include "sensor_msgs/msg/detail/battery_state__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__control_msgs__msg__BatteryStateArray __attribute__((deprecated))
#else
# define DEPRECATED__control_msgs__msg__BatteryStateArray __declspec(deprecated)
#endif

namespace control_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct BatteryStateArray_
{
  using Type = BatteryStateArray_<ContainerAllocator>;

  explicit BatteryStateArray_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
  }

  explicit BatteryStateArray_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
    (void)_alloc;
  }

  // field types and members
  using _battery_states_type =
    std::vector<sensor_msgs::msg::BatteryState_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<sensor_msgs::msg::BatteryState_<ContainerAllocator>>>;
  _battery_states_type battery_states;

  // setters for named parameter idiom
  Type & set__battery_states(
    const std::vector<sensor_msgs::msg::BatteryState_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<sensor_msgs::msg::BatteryState_<ContainerAllocator>>> & _arg)
  {
    this->battery_states = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    control_msgs::msg::BatteryStateArray_<ContainerAllocator> *;
  using ConstRawPtr =
    const control_msgs::msg::BatteryStateArray_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      control_msgs::msg::BatteryStateArray_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      control_msgs::msg::BatteryStateArray_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__control_msgs__msg__BatteryStateArray
    std::shared_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__control_msgs__msg__BatteryStateArray
    std::shared_ptr<control_msgs::msg::BatteryStateArray_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const BatteryStateArray_ & other) const
  {
    if (this->battery_states != other.battery_states) {
      return false;
    }
    return true;
  }
  bool operator!=(const BatteryStateArray_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct BatteryStateArray_

// alias to use template instance with default allocator
using BatteryStateArray =
  control_msgs::msg::BatteryStateArray_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace control_msgs

#endif  // CONTROL_MSGS__MSG__DETAIL__BATTERY_STATE_ARRAY__STRUCT_HPP_
