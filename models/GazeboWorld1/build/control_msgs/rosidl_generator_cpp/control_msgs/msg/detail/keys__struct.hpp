// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from control_msgs:msg/Keys.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "control_msgs/msg/keys.hpp"


#ifndef CONTROL_MSGS__MSG__DETAIL__KEYS__STRUCT_HPP_
#define CONTROL_MSGS__MSG__DETAIL__KEYS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__control_msgs__msg__Keys __attribute__((deprecated))
#else
# define DEPRECATED__control_msgs__msg__Keys __declspec(deprecated)
#endif

namespace control_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct Keys_
{
  using Type = Keys_<ContainerAllocator>;

  explicit Keys_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_init)
  {
    (void)_init;
  }

  explicit Keys_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : header(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _header_type =
    std_msgs::msg::Header_<ContainerAllocator>;
  _header_type header;
  using _keys_type =
    std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>>;
  _keys_type keys;

  // setters for named parameter idiom
  Type & set__header(
    const std_msgs::msg::Header_<ContainerAllocator> & _arg)
  {
    this->header = _arg;
    return *this;
  }
  Type & set__keys(
    const std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>> & _arg)
  {
    this->keys = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    control_msgs::msg::Keys_<ContainerAllocator> *;
  using ConstRawPtr =
    const control_msgs::msg::Keys_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<control_msgs::msg::Keys_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<control_msgs::msg::Keys_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      control_msgs::msg::Keys_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<control_msgs::msg::Keys_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      control_msgs::msg::Keys_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<control_msgs::msg::Keys_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<control_msgs::msg::Keys_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<control_msgs::msg::Keys_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__control_msgs__msg__Keys
    std::shared_ptr<control_msgs::msg::Keys_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__control_msgs__msg__Keys
    std::shared_ptr<control_msgs::msg::Keys_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Keys_ & other) const
  {
    if (this->header != other.header) {
      return false;
    }
    if (this->keys != other.keys) {
      return false;
    }
    return true;
  }
  bool operator!=(const Keys_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Keys_

// alias to use template instance with default allocator
using Keys =
  control_msgs::msg::Keys_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace control_msgs

#endif  // CONTROL_MSGS__MSG__DETAIL__KEYS__STRUCT_HPP_
