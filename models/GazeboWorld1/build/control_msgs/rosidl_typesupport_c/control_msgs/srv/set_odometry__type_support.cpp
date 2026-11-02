// generated from rosidl_typesupport_c/resource/idl__type_support.cpp.em
// with input from control_msgs:srv/SetOdometry.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "control_msgs/srv/detail/set_odometry__struct.h"
#include "control_msgs/srv/detail/set_odometry__type_support.h"
#include "control_msgs/srv/detail/set_odometry__functions.h"
#include "rosidl_typesupport_c/identifier.h"
#include "rosidl_typesupport_c/message_type_support_dispatch.h"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_c/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace control_msgs
{

namespace srv
{

namespace rosidl_typesupport_c
{

typedef struct _SetOdometry_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetOdometry_Request_type_support_ids_t;

static const _SetOdometry_Request_type_support_ids_t _SetOdometry_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _SetOdometry_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetOdometry_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetOdometry_Request_type_support_symbol_names_t _SetOdometry_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, control_msgs, srv, SetOdometry_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, control_msgs, srv, SetOdometry_Request)),
  }
};

typedef struct _SetOdometry_Request_type_support_data_t
{
  void * data[2];
} _SetOdometry_Request_type_support_data_t;

static _SetOdometry_Request_type_support_data_t _SetOdometry_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetOdometry_Request_message_typesupport_map = {
  2,
  "control_msgs",
  &_SetOdometry_Request_message_typesupport_ids.typesupport_identifier[0],
  &_SetOdometry_Request_message_typesupport_symbol_names.symbol_name[0],
  &_SetOdometry_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SetOdometry_Request_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetOdometry_Request_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
  &control_msgs__srv__SetOdometry_Request__get_type_hash,
  &control_msgs__srv__SetOdometry_Request__get_type_description,
  &control_msgs__srv__SetOdometry_Request__get_type_description_sources,
};

}  // namespace rosidl_typesupport_c

}  // namespace srv

}  // namespace control_msgs

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, control_msgs, srv, SetOdometry_Request)() {
  return &::control_msgs::srv::rosidl_typesupport_c::SetOdometry_Request_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "control_msgs/srv/detail/set_odometry__struct.h"
// already included above
// #include "control_msgs/srv/detail/set_odometry__type_support.h"
// already included above
// #include "control_msgs/srv/detail/set_odometry__functions.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace control_msgs
{

namespace srv
{

namespace rosidl_typesupport_c
{

typedef struct _SetOdometry_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetOdometry_Response_type_support_ids_t;

static const _SetOdometry_Response_type_support_ids_t _SetOdometry_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _SetOdometry_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetOdometry_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetOdometry_Response_type_support_symbol_names_t _SetOdometry_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, control_msgs, srv, SetOdometry_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, control_msgs, srv, SetOdometry_Response)),
  }
};

typedef struct _SetOdometry_Response_type_support_data_t
{
  void * data[2];
} _SetOdometry_Response_type_support_data_t;

static _SetOdometry_Response_type_support_data_t _SetOdometry_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetOdometry_Response_message_typesupport_map = {
  2,
  "control_msgs",
  &_SetOdometry_Response_message_typesupport_ids.typesupport_identifier[0],
  &_SetOdometry_Response_message_typesupport_symbol_names.symbol_name[0],
  &_SetOdometry_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SetOdometry_Response_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetOdometry_Response_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
  &control_msgs__srv__SetOdometry_Response__get_type_hash,
  &control_msgs__srv__SetOdometry_Response__get_type_description,
  &control_msgs__srv__SetOdometry_Response__get_type_description_sources,
};

}  // namespace rosidl_typesupport_c

}  // namespace srv

}  // namespace control_msgs

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, control_msgs, srv, SetOdometry_Response)() {
  return &::control_msgs::srv::rosidl_typesupport_c::SetOdometry_Response_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "control_msgs/srv/detail/set_odometry__struct.h"
// already included above
// #include "control_msgs/srv/detail/set_odometry__type_support.h"
// already included above
// #include "control_msgs/srv/detail/set_odometry__functions.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
// already included above
// #include "rosidl_typesupport_c/message_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_c/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace control_msgs
{

namespace srv
{

namespace rosidl_typesupport_c
{

typedef struct _SetOdometry_Event_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetOdometry_Event_type_support_ids_t;

static const _SetOdometry_Event_type_support_ids_t _SetOdometry_Event_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _SetOdometry_Event_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetOdometry_Event_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetOdometry_Event_type_support_symbol_names_t _SetOdometry_Event_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, control_msgs, srv, SetOdometry_Event)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, control_msgs, srv, SetOdometry_Event)),
  }
};

typedef struct _SetOdometry_Event_type_support_data_t
{
  void * data[2];
} _SetOdometry_Event_type_support_data_t;

static _SetOdometry_Event_type_support_data_t _SetOdometry_Event_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetOdometry_Event_message_typesupport_map = {
  2,
  "control_msgs",
  &_SetOdometry_Event_message_typesupport_ids.typesupport_identifier[0],
  &_SetOdometry_Event_message_typesupport_symbol_names.symbol_name[0],
  &_SetOdometry_Event_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SetOdometry_Event_message_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetOdometry_Event_message_typesupport_map),
  rosidl_typesupport_c__get_message_typesupport_handle_function,
  &control_msgs__srv__SetOdometry_Event__get_type_hash,
  &control_msgs__srv__SetOdometry_Event__get_type_description,
  &control_msgs__srv__SetOdometry_Event__get_type_description_sources,
};

}  // namespace rosidl_typesupport_c

}  // namespace srv

}  // namespace control_msgs

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_c, control_msgs, srv, SetOdometry_Event)() {
  return &::control_msgs::srv::rosidl_typesupport_c::SetOdometry_Event_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "control_msgs/srv/detail/set_odometry__type_support.h"
// already included above
// #include "rosidl_typesupport_c/identifier.h"
#include "rosidl_typesupport_c/service_type_support_dispatch.h"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
#include "service_msgs/msg/service_event_info.h"
#include "builtin_interfaces/msg/time.h"

namespace control_msgs
{

namespace srv
{

namespace rosidl_typesupport_c
{
typedef struct _SetOdometry_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetOdometry_type_support_ids_t;

static const _SetOdometry_type_support_ids_t _SetOdometry_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_c",  // ::rosidl_typesupport_fastrtps_c::typesupport_identifier,
    "rosidl_typesupport_introspection_c",  // ::rosidl_typesupport_introspection_c::typesupport_identifier,
  }
};

typedef struct _SetOdometry_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetOdometry_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetOdometry_type_support_symbol_names_t _SetOdometry_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, control_msgs, srv, SetOdometry)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, control_msgs, srv, SetOdometry)),
  }
};

typedef struct _SetOdometry_type_support_data_t
{
  void * data[2];
} _SetOdometry_type_support_data_t;

static _SetOdometry_type_support_data_t _SetOdometry_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetOdometry_service_typesupport_map = {
  2,
  "control_msgs",
  &_SetOdometry_service_typesupport_ids.typesupport_identifier[0],
  &_SetOdometry_service_typesupport_symbol_names.symbol_name[0],
  &_SetOdometry_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t SetOdometry_service_type_support_handle = {
  rosidl_typesupport_c__typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetOdometry_service_typesupport_map),
  rosidl_typesupport_c__get_service_typesupport_handle_function,
  &SetOdometry_Request_message_type_support_handle,
  &SetOdometry_Response_message_type_support_handle,
  &SetOdometry_Event_message_type_support_handle,
  ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_CREATE_EVENT_MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c,
    control_msgs,
    srv,
    SetOdometry
  ),
  ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_DESTROY_EVENT_MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c,
    control_msgs,
    srv,
    SetOdometry
  ),
  &control_msgs__srv__SetOdometry__get_type_hash,
  &control_msgs__srv__SetOdometry__get_type_description,
  &control_msgs__srv__SetOdometry__get_type_description_sources,
};

}  // namespace rosidl_typesupport_c

}  // namespace srv

}  // namespace control_msgs

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_c, control_msgs, srv, SetOdometry)() {
  return &::control_msgs::srv::rosidl_typesupport_c::SetOdometry_service_type_support_handle;
}

#ifdef __cplusplus
}
#endif
