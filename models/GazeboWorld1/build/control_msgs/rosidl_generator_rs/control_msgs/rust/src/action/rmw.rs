
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_Goal() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__init(msg: *mut ExecuteMotionPrimitiveSequence_Goal) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Goal>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Goal>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Goal>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: super::super::msg::rmw::MotionPrimitiveSequence,

}



impl Default for ExecuteMotionPrimitiveSequence_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_Goal() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_Result() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Result__init(msg: *mut ExecuteMotionPrimitiveSequence_Result) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Result>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Result>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Result>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_string: rosidl_runtime_rs::String,

}

impl ExecuteMotionPrimitiveSequence_Result {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUCCESSFUL: i32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_GOAL: i32 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTION_EXECUTION_FAILED: i32 = -2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OLD_HEADER_TIMESTAMP: i32 = -3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ABORTED_BY_DEACTIVATION: i32 = -4;

}


impl Default for ExecuteMotionPrimitiveSequence_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_Result__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_Result where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_Result() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__init(msg: *mut ExecuteMotionPrimitiveSequence_Feedback) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Feedback>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Feedback>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_Feedback>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_primitive_index: u8,

}



impl Default for ExecuteMotionPrimitiveSequence_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__init(msg: *mut ExecuteMotionPrimitiveSequence_FeedbackMessage) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_FeedbackMessage>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_FeedbackMessage>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_FeedbackMessage>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::ExecuteMotionPrimitiveSequence_Feedback,

}



impl Default for ExecuteMotionPrimitiveSequence_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_Goal() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_Goal__init(msg: *mut FollowJointTrajectory_Goal) -> bool;
    fn control_msgs__action__FollowJointTrajectory_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Goal>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Goal>);
    fn control_msgs__action__FollowJointTrajectory_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Goal>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: trajectory_msgs::msg::rmw::JointTrajectory,

    /// The trajectory for all planar or floating joints (i.e. individual joints with more than one DOF)
    pub multi_dof_trajectory: trajectory_msgs::msg::rmw::MultiDOFJointTrajectory,

    /// Tolerances for the trajectory.  If the measured joint values fall
    /// outside the tolerances the trajectory goal is aborted.  Any
    /// tolerances that are not specified (by being omitted or set to 0) are
    /// set to the defaults for the action server (often taken from the
    /// parameter server).
    /// Tolerances applied to the joints as the trajectory is executed.  If
    /// violated, the goal aborts with error_code set to
    /// PATH_TOLERANCE_VIOLATED.
    pub path_tolerance: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointTolerance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub component_path_tolerance: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointComponentTolerance>,

    /// To report success, the joints must be within goal_tolerance of the
    /// final trajectory value.  The goal must be achieved by time the
    /// trajectory ends plus goal_time_tolerance.  (goal_time_tolerance
    /// allows some leeway in time, so that the trajectory goal can still
    /// succeed even if the joints reach the goal some time after the
    /// precise end time of the trajectory).
    ///
    /// If the joints are not within goal_tolerance after "trajectory finish
    /// time" + goal_time_tolerance, the goal aborts with error_code set to
    /// GOAL_TOLERANCE_VIOLATED
    pub goal_tolerance: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointTolerance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub component_goal_tolerance: rosidl_runtime_rs::Sequence<super::super::msg::rmw::JointComponentTolerance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_time_tolerance: builtin_interfaces::msg::rmw::Duration,

}



impl Default for FollowJointTrajectory_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_Goal__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_Goal() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_Result() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_Result__init(msg: *mut FollowJointTrajectory_Result) -> bool;
    fn control_msgs__action__FollowJointTrajectory_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Result>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Result>);
    fn control_msgs__action__FollowJointTrajectory_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Result>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: i32,

    /// Human readable description of the error code. Contains complementary
    /// information that is especially useful when execution fails, for instance:
    /// - INVALID_GOAL: The reason for the invalid goal (e.g., the requested
    ///   trajectory is in the past).
    /// - INVALID_JOINTS: The mismatch between the expected controller joints
    ///   and those provided in the goal.
    /// - PATH_TOLERANCE_VIOLATED and GOAL_TOLERANCE_VIOLATED: Which joint
    ///   violated which tolerance, and by how much.
    pub error_string: rosidl_runtime_rs::String,

}

impl FollowJointTrajectory_Result {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SUCCESSFUL: i32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_GOAL: i32 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_JOINTS: i32 = -2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OLD_HEADER_TIMESTAMP: i32 = -3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PATH_TOLERANCE_VIOLATED: i32 = -4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GOAL_TOLERANCE_VIOLATED: i32 = -5;

}


impl Default for FollowJointTrajectory_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_Result__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_Result where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_Result() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_Feedback__init(msg: *mut FollowJointTrajectory_Feedback) -> bool;
    fn control_msgs__action__FollowJointTrajectory_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Feedback>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Feedback>);
    fn control_msgs__action__FollowJointTrajectory_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_Feedback>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub desired: trajectory_msgs::msg::rmw::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub actual: trajectory_msgs::msg::rmw::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error: trajectory_msgs::msg::rmw::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_desired: trajectory_msgs::msg::rmw::MultiDOFJointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_actual: trajectory_msgs::msg::rmw::MultiDOFJointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_error: trajectory_msgs::msg::rmw::MultiDOFJointTrajectoryPoint,

}



impl Default for FollowJointTrajectory_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_Feedback__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_Feedback() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_FeedbackMessage__init(msg: *mut FollowJointTrajectory_FeedbackMessage) -> bool;
    fn control_msgs__action__FollowJointTrajectory_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_FeedbackMessage>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_FeedbackMessage>);
    fn control_msgs__action__FollowJointTrajectory_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_FeedbackMessage>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::FollowJointTrajectory_Feedback,

}



impl Default for FollowJointTrajectory_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_FeedbackMessage() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_Goal() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_Goal__init(msg: *mut GripperCommand_Goal) -> bool;
    fn control_msgs__action__GripperCommand_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Goal>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Goal>);
    fn control_msgs__action__GripperCommand_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Goal>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: super::super::msg::rmw::GripperCommand,

}



impl Default for GripperCommand_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_Goal__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_Goal() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_Result() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_Result__init(msg: *mut GripperCommand_Result) -> bool;
    fn control_msgs__action__GripperCommand_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Result>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Result>);
    fn control_msgs__action__GripperCommand_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Result>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_Result {
    /// The current gripper gap size (in meters)
    pub position: f64,

    /// The current effort exerted (in Newtons)
    pub effort: f64,

    /// True iff the gripper is exerting max effort and not moving
    pub stalled: bool,

    /// True iff the gripper position has reached the commanded setpoint
    pub reached_goal: bool,

}



impl Default for GripperCommand_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_Result__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_Result where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_Result() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_Feedback__init(msg: *mut GripperCommand_Feedback) -> bool;
    fn control_msgs__action__GripperCommand_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Feedback>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Feedback>);
    fn control_msgs__action__GripperCommand_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Feedback>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_Feedback {
    /// The current gripper gap size (in meters)
    pub position: f64,

    /// The current effort exerted (in Newtons)
    pub effort: f64,

    /// True iff the gripper is exerting max effort and not moving
    pub stalled: bool,

    /// True iff the gripper position has reached the commanded setpoint
    pub reached_goal: bool,

}



impl Default for GripperCommand_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_Feedback__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_Feedback() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_FeedbackMessage__init(msg: *mut GripperCommand_FeedbackMessage) -> bool;
    fn control_msgs__action__GripperCommand_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>);
    fn control_msgs__action__GripperCommand_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::GripperCommand_Feedback,

}



impl Default for GripperCommand_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_FeedbackMessage() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_Goal() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_Goal__init(msg: *mut JointTrajectory_Goal) -> bool;
    fn control_msgs__action__JointTrajectory_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Goal>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Goal>);
    fn control_msgs__action__JointTrajectory_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Goal>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: trajectory_msgs::msg::rmw::JointTrajectory,

}



impl Default for JointTrajectory_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_Goal__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_Goal() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_Result() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_Result__init(msg: *mut JointTrajectory_Result) -> bool;
    fn control_msgs__action__JointTrajectory_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Result>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Result>);
    fn control_msgs__action__JointTrajectory_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Result>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for JointTrajectory_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_Result__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_Result where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_Result() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_Feedback__init(msg: *mut JointTrajectory_Feedback) -> bool;
    fn control_msgs__action__JointTrajectory_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Feedback>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Feedback>);
    fn control_msgs__action__JointTrajectory_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_Feedback>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for JointTrajectory_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_Feedback__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_Feedback() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_FeedbackMessage__init(msg: *mut JointTrajectory_FeedbackMessage) -> bool;
    fn control_msgs__action__JointTrajectory_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_FeedbackMessage>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_FeedbackMessage>);
    fn control_msgs__action__JointTrajectory_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_FeedbackMessage>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::JointTrajectory_Feedback,

}



impl Default for JointTrajectory_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_FeedbackMessage() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_Goal() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_Goal__init(msg: *mut ParallelGripperCommand_Goal) -> bool;
    fn control_msgs__action__ParallelGripperCommand_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Goal>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Goal>);
    fn control_msgs__action__ParallelGripperCommand_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Goal>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: sensor_msgs::msg::rmw::JointState,

}



impl Default for ParallelGripperCommand_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_Goal__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_Goal() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_Result() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_Result__init(msg: *mut ParallelGripperCommand_Result) -> bool;
    fn control_msgs__action__ParallelGripperCommand_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Result>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Result>);
    fn control_msgs__action__ParallelGripperCommand_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Result>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_Result {
    /// The current gripper state.
    pub state: sensor_msgs::msg::rmw::JointState,

    /// position of each joint (radians or meters)
    /// optional: velocity of each joint (radians or meters / second)
    /// optional: effort of each joint (Newtons or Newton-meters)
    /// True if the gripper is exerting max effort and not moving
    pub stalled: bool,

    /// True if the gripper position has reached the commanded setpoint
    pub reached_goal: bool,

}



impl Default for ParallelGripperCommand_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_Result__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_Result where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_Result() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_Feedback__init(msg: *mut ParallelGripperCommand_Feedback) -> bool;
    fn control_msgs__action__ParallelGripperCommand_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Feedback>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Feedback>);
    fn control_msgs__action__ParallelGripperCommand_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_Feedback>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_Feedback {
    /// The current gripper state.
    pub state: sensor_msgs::msg::rmw::JointState,

}



impl Default for ParallelGripperCommand_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_Feedback__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_Feedback() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_FeedbackMessage__init(msg: *mut ParallelGripperCommand_FeedbackMessage) -> bool;
    fn control_msgs__action__ParallelGripperCommand_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_FeedbackMessage>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_FeedbackMessage>);
    fn control_msgs__action__ParallelGripperCommand_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_FeedbackMessage>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::ParallelGripperCommand_Feedback,

}



impl Default for ParallelGripperCommand_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_FeedbackMessage() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_Goal() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_Goal__init(msg: *mut PointHead_Goal) -> bool;
    fn control_msgs__action__PointHead_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_Goal>, size: usize) -> bool;
    fn control_msgs__action__PointHead_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_Goal>);
    fn control_msgs__action__PointHead_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_Goal>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target: geometry_msgs::msg::rmw::PointStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pointing_axis: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pointing_frame: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub min_duration: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_velocity: f64,

}



impl Default for PointHead_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_Goal__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_Goal() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_Result() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_Result__init(msg: *mut PointHead_Result) -> bool;
    fn control_msgs__action__PointHead_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_Result>, size: usize) -> bool;
    fn control_msgs__action__PointHead_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_Result>);
    fn control_msgs__action__PointHead_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_Result>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for PointHead_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_Result__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_Result where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_Result() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_Feedback__init(msg: *mut PointHead_Feedback) -> bool;
    fn control_msgs__action__PointHead_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_Feedback>, size: usize) -> bool;
    fn control_msgs__action__PointHead_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_Feedback>);
    fn control_msgs__action__PointHead_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_Feedback>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pointing_angle_error: f64,

}



impl Default for PointHead_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_Feedback__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_Feedback() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_FeedbackMessage__init(msg: *mut PointHead_FeedbackMessage) -> bool;
    fn control_msgs__action__PointHead_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_FeedbackMessage>, size: usize) -> bool;
    fn control_msgs__action__PointHead_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_FeedbackMessage>);
    fn control_msgs__action__PointHead_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_FeedbackMessage>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::PointHead_Feedback,

}



impl Default for PointHead_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_FeedbackMessage() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_Goal() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_Goal__init(msg: *mut SingleJointPosition_Goal) -> bool;
    fn control_msgs__action__SingleJointPosition_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Goal>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Goal>);
    fn control_msgs__action__SingleJointPosition_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Goal>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub min_duration: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_velocity: f64,

}



impl Default for SingleJointPosition_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_Goal__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_Goal() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_Result() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_Result__init(msg: *mut SingleJointPosition_Result) -> bool;
    fn control_msgs__action__SingleJointPosition_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Result>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Result>);
    fn control_msgs__action__SingleJointPosition_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Result>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SingleJointPosition_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_Result__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_Result where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_Result() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_Feedback__init(msg: *mut SingleJointPosition_Feedback) -> bool;
    fn control_msgs__action__SingleJointPosition_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Feedback>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Feedback>);
    fn control_msgs__action__SingleJointPosition_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_Feedback>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error: f64,

}



impl Default for SingleJointPosition_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_Feedback__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_Feedback() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_FeedbackMessage__init(msg: *mut SingleJointPosition_FeedbackMessage) -> bool;
    fn control_msgs__action__SingleJointPosition_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_FeedbackMessage>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_FeedbackMessage>);
    fn control_msgs__action__SingleJointPosition_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_FeedbackMessage>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::SingleJointPosition_Feedback,

}



impl Default for SingleJointPosition_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_FeedbackMessage() }
  }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__init(msg: *mut ExecuteMotionPrimitiveSequence_SendGoal_Request) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Request>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Request>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Request>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::ExecuteMotionPrimitiveSequence_Goal,

}



impl Default for ExecuteMotionPrimitiveSequence_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__init(msg: *mut ExecuteMotionPrimitiveSequence_SendGoal_Response) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Response>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Response>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_SendGoal_Response>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ExecuteMotionPrimitiveSequence_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__init(msg: *mut ExecuteMotionPrimitiveSequence_GetResult_Request) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Request>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Request>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Request>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for ExecuteMotionPrimitiveSequence_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__init(msg: *mut ExecuteMotionPrimitiveSequence_GetResult_Response) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Response>, size: usize) -> bool;
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Response>);
    fn control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecuteMotionPrimitiveSequence_GetResult_Response>) -> bool;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::ExecuteMotionPrimitiveSequence_Result,

}



impl Default for ExecuteMotionPrimitiveSequence_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecuteMotionPrimitiveSequence_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecuteMotionPrimitiveSequence_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ExecuteMotionPrimitiveSequence_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Request__init(msg: *mut FollowJointTrajectory_SendGoal_Request) -> bool;
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Request>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Request>);
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Request>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::FollowJointTrajectory_Goal,

}



impl Default for FollowJointTrajectory_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_SendGoal_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Response__init(msg: *mut FollowJointTrajectory_SendGoal_Response) -> bool;
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Response>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Response>);
    fn control_msgs__action__FollowJointTrajectory_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_SendGoal_Response>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for FollowJointTrajectory_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_SendGoal_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_GetResult_Request__init(msg: *mut FollowJointTrajectory_GetResult_Request) -> bool;
    fn control_msgs__action__FollowJointTrajectory_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Request>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Request>);
    fn control_msgs__action__FollowJointTrajectory_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Request>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for FollowJointTrajectory_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_GetResult_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__FollowJointTrajectory_GetResult_Response__init(msg: *mut FollowJointTrajectory_GetResult_Response) -> bool;
    fn control_msgs__action__FollowJointTrajectory_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Response>, size: usize) -> bool;
    fn control_msgs__action__FollowJointTrajectory_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Response>);
    fn control_msgs__action__FollowJointTrajectory_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FollowJointTrajectory_GetResult_Response>) -> bool;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::FollowJointTrajectory_Result,

}



impl Default for FollowJointTrajectory_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__FollowJointTrajectory_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__FollowJointTrajectory_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FollowJointTrajectory_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__FollowJointTrajectory_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FollowJointTrajectory_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/FollowJointTrajectory_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__FollowJointTrajectory_GetResult_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_SendGoal_Request__init(msg: *mut GripperCommand_SendGoal_Request) -> bool;
    fn control_msgs__action__GripperCommand_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>);
    fn control_msgs__action__GripperCommand_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::GripperCommand_Goal,

}



impl Default for GripperCommand_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_SendGoal_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_SendGoal_Response__init(msg: *mut GripperCommand_SendGoal_Response) -> bool;
    fn control_msgs__action__GripperCommand_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>);
    fn control_msgs__action__GripperCommand_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for GripperCommand_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_SendGoal_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_GetResult_Request__init(msg: *mut GripperCommand_GetResult_Request) -> bool;
    fn control_msgs__action__GripperCommand_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>);
    fn control_msgs__action__GripperCommand_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for GripperCommand_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_GetResult_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__GripperCommand_GetResult_Response__init(msg: *mut GripperCommand_GetResult_Response) -> bool;
    fn control_msgs__action__GripperCommand_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>, size: usize) -> bool;
    fn control_msgs__action__GripperCommand_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>);
    fn control_msgs__action__GripperCommand_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>) -> bool;
}

// Corresponds to control_msgs__action__GripperCommand_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::GripperCommand_Result,

}



impl Default for GripperCommand_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__GripperCommand_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__GripperCommand_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__GripperCommand_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/GripperCommand_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__GripperCommand_GetResult_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_SendGoal_Request__init(msg: *mut JointTrajectory_SendGoal_Request) -> bool;
    fn control_msgs__action__JointTrajectory_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Request>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Request>);
    fn control_msgs__action__JointTrajectory_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Request>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::JointTrajectory_Goal,

}



impl Default for JointTrajectory_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_SendGoal_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_SendGoal_Response__init(msg: *mut JointTrajectory_SendGoal_Response) -> bool;
    fn control_msgs__action__JointTrajectory_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Response>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Response>);
    fn control_msgs__action__JointTrajectory_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_SendGoal_Response>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for JointTrajectory_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_SendGoal_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_GetResult_Request__init(msg: *mut JointTrajectory_GetResult_Request) -> bool;
    fn control_msgs__action__JointTrajectory_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Request>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Request>);
    fn control_msgs__action__JointTrajectory_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Request>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for JointTrajectory_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_GetResult_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__JointTrajectory_GetResult_Response__init(msg: *mut JointTrajectory_GetResult_Response) -> bool;
    fn control_msgs__action__JointTrajectory_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Response>, size: usize) -> bool;
    fn control_msgs__action__JointTrajectory_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Response>);
    fn control_msgs__action__JointTrajectory_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectory_GetResult_Response>) -> bool;
}

// Corresponds to control_msgs__action__JointTrajectory_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::JointTrajectory_Result,

}



impl Default for JointTrajectory_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__JointTrajectory_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__JointTrajectory_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectory_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__JointTrajectory_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectory_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/JointTrajectory_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__JointTrajectory_GetResult_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Request__init(msg: *mut ParallelGripperCommand_SendGoal_Request) -> bool;
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Request>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Request>);
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Request>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::ParallelGripperCommand_Goal,

}



impl Default for ParallelGripperCommand_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_SendGoal_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Response__init(msg: *mut ParallelGripperCommand_SendGoal_Response) -> bool;
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Response>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Response>);
    fn control_msgs__action__ParallelGripperCommand_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_SendGoal_Response>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ParallelGripperCommand_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_SendGoal_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_GetResult_Request__init(msg: *mut ParallelGripperCommand_GetResult_Request) -> bool;
    fn control_msgs__action__ParallelGripperCommand_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Request>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Request>);
    fn control_msgs__action__ParallelGripperCommand_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Request>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for ParallelGripperCommand_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_GetResult_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__ParallelGripperCommand_GetResult_Response__init(msg: *mut ParallelGripperCommand_GetResult_Response) -> bool;
    fn control_msgs__action__ParallelGripperCommand_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Response>, size: usize) -> bool;
    fn control_msgs__action__ParallelGripperCommand_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Response>);
    fn control_msgs__action__ParallelGripperCommand_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ParallelGripperCommand_GetResult_Response>) -> bool;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::ParallelGripperCommand_Result,

}



impl Default for ParallelGripperCommand_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__ParallelGripperCommand_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__ParallelGripperCommand_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParallelGripperCommand_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__ParallelGripperCommand_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParallelGripperCommand_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/ParallelGripperCommand_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__ParallelGripperCommand_GetResult_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_SendGoal_Request__init(msg: *mut PointHead_SendGoal_Request) -> bool;
    fn control_msgs__action__PointHead_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_SendGoal_Request>, size: usize) -> bool;
    fn control_msgs__action__PointHead_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_SendGoal_Request>);
    fn control_msgs__action__PointHead_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_SendGoal_Request>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::PointHead_Goal,

}



impl Default for PointHead_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_SendGoal_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_SendGoal_Response__init(msg: *mut PointHead_SendGoal_Response) -> bool;
    fn control_msgs__action__PointHead_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_SendGoal_Response>, size: usize) -> bool;
    fn control_msgs__action__PointHead_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_SendGoal_Response>);
    fn control_msgs__action__PointHead_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_SendGoal_Response>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for PointHead_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_SendGoal_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_GetResult_Request__init(msg: *mut PointHead_GetResult_Request) -> bool;
    fn control_msgs__action__PointHead_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_GetResult_Request>, size: usize) -> bool;
    fn control_msgs__action__PointHead_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_GetResult_Request>);
    fn control_msgs__action__PointHead_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_GetResult_Request>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for PointHead_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_GetResult_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__PointHead_GetResult_Response__init(msg: *mut PointHead_GetResult_Response) -> bool;
    fn control_msgs__action__PointHead_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointHead_GetResult_Response>, size: usize) -> bool;
    fn control_msgs__action__PointHead_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointHead_GetResult_Response>);
    fn control_msgs__action__PointHead_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointHead_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PointHead_GetResult_Response>) -> bool;
}

// Corresponds to control_msgs__action__PointHead_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::PointHead_Result,

}



impl Default for PointHead_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__PointHead_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__PointHead_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointHead_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__PointHead_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointHead_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointHead_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/PointHead_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__PointHead_GetResult_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_SendGoal_Request__init(msg: *mut SingleJointPosition_SendGoal_Request) -> bool;
    fn control_msgs__action__SingleJointPosition_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Request>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Request>);
    fn control_msgs__action__SingleJointPosition_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Request>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::SingleJointPosition_Goal,

}



impl Default for SingleJointPosition_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_SendGoal_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_SendGoal_Response__init(msg: *mut SingleJointPosition_SendGoal_Response) -> bool;
    fn control_msgs__action__SingleJointPosition_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Response>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Response>);
    fn control_msgs__action__SingleJointPosition_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_SendGoal_Response>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for SingleJointPosition_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_SendGoal_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_GetResult_Request__init(msg: *mut SingleJointPosition_GetResult_Request) -> bool;
    fn control_msgs__action__SingleJointPosition_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Request>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Request>);
    fn control_msgs__action__SingleJointPosition_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Request>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for SingleJointPosition_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_GetResult_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__action__SingleJointPosition_GetResult_Response__init(msg: *mut SingleJointPosition_GetResult_Response) -> bool;
    fn control_msgs__action__SingleJointPosition_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Response>, size: usize) -> bool;
    fn control_msgs__action__SingleJointPosition_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Response>);
    fn control_msgs__action__SingleJointPosition_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleJointPosition_GetResult_Response>) -> bool;
}

// Corresponds to control_msgs__action__SingleJointPosition_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::SingleJointPosition_Result,

}



impl Default for SingleJointPosition_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__action__SingleJointPosition_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__action__SingleJointPosition_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleJointPosition_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__action__SingleJointPosition_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleJointPosition_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/action/SingleJointPosition_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__action__SingleJointPosition_GetResult_Response() }
  }
}






#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteMotionPrimitiveSequence_SendGoal;

impl rosidl_runtime_rs::Service for ExecuteMotionPrimitiveSequence_SendGoal {
    type Request = ExecuteMotionPrimitiveSequence_SendGoal_Request;
    type Response = ExecuteMotionPrimitiveSequence_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteMotionPrimitiveSequence_GetResult;

impl rosidl_runtime_rs::Service for ExecuteMotionPrimitiveSequence_GetResult {
    type Request = ExecuteMotionPrimitiveSequence_GetResult_Request;
    type Response = ExecuteMotionPrimitiveSequence_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__FollowJointTrajectory_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct FollowJointTrajectory_SendGoal;

impl rosidl_runtime_rs::Service for FollowJointTrajectory_SendGoal {
    type Request = FollowJointTrajectory_SendGoal_Request;
    type Response = FollowJointTrajectory_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__FollowJointTrajectory_SendGoal() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__FollowJointTrajectory_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__FollowJointTrajectory_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct FollowJointTrajectory_GetResult;

impl rosidl_runtime_rs::Service for FollowJointTrajectory_GetResult {
    type Request = FollowJointTrajectory_GetResult_Request;
    type Response = FollowJointTrajectory_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__FollowJointTrajectory_GetResult() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__GripperCommand_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__GripperCommand_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperCommand_SendGoal;

impl rosidl_runtime_rs::Service for GripperCommand_SendGoal {
    type Request = GripperCommand_SendGoal_Request;
    type Response = GripperCommand_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__GripperCommand_SendGoal() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__GripperCommand_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__GripperCommand_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperCommand_GetResult;

impl rosidl_runtime_rs::Service for GripperCommand_GetResult {
    type Request = GripperCommand_GetResult_Request;
    type Response = GripperCommand_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__GripperCommand_GetResult() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__JointTrajectory_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__JointTrajectory_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct JointTrajectory_SendGoal;

impl rosidl_runtime_rs::Service for JointTrajectory_SendGoal {
    type Request = JointTrajectory_SendGoal_Request;
    type Response = JointTrajectory_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__JointTrajectory_SendGoal() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__JointTrajectory_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__JointTrajectory_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct JointTrajectory_GetResult;

impl rosidl_runtime_rs::Service for JointTrajectory_GetResult {
    type Request = JointTrajectory_GetResult_Request;
    type Response = JointTrajectory_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__JointTrajectory_GetResult() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ParallelGripperCommand_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct ParallelGripperCommand_SendGoal;

impl rosidl_runtime_rs::Service for ParallelGripperCommand_SendGoal {
    type Request = ParallelGripperCommand_SendGoal_Request;
    type Response = ParallelGripperCommand_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ParallelGripperCommand_SendGoal() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ParallelGripperCommand_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__ParallelGripperCommand_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct ParallelGripperCommand_GetResult;

impl rosidl_runtime_rs::Service for ParallelGripperCommand_GetResult {
    type Request = ParallelGripperCommand_GetResult_Request;
    type Response = ParallelGripperCommand_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__ParallelGripperCommand_GetResult() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__PointHead_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__PointHead_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct PointHead_SendGoal;

impl rosidl_runtime_rs::Service for PointHead_SendGoal {
    type Request = PointHead_SendGoal_Request;
    type Response = PointHead_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__PointHead_SendGoal() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__PointHead_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__PointHead_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct PointHead_GetResult;

impl rosidl_runtime_rs::Service for PointHead_GetResult {
    type Request = PointHead_GetResult_Request;
    type Response = PointHead_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__PointHead_GetResult() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__SingleJointPosition_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__SingleJointPosition_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct SingleJointPosition_SendGoal;

impl rosidl_runtime_rs::Service for SingleJointPosition_SendGoal {
    type Request = SingleJointPosition_SendGoal_Request;
    type Response = SingleJointPosition_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__SingleJointPosition_SendGoal() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__SingleJointPosition_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__SingleJointPosition_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct SingleJointPosition_GetResult;

impl rosidl_runtime_rs::Service for SingleJointPosition_GetResult {
    type Request = SingleJointPosition_GetResult_Request;
    type Response = SingleJointPosition_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__action__SingleJointPosition_GetResult() }
    }
}


