// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from control_msgs:srv/SetOdometry.idl
// generated code does not contain a copyright notice
#include "control_msgs/srv/detail/set_odometry__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"

bool
control_msgs__srv__SetOdometry_Request__init(control_msgs__srv__SetOdometry_Request * msg)
{
  if (!msg) {
    return false;
  }
  // x
  // y
  // z
  // roll
  // pitch
  // yaw
  return true;
}

void
control_msgs__srv__SetOdometry_Request__fini(control_msgs__srv__SetOdometry_Request * msg)
{
  if (!msg) {
    return;
  }
  // x
  // y
  // z
  // roll
  // pitch
  // yaw
}

bool
control_msgs__srv__SetOdometry_Request__are_equal(const control_msgs__srv__SetOdometry_Request * lhs, const control_msgs__srv__SetOdometry_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // x
  if (lhs->x != rhs->x) {
    return false;
  }
  // y
  if (lhs->y != rhs->y) {
    return false;
  }
  // z
  if (lhs->z != rhs->z) {
    return false;
  }
  // roll
  if (lhs->roll != rhs->roll) {
    return false;
  }
  // pitch
  if (lhs->pitch != rhs->pitch) {
    return false;
  }
  // yaw
  if (lhs->yaw != rhs->yaw) {
    return false;
  }
  return true;
}

bool
control_msgs__srv__SetOdometry_Request__copy(
  const control_msgs__srv__SetOdometry_Request * input,
  control_msgs__srv__SetOdometry_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // x
  output->x = input->x;
  // y
  output->y = input->y;
  // z
  output->z = input->z;
  // roll
  output->roll = input->roll;
  // pitch
  output->pitch = input->pitch;
  // yaw
  output->yaw = input->yaw;
  return true;
}

control_msgs__srv__SetOdometry_Request *
control_msgs__srv__SetOdometry_Request__create(void)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Request * msg = (control_msgs__srv__SetOdometry_Request *)allocator.allocate(sizeof(control_msgs__srv__SetOdometry_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(control_msgs__srv__SetOdometry_Request));
  bool success = control_msgs__srv__SetOdometry_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
control_msgs__srv__SetOdometry_Request__destroy(control_msgs__srv__SetOdometry_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    control_msgs__srv__SetOdometry_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
control_msgs__srv__SetOdometry_Request__Sequence__init(control_msgs__srv__SetOdometry_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Request * data = NULL;

  if (size) {
    if (size > SIZE_MAX / sizeof(control_msgs__srv__SetOdometry_Request)) {
      return false;
    }
    data = (control_msgs__srv__SetOdometry_Request *)allocator.zero_allocate(size, sizeof(control_msgs__srv__SetOdometry_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = control_msgs__srv__SetOdometry_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        control_msgs__srv__SetOdometry_Request__fini(&data[i - 1]);
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
control_msgs__srv__SetOdometry_Request__Sequence__fini(control_msgs__srv__SetOdometry_Request__Sequence * array)
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
      control_msgs__srv__SetOdometry_Request__fini(&array->data[i]);
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

control_msgs__srv__SetOdometry_Request__Sequence *
control_msgs__srv__SetOdometry_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Request__Sequence * array = (control_msgs__srv__SetOdometry_Request__Sequence *)allocator.allocate(sizeof(control_msgs__srv__SetOdometry_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = control_msgs__srv__SetOdometry_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
control_msgs__srv__SetOdometry_Request__Sequence__destroy(control_msgs__srv__SetOdometry_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    control_msgs__srv__SetOdometry_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
control_msgs__srv__SetOdometry_Request__Sequence__are_equal(const control_msgs__srv__SetOdometry_Request__Sequence * lhs, const control_msgs__srv__SetOdometry_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!control_msgs__srv__SetOdometry_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
control_msgs__srv__SetOdometry_Request__Sequence__copy(
  const control_msgs__srv__SetOdometry_Request__Sequence * input,
  control_msgs__srv__SetOdometry_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    if (input->size > SIZE_MAX / sizeof(control_msgs__srv__SetOdometry_Request)) {
      return false;
    }
    const size_t allocation_size =
      input->size * sizeof(control_msgs__srv__SetOdometry_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    control_msgs__srv__SetOdometry_Request * data =
      (control_msgs__srv__SetOdometry_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!control_msgs__srv__SetOdometry_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          control_msgs__srv__SetOdometry_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!control_msgs__srv__SetOdometry_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `message`
#include "rosidl_runtime_c/string_functions.h"

bool
control_msgs__srv__SetOdometry_Response__init(control_msgs__srv__SetOdometry_Response * msg)
{
  if (!msg) {
    return false;
  }
  // success
  // message
  if (!rosidl_runtime_c__String__init(&msg->message)) {
    control_msgs__srv__SetOdometry_Response__fini(msg);
    return false;
  }
  return true;
}

void
control_msgs__srv__SetOdometry_Response__fini(control_msgs__srv__SetOdometry_Response * msg)
{
  if (!msg) {
    return;
  }
  // success
  // message
  rosidl_runtime_c__String__fini(&msg->message);
}

bool
control_msgs__srv__SetOdometry_Response__are_equal(const control_msgs__srv__SetOdometry_Response * lhs, const control_msgs__srv__SetOdometry_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // success
  if (lhs->success != rhs->success) {
    return false;
  }
  // message
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->message), &(rhs->message)))
  {
    return false;
  }
  return true;
}

bool
control_msgs__srv__SetOdometry_Response__copy(
  const control_msgs__srv__SetOdometry_Response * input,
  control_msgs__srv__SetOdometry_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // success
  output->success = input->success;
  // message
  if (!rosidl_runtime_c__String__copy(
      &(input->message), &(output->message)))
  {
    return false;
  }
  return true;
}

control_msgs__srv__SetOdometry_Response *
control_msgs__srv__SetOdometry_Response__create(void)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Response * msg = (control_msgs__srv__SetOdometry_Response *)allocator.allocate(sizeof(control_msgs__srv__SetOdometry_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(control_msgs__srv__SetOdometry_Response));
  bool success = control_msgs__srv__SetOdometry_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
control_msgs__srv__SetOdometry_Response__destroy(control_msgs__srv__SetOdometry_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    control_msgs__srv__SetOdometry_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
control_msgs__srv__SetOdometry_Response__Sequence__init(control_msgs__srv__SetOdometry_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Response * data = NULL;

  if (size) {
    if (size > SIZE_MAX / sizeof(control_msgs__srv__SetOdometry_Response)) {
      return false;
    }
    data = (control_msgs__srv__SetOdometry_Response *)allocator.zero_allocate(size, sizeof(control_msgs__srv__SetOdometry_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = control_msgs__srv__SetOdometry_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        control_msgs__srv__SetOdometry_Response__fini(&data[i - 1]);
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
control_msgs__srv__SetOdometry_Response__Sequence__fini(control_msgs__srv__SetOdometry_Response__Sequence * array)
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
      control_msgs__srv__SetOdometry_Response__fini(&array->data[i]);
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

control_msgs__srv__SetOdometry_Response__Sequence *
control_msgs__srv__SetOdometry_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Response__Sequence * array = (control_msgs__srv__SetOdometry_Response__Sequence *)allocator.allocate(sizeof(control_msgs__srv__SetOdometry_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = control_msgs__srv__SetOdometry_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
control_msgs__srv__SetOdometry_Response__Sequence__destroy(control_msgs__srv__SetOdometry_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    control_msgs__srv__SetOdometry_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
control_msgs__srv__SetOdometry_Response__Sequence__are_equal(const control_msgs__srv__SetOdometry_Response__Sequence * lhs, const control_msgs__srv__SetOdometry_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!control_msgs__srv__SetOdometry_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
control_msgs__srv__SetOdometry_Response__Sequence__copy(
  const control_msgs__srv__SetOdometry_Response__Sequence * input,
  control_msgs__srv__SetOdometry_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    if (input->size > SIZE_MAX / sizeof(control_msgs__srv__SetOdometry_Response)) {
      return false;
    }
    const size_t allocation_size =
      input->size * sizeof(control_msgs__srv__SetOdometry_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    control_msgs__srv__SetOdometry_Response * data =
      (control_msgs__srv__SetOdometry_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!control_msgs__srv__SetOdometry_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          control_msgs__srv__SetOdometry_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!control_msgs__srv__SetOdometry_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `info`
#include "service_msgs/msg/detail/service_event_info__functions.h"
// Member `request`
// Member `response`
// already included above
// #include "control_msgs/srv/detail/set_odometry__functions.h"

bool
control_msgs__srv__SetOdometry_Event__init(control_msgs__srv__SetOdometry_Event * msg)
{
  if (!msg) {
    return false;
  }
  // info
  if (!service_msgs__msg__ServiceEventInfo__init(&msg->info)) {
    control_msgs__srv__SetOdometry_Event__fini(msg);
    return false;
  }
  // request
  if (!control_msgs__srv__SetOdometry_Request__Sequence__init(&msg->request, 0)) {
    control_msgs__srv__SetOdometry_Event__fini(msg);
    return false;
  }
  // response
  if (!control_msgs__srv__SetOdometry_Response__Sequence__init(&msg->response, 0)) {
    control_msgs__srv__SetOdometry_Event__fini(msg);
    return false;
  }
  return true;
}

void
control_msgs__srv__SetOdometry_Event__fini(control_msgs__srv__SetOdometry_Event * msg)
{
  if (!msg) {
    return;
  }
  // info
  service_msgs__msg__ServiceEventInfo__fini(&msg->info);
  // request
  control_msgs__srv__SetOdometry_Request__Sequence__fini(&msg->request);
  // response
  control_msgs__srv__SetOdometry_Response__Sequence__fini(&msg->response);
}

bool
control_msgs__srv__SetOdometry_Event__are_equal(const control_msgs__srv__SetOdometry_Event * lhs, const control_msgs__srv__SetOdometry_Event * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // info
  if (!service_msgs__msg__ServiceEventInfo__are_equal(
      &(lhs->info), &(rhs->info)))
  {
    return false;
  }
  // request
  if (!control_msgs__srv__SetOdometry_Request__Sequence__are_equal(
      &(lhs->request), &(rhs->request)))
  {
    return false;
  }
  // response
  if (!control_msgs__srv__SetOdometry_Response__Sequence__are_equal(
      &(lhs->response), &(rhs->response)))
  {
    return false;
  }
  return true;
}

bool
control_msgs__srv__SetOdometry_Event__copy(
  const control_msgs__srv__SetOdometry_Event * input,
  control_msgs__srv__SetOdometry_Event * output)
{
  if (!input || !output) {
    return false;
  }
  // info
  if (!service_msgs__msg__ServiceEventInfo__copy(
      &(input->info), &(output->info)))
  {
    return false;
  }
  // request
  if (!control_msgs__srv__SetOdometry_Request__Sequence__copy(
      &(input->request), &(output->request)))
  {
    return false;
  }
  // response
  if (!control_msgs__srv__SetOdometry_Response__Sequence__copy(
      &(input->response), &(output->response)))
  {
    return false;
  }
  return true;
}

control_msgs__srv__SetOdometry_Event *
control_msgs__srv__SetOdometry_Event__create(void)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Event * msg = (control_msgs__srv__SetOdometry_Event *)allocator.allocate(sizeof(control_msgs__srv__SetOdometry_Event), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(control_msgs__srv__SetOdometry_Event));
  bool success = control_msgs__srv__SetOdometry_Event__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
control_msgs__srv__SetOdometry_Event__destroy(control_msgs__srv__SetOdometry_Event * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    control_msgs__srv__SetOdometry_Event__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
control_msgs__srv__SetOdometry_Event__Sequence__init(control_msgs__srv__SetOdometry_Event__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Event * data = NULL;

  if (size) {
    if (size > SIZE_MAX / sizeof(control_msgs__srv__SetOdometry_Event)) {
      return false;
    }
    data = (control_msgs__srv__SetOdometry_Event *)allocator.zero_allocate(size, sizeof(control_msgs__srv__SetOdometry_Event), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = control_msgs__srv__SetOdometry_Event__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        control_msgs__srv__SetOdometry_Event__fini(&data[i - 1]);
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
control_msgs__srv__SetOdometry_Event__Sequence__fini(control_msgs__srv__SetOdometry_Event__Sequence * array)
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
      control_msgs__srv__SetOdometry_Event__fini(&array->data[i]);
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

control_msgs__srv__SetOdometry_Event__Sequence *
control_msgs__srv__SetOdometry_Event__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  control_msgs__srv__SetOdometry_Event__Sequence * array = (control_msgs__srv__SetOdometry_Event__Sequence *)allocator.allocate(sizeof(control_msgs__srv__SetOdometry_Event__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = control_msgs__srv__SetOdometry_Event__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
control_msgs__srv__SetOdometry_Event__Sequence__destroy(control_msgs__srv__SetOdometry_Event__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    control_msgs__srv__SetOdometry_Event__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
control_msgs__srv__SetOdometry_Event__Sequence__are_equal(const control_msgs__srv__SetOdometry_Event__Sequence * lhs, const control_msgs__srv__SetOdometry_Event__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!control_msgs__srv__SetOdometry_Event__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
control_msgs__srv__SetOdometry_Event__Sequence__copy(
  const control_msgs__srv__SetOdometry_Event__Sequence * input,
  control_msgs__srv__SetOdometry_Event__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    if (input->size > SIZE_MAX / sizeof(control_msgs__srv__SetOdometry_Event)) {
      return false;
    }
    const size_t allocation_size =
      input->size * sizeof(control_msgs__srv__SetOdometry_Event);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    control_msgs__srv__SetOdometry_Event * data =
      (control_msgs__srv__SetOdometry_Event *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!control_msgs__srv__SetOdometry_Event__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          control_msgs__srv__SetOdometry_Event__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!control_msgs__srv__SetOdometry_Event__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
