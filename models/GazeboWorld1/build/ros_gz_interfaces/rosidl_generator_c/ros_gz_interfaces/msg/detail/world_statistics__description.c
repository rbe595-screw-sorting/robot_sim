// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from ros_gz_interfaces:msg/WorldStatistics.idl
// generated code does not contain a copyright notice

#include "ros_gz_interfaces/msg/detail/world_statistics__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_ros_gz_interfaces
const rosidl_type_hash_t *
ros_gz_interfaces__msg__WorldStatistics__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0x53, 0x35, 0xd3, 0x3b, 0x33, 0x20, 0xa2, 0x55,
      0x9c, 0xf0, 0xc3, 0xfc, 0x76, 0x48, 0xaf, 0x05,
      0x61, 0x39, 0xc9, 0x62, 0x6e, 0x5d, 0x0a, 0x76,
      0x8b, 0xa4, 0x74, 0x77, 0xd3, 0xb8, 0x1a, 0x94,
    }};
  return &hash;
}

#include <assert.h>
#include <string.h>

// Include directives for referenced types
#include "ros_gz_interfaces/msg/detail/log_playback_statistics__functions.h"
#include "std_msgs/msg/detail/header__functions.h"
#include "builtin_interfaces/msg/detail/time__functions.h"

// Hashes for external referenced types
#ifndef NDEBUG
static const rosidl_type_hash_t builtin_interfaces__msg__Time__EXPECTED_HASH = {1, {
    0xb1, 0x06, 0x23, 0x5e, 0x25, 0xa4, 0xc5, 0xed,
    0x35, 0x09, 0x8a, 0xa0, 0xa6, 0x1a, 0x3e, 0xe9,
    0xc9, 0xb1, 0x8d, 0x19, 0x7f, 0x39, 0x8b, 0x0e,
    0x42, 0x06, 0xce, 0xa9, 0xac, 0xf9, 0xc1, 0x97,
  }};
static const rosidl_type_hash_t ros_gz_interfaces__msg__LogPlaybackStatistics__EXPECTED_HASH = {1, {
    0xf5, 0x61, 0x4a, 0x5b, 0x25, 0xcf, 0x37, 0x29,
    0x76, 0x2d, 0xcf, 0x09, 0x9f, 0xee, 0x12, 0x8c,
    0x43, 0x00, 0x4c, 0x65, 0x63, 0x69, 0x25, 0x49,
    0x79, 0xa6, 0x3e, 0x30, 0xe2, 0x30, 0x73, 0x0c,
  }};
static const rosidl_type_hash_t std_msgs__msg__Header__EXPECTED_HASH = {1, {
    0xf4, 0x9f, 0xb3, 0xae, 0x2c, 0xf0, 0x70, 0xf7,
    0x93, 0x64, 0x5f, 0xf7, 0x49, 0x68, 0x3a, 0xc6,
    0xb0, 0x62, 0x03, 0xe4, 0x1c, 0x89, 0x1e, 0x17,
    0x70, 0x1b, 0x1c, 0xb5, 0x97, 0xce, 0x6a, 0x01,
  }};
#endif

static char ros_gz_interfaces__msg__WorldStatistics__TYPE_NAME[] = "ros_gz_interfaces/msg/WorldStatistics";
static char builtin_interfaces__msg__Time__TYPE_NAME[] = "builtin_interfaces/msg/Time";
static char ros_gz_interfaces__msg__LogPlaybackStatistics__TYPE_NAME[] = "ros_gz_interfaces/msg/LogPlaybackStatistics";
static char std_msgs__msg__Header__TYPE_NAME[] = "std_msgs/msg/Header";

// Define type names, field names, and default values
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__header[] = "header";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__sim_time[] = "sim_time";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__pause_time[] = "pause_time";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__real_time[] = "real_time";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__paused[] = "paused";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__iterations[] = "iterations";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__model_count[] = "model_count";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__log_playback_statistics[] = "log_playback_statistics";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__real_time_factor[] = "real_time_factor";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__step_size[] = "step_size";
static char ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__stepping[] = "stepping";

static rosidl_runtime_c__type_description__Field ros_gz_interfaces__msg__WorldStatistics__FIELDS[] = {
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__header, 6, 6},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {std_msgs__msg__Header__TYPE_NAME, 19, 19},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__sim_time, 8, 8},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__pause_time, 10, 10},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__real_time, 9, 9},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__paused, 6, 6},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__iterations, 10, 10},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_UINT64,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__model_count, 11, 11},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_INT32,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__log_playback_statistics, 23, 23},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {ros_gz_interfaces__msg__LogPlaybackStatistics__TYPE_NAME, 43, 43},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__real_time_factor, 16, 16},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_DOUBLE,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__step_size, 9, 9},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    },
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__WorldStatistics__FIELD_NAME__stepping, 8, 8},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

static rosidl_runtime_c__type_description__IndividualTypeDescription ros_gz_interfaces__msg__WorldStatistics__REFERENCED_TYPE_DESCRIPTIONS[] = {
  {
    {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    {NULL, 0, 0},
  },
  {
    {ros_gz_interfaces__msg__LogPlaybackStatistics__TYPE_NAME, 43, 43},
    {NULL, 0, 0},
  },
  {
    {std_msgs__msg__Header__TYPE_NAME, 19, 19},
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
ros_gz_interfaces__msg__WorldStatistics__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {ros_gz_interfaces__msg__WorldStatistics__TYPE_NAME, 37, 37},
      {ros_gz_interfaces__msg__WorldStatistics__FIELDS, 11, 11},
    },
    {ros_gz_interfaces__msg__WorldStatistics__REFERENCED_TYPE_DESCRIPTIONS, 3, 3},
  };
  if (!constructed) {
    assert(0 == memcmp(&builtin_interfaces__msg__Time__EXPECTED_HASH, builtin_interfaces__msg__Time__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[0].fields = builtin_interfaces__msg__Time__get_type_description(NULL)->type_description.fields;
    assert(0 == memcmp(&ros_gz_interfaces__msg__LogPlaybackStatistics__EXPECTED_HASH, ros_gz_interfaces__msg__LogPlaybackStatistics__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[1].fields = ros_gz_interfaces__msg__LogPlaybackStatistics__get_type_description(NULL)->type_description.fields;
    assert(0 == memcmp(&std_msgs__msg__Header__EXPECTED_HASH, std_msgs__msg__Header__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[2].fields = std_msgs__msg__Header__get_type_description(NULL)->type_description.fields;
    constructed = true;
  }
  return &description;
}

static char toplevel_type_raw_source[] =
  "# optional header data\n"
  "std_msgs/Header header\n"
  "\n"
  "# Current simulation time\n"
  "builtin_interfaces/Time sim_time\n"
  "\n"
  "# Total time spent paused\n"
  "builtin_interfaces/Time pause_time\n"
  "\n"
  "# Current real time\n"
  "builtin_interfaces/Time real_time\n"
  "\n"
  "# Whether currently paused\n"
  "bool paused\n"
  "\n"
  "# Current iteration count\n"
  "uint64 iterations\n"
  "\n"
  "# Total number of models in the world\n"
  "int32 model_count\n"
  "\n"
  "# Statistics for log playback\n"
  "LogPlaybackStatistics log_playback_statistics\n"
  "\n"
  "# This factor expresses how much real time elapses with each step\n"
  "# of simulation time.\n"
  "float64 real_time_factor\n"
  "\n"
  "# Iteration step size. It's zero when paused.\n"
  "builtin_interfaces/Time step_size\n"
  "\n"
  "# True if the simulator is stepping, as opposed to running. This\n"
  "# field can be ignored if the paused field is true.\n"
  "bool stepping";

static char msg_encoding[] = "msg";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
ros_gz_interfaces__msg__WorldStatistics__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {ros_gz_interfaces__msg__WorldStatistics__TYPE_NAME, 37, 37},
    {msg_encoding, 3, 3},
    {toplevel_type_raw_source, 769, 769},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
ros_gz_interfaces__msg__WorldStatistics__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[4];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 4, 4};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *ros_gz_interfaces__msg__WorldStatistics__get_individual_type_description_source(NULL),
    sources[1] = *builtin_interfaces__msg__Time__get_individual_type_description_source(NULL);
    sources[2] = *ros_gz_interfaces__msg__LogPlaybackStatistics__get_individual_type_description_source(NULL);
    sources[3] = *std_msgs__msg__Header__get_individual_type_description_source(NULL);
    constructed = true;
  }
  return &source_sequence;
}
