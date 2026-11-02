// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ros_gz_interfaces:msg/AirSpeed.idl
// generated code does not contain a copyright notice
#include "ros_gz_interfaces/msg/detail/air_speed__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/detail/header__functions.h"

bool
ros_gz_interfaces__msg__AirSpeed__init(ros_gz_interfaces__msg__AirSpeed * msg)
{
  if (!msg) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__init(&msg->header)) {
    ros_gz_interfaces__msg__AirSpeed__fini(msg);
    return false;
  }
  // diff_pressure
  // temperature
  return true;
}

void
ros_gz_interfaces__msg__AirSpeed__fini(ros_gz_interfaces__msg__AirSpeed * msg)
{
  if (!msg) {
    return;
  }
  // header
  std_msgs__msg__Header__fini(&msg->header);
  // diff_pressure
  // temperature
}

bool
ros_gz_interfaces__msg__AirSpeed__are_equal(const ros_gz_interfaces__msg__AirSpeed * lhs, const ros_gz_interfaces__msg__AirSpeed * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__are_equal(
      &(lhs->header), &(rhs->header)))
  {
    return false;
  }
  // diff_pressure
  if (lhs->diff_pressure != rhs->diff_pressure) {
    return false;
  }
  // temperature
  if (lhs->temperature != rhs->temperature) {
    return false;
  }
  return true;
}

bool
ros_gz_interfaces__msg__AirSpeed__copy(
  const ros_gz_interfaces__msg__AirSpeed * input,
  ros_gz_interfaces__msg__AirSpeed * output)
{
  if (!input || !output) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__copy(
      &(input->header), &(output->header)))
  {
    return false;
  }
  // diff_pressure
  output->diff_pressure = input->diff_pressure;
  // temperature
  output->temperature = input->temperature;
  return true;
}

ros_gz_interfaces__msg__AirSpeed *
ros_gz_interfaces__msg__AirSpeed__create(void)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ros_gz_interfaces__msg__AirSpeed * msg = (ros_gz_interfaces__msg__AirSpeed *)allocator.allocate(sizeof(ros_gz_interfaces__msg__AirSpeed), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ros_gz_interfaces__msg__AirSpeed));
  bool success = ros_gz_interfaces__msg__AirSpeed__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ros_gz_interfaces__msg__AirSpeed__destroy(ros_gz_interfaces__msg__AirSpeed * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ros_gz_interfaces__msg__AirSpeed__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ros_gz_interfaces__msg__AirSpeed__Sequence__init(ros_gz_interfaces__msg__AirSpeed__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ros_gz_interfaces__msg__AirSpeed * data = NULL;

  if (size) {
    if (size > SIZE_MAX / sizeof(ros_gz_interfaces__msg__AirSpeed)) {
      return false;
    }
    data = (ros_gz_interfaces__msg__AirSpeed *)allocator.zero_allocate(size, sizeof(ros_gz_interfaces__msg__AirSpeed), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ros_gz_interfaces__msg__AirSpeed__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ros_gz_interfaces__msg__AirSpeed__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
ros_gz_interfaces__msg__AirSpeed__Sequence__fini(ros_gz_interfaces__msg__AirSpeed__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      ros_gz_interfaces__msg__AirSpeed__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

ros_gz_interfaces__msg__AirSpeed__Sequence *
ros_gz_interfaces__msg__AirSpeed__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ros_gz_interfaces__msg__AirSpeed__Sequence * array = (ros_gz_interfaces__msg__AirSpeed__Sequence *)allocator.allocate(sizeof(ros_gz_interfaces__msg__AirSpeed__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ros_gz_interfaces__msg__AirSpeed__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ros_gz_interfaces__msg__AirSpeed__Sequence__destroy(ros_gz_interfaces__msg__AirSpeed__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ros_gz_interfaces__msg__AirSpeed__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ros_gz_interfaces__msg__AirSpeed__Sequence__are_equal(const ros_gz_interfaces__msg__AirSpeed__Sequence * lhs, const ros_gz_interfaces__msg__AirSpeed__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ros_gz_interfaces__msg__AirSpeed__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ros_gz_interfaces__msg__AirSpeed__Sequence__copy(
  const ros_gz_interfaces__msg__AirSpeed__Sequence * input,
  ros_gz_interfaces__msg__AirSpeed__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    if (input->size > SIZE_MAX / sizeof(ros_gz_interfaces__msg__AirSpeed)) {
      return false;
    }
    const size_t allocation_size =
      input->size * sizeof(ros_gz_interfaces__msg__AirSpeed);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ros_gz_interfaces__msg__AirSpeed * data =
      (ros_gz_interfaces__msg__AirSpeed *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ros_gz_interfaces__msg__AirSpeed__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ros_gz_interfaces__msg__AirSpeed__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ros_gz_interfaces__msg__AirSpeed__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
