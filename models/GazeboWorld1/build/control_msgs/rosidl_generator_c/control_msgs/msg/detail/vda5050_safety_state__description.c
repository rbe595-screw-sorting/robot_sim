// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from control_msgs:msg/VDA5050SafetyState.idl
// generated code does not contain a copyright notice

#include "control_msgs/msg/detail/vda5050_safety_state__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_control_msgs
const rosidl_type_hash_t *
control_msgs__msg__VDA5050SafetyState__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0x7c, 0x7a, 0xa3, 0x67, 0xaa, 0x9f, 0x12, 0xf1,
      0xed, 0x5b, 0xf5, 0x6a, 0x08, 0x63, 0x63, 0x35,
      0x70, 0xfb, 0xb9, 0x24, 0x98, 0x29, 0x12, 0x33,
      0xb6, 0x0f, 0x15, 0x67, 0x66, 0xe9, 0xb1, 0x80,
    }};
  return &hash;
}

#include <assert.h>
#include <string.h>

// Include directives for referenced types

// Hashes for external referenced types
#ifndef NDEBUG
#endif

static char control_msgs__msg__VDA5050SafetyState__TYPE_NAME[] = "control_msgs/msg/VDA5050SafetyState";

// Define type names, field names, and default values
static char control_msgs__msg__VDA5050SafetyState__FIELD_NAME__e_stop[] = "e_stop";
static char control_msgs__msg__VDA5050SafetyState__FIELD_NAME__field_violation[] = "field_violation";

static rosidl_runtime_c__type_description__Field control_msgs__msg__VDA5050SafetyState__FIELDS[] = {
  {
    {control_msgs__msg__VDA5050SafetyState__FIELD_NAME__e_stop, 6, 6},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_STRING,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {control_msgs__msg__VDA5050SafetyState__FIELD_NAME__field_violation, 15, 15},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
control_msgs__msg__VDA5050SafetyState__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {control_msgs__msg__VDA5050SafetyState__TYPE_NAME, 35, 35},
      {control_msgs__msg__VDA5050SafetyState__FIELDS, 2, 2},
    },
    {NULL, 0, 0},
  };
  if (!constructed) {
    constructed = true;
  }
  return &description;
}

static char toplevel_type_raw_source[] =
  "string e_stop           # Enum {autoAck, manual, remote, none} Acknowledge-Type of eStop:\n"
  "                        # autoAck: autoacknowledgeable e-stop is activated e.g. by bumper or protective field\n"
  "                        # manual: e-stop has to be acknowledged manually at the vehicle\n"
  "                        # remote: facility estop has to be acknowledged remotely\n"
  "                        # none: no e-stop activated\n"
  "bool field_violation    # Protective field violation. True: field is violated False: field is not violated\n"
  "\n"
  "# Enums for eStop\n"
  "string AUTO_ACK=\"autoAck\"\n"
  "string MANUAL=\"manual\"\n"
  "string REMOTE=\"remote\"\n"
  "string NONE=\"none\"";

static char msg_encoding[] = "msg";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
control_msgs__msg__VDA5050SafetyState__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {control_msgs__msg__VDA5050SafetyState__TYPE_NAME, 35, 35},
    {msg_encoding, 3, 3},
    {toplevel_type_raw_source, 638, 638},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
control_msgs__msg__VDA5050SafetyState__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[1];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 1, 1};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *control_msgs__msg__VDA5050SafetyState__get_individual_type_description_source(NULL),
    constructed = true;
  }
  return &source_sequence;
}
