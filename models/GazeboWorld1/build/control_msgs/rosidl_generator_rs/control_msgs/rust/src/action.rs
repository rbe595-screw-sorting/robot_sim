
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: super::msg::MotionPrimitiveSequence,

}



impl Default for ExecuteMotionPrimitiveSequence_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_Goal {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: super::msg::MotionPrimitiveSequence::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: super::msg::MotionPrimitiveSequence::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory: super::msg::MotionPrimitiveSequence::from_rmw_message(msg.trajectory),
    }
  }
}


// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_string: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_Result::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_Result {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: msg.error_code,
        error_string: msg.error_string.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_code: msg.error_code,
        error_string: msg.error_string.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: msg.error_code,
      error_string: msg.error_string.to_string(),
    }
  }
}


// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_primitive_index: u8,

}



impl Default for ExecuteMotionPrimitiveSequence_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_Feedback {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_primitive_index: msg.current_primitive_index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      current_primitive_index: msg.current_primitive_index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_primitive_index: msg.current_primitive_index,
    }
  }
}


// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::ExecuteMotionPrimitiveSequence_Feedback,

}



impl Default for ExecuteMotionPrimitiveSequence_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_FeedbackMessage {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::ExecuteMotionPrimitiveSequence_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::ExecuteMotionPrimitiveSequence_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::ExecuteMotionPrimitiveSequence_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: trajectory_msgs::msg::JointTrajectory,

    /// The trajectory for all planar or floating joints (i.e. individual joints with more than one DOF)
    pub multi_dof_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory,

    /// Tolerances for the trajectory.  If the measured joint values fall
    /// outside the tolerances the trajectory goal is aborted.  Any
    /// tolerances that are not specified (by being omitted or set to 0) are
    /// set to the defaults for the action server (often taken from the
    /// parameter server).
    /// Tolerances applied to the joints as the trajectory is executed.  If
    /// violated, the goal aborts with error_code set to
    /// PATH_TOLERANCE_VIOLATED.
    pub path_tolerance: Vec<super::msg::JointTolerance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub component_path_tolerance: Vec<super::msg::JointComponentTolerance>,

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
    pub goal_tolerance: Vec<super::msg::JointTolerance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub component_goal_tolerance: Vec<super::msg::JointComponentTolerance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_time_tolerance: builtin_interfaces::msg::Duration,

}



impl Default for FollowJointTrajectory_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_Goal {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory)).into_owned(),
        multi_dof_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_trajectory)).into_owned(),
        path_tolerance: msg.path_tolerance
          .into_iter()
          .map(|elem| super::msg::JointTolerance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        component_path_tolerance: msg.component_path_tolerance
          .into_iter()
          .map(|elem| super::msg::JointComponentTolerance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        goal_tolerance: msg.goal_tolerance
          .into_iter()
          .map(|elem| super::msg::JointTolerance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        component_goal_tolerance: msg.component_goal_tolerance
          .into_iter()
          .map(|elem| super::msg::JointComponentTolerance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        goal_time_tolerance: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.goal_time_tolerance)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory)).into_owned(),
        multi_dof_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_trajectory)).into_owned(),
        path_tolerance: msg.path_tolerance
          .iter()
          .map(|elem| super::msg::JointTolerance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        component_path_tolerance: msg.component_path_tolerance
          .iter()
          .map(|elem| super::msg::JointComponentTolerance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        goal_tolerance: msg.goal_tolerance
          .iter()
          .map(|elem| super::msg::JointTolerance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        component_goal_tolerance: msg.component_goal_tolerance
          .iter()
          .map(|elem| super::msg::JointComponentTolerance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        goal_time_tolerance: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_time_tolerance)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.trajectory),
      multi_dof_trajectory: trajectory_msgs::msg::MultiDOFJointTrajectory::from_rmw_message(msg.multi_dof_trajectory),
      path_tolerance: msg.path_tolerance
          .into_iter()
          .map(super::msg::JointTolerance::from_rmw_message)
          .collect(),
      component_path_tolerance: msg.component_path_tolerance
          .into_iter()
          .map(super::msg::JointComponentTolerance::from_rmw_message)
          .collect(),
      goal_tolerance: msg.goal_tolerance
          .into_iter()
          .map(super::msg::JointTolerance::from_rmw_message)
          .collect(),
      component_goal_tolerance: msg.component_goal_tolerance
          .into_iter()
          .map(super::msg::JointComponentTolerance::from_rmw_message)
          .collect(),
      goal_time_tolerance: builtin_interfaces::msg::Duration::from_rmw_message(msg.goal_time_tolerance),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub error_string: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_Result::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_Result {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_code: msg.error_code,
        error_string: msg.error_string.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_code: msg.error_code,
        error_string: msg.error_string.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_code: msg.error_code,
      error_string: msg.error_string.to_string(),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_names: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub desired: trajectory_msgs::msg::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub actual: trajectory_msgs::msg::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error: trajectory_msgs::msg::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_joint_names: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_desired: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_actual: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint,

}



impl Default for FollowJointTrajectory_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_Feedback {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        desired: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.desired)).into_owned(),
        actual: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.actual)).into_owned(),
        error: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.error)).into_owned(),
        multi_dof_joint_names: msg.multi_dof_joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        multi_dof_desired: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_desired)).into_owned(),
        multi_dof_actual: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_actual)).into_owned(),
        multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_error)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        desired: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.desired)).into_owned(),
        actual: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.actual)).into_owned(),
        error: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error)).into_owned(),
        multi_dof_joint_names: msg.multi_dof_joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        multi_dof_desired: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_desired)).into_owned(),
        multi_dof_actual: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_actual)).into_owned(),
        multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_error)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      desired: trajectory_msgs::msg::JointTrajectoryPoint::from_rmw_message(msg.desired),
      actual: trajectory_msgs::msg::JointTrajectoryPoint::from_rmw_message(msg.actual),
      error: trajectory_msgs::msg::JointTrajectoryPoint::from_rmw_message(msg.error),
      multi_dof_joint_names: msg.multi_dof_joint_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      multi_dof_desired: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_rmw_message(msg.multi_dof_desired),
      multi_dof_actual: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_rmw_message(msg.multi_dof_actual),
      multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_rmw_message(msg.multi_dof_error),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::FollowJointTrajectory_Feedback,

}



impl Default for FollowJointTrajectory_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_FeedbackMessage {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::FollowJointTrajectory_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::FollowJointTrajectory_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::FollowJointTrajectory_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: super::msg::GripperCommand,

}



impl Default for GripperCommand_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Goal {
  type RmwMsg = super::action::rmw::GripperCommand_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: super::msg::GripperCommand::into_rmw_message(std::borrow::Cow::Owned(msg.command)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: super::msg::GripperCommand::into_rmw_message(std::borrow::Cow::Borrowed(&msg.command)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command: super::msg::GripperCommand::from_rmw_message(msg.command),
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_Result::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Result {
  type RmwMsg = super::action::rmw::GripperCommand_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: msg.position,
        effort: msg.effort,
        stalled: msg.stalled,
        reached_goal: msg.reached_goal,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      position: msg.position,
      effort: msg.effort,
      stalled: msg.stalled,
      reached_goal: msg.reached_goal,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: msg.position,
      effort: msg.effort,
      stalled: msg.stalled,
      reached_goal: msg.reached_goal,
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Feedback {
  type RmwMsg = super::action::rmw::GripperCommand_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: msg.position,
        effort: msg.effort,
        stalled: msg.stalled,
        reached_goal: msg.reached_goal,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      position: msg.position,
      effort: msg.effort,
      stalled: msg.stalled,
      reached_goal: msg.reached_goal,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: msg.position,
      effort: msg.effort,
      stalled: msg.stalled,
      reached_goal: msg.reached_goal,
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::GripperCommand_Feedback,

}



impl Default for GripperCommand_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_FeedbackMessage {
  type RmwMsg = super::action::rmw::GripperCommand_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::GripperCommand_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::GripperCommand_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::GripperCommand_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: trajectory_msgs::msg::JointTrajectory,

}



impl Default for JointTrajectory_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_Goal {
  type RmwMsg = super::action::rmw::JointTrajectory_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory: trajectory_msgs::msg::JointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory: trajectory_msgs::msg::JointTrajectory::from_rmw_message(msg.trajectory),
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for JointTrajectory_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_Result::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_Result {
  type RmwMsg = super::action::rmw::JointTrajectory_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for JointTrajectory_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_Feedback {
  type RmwMsg = super::action::rmw::JointTrajectory_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::JointTrajectory_Feedback,

}



impl Default for JointTrajectory_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_FeedbackMessage {
  type RmwMsg = super::action::rmw::JointTrajectory_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::JointTrajectory_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::JointTrajectory_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::JointTrajectory_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: sensor_msgs::msg::JointState,

}



impl Default for ParallelGripperCommand_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_Goal {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(msg.command)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.command)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command: sensor_msgs::msg::JointState::from_rmw_message(msg.command),
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_Result {
    /// The current gripper state.
    pub state: sensor_msgs::msg::JointState,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_Result::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_Result {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
        stalled: msg.stalled,
        reached_goal: msg.reached_goal,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      stalled: msg.stalled,
      reached_goal: msg.reached_goal,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: sensor_msgs::msg::JointState::from_rmw_message(msg.state),
      stalled: msg.stalled,
      reached_goal: msg.reached_goal,
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_Feedback {
    /// The current gripper state.
    pub state: sensor_msgs::msg::JointState,

}



impl Default for ParallelGripperCommand_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_Feedback {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: sensor_msgs::msg::JointState::from_rmw_message(msg.state),
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::ParallelGripperCommand_Feedback,

}



impl Default for ParallelGripperCommand_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_FeedbackMessage {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::ParallelGripperCommand_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::ParallelGripperCommand_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::ParallelGripperCommand_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to control_msgs__action__PointHead_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target: geometry_msgs::msg::PointStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pointing_axis: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pointing_frame: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub min_duration: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_velocity: f64,

}



impl Default for PointHead_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_Goal {
  type RmwMsg = super::action::rmw::PointHead_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target: geometry_msgs::msg::PointStamped::into_rmw_message(std::borrow::Cow::Owned(msg.target)).into_owned(),
        pointing_axis: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.pointing_axis)).into_owned(),
        pointing_frame: msg.pointing_frame.as_str().into(),
        min_duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.min_duration)).into_owned(),
        max_velocity: msg.max_velocity,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target: geometry_msgs::msg::PointStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target)).into_owned(),
        pointing_axis: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pointing_axis)).into_owned(),
        pointing_frame: msg.pointing_frame.as_str().into(),
        min_duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.min_duration)).into_owned(),
      max_velocity: msg.max_velocity,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      target: geometry_msgs::msg::PointStamped::from_rmw_message(msg.target),
      pointing_axis: geometry_msgs::msg::Vector3::from_rmw_message(msg.pointing_axis),
      pointing_frame: msg.pointing_frame.to_string(),
      min_duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.min_duration),
      max_velocity: msg.max_velocity,
    }
  }
}


// Corresponds to control_msgs__action__PointHead_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for PointHead_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_Result::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_Result {
  type RmwMsg = super::action::rmw::PointHead_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to control_msgs__action__PointHead_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pointing_angle_error: f64,

}



impl Default for PointHead_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_Feedback {
  type RmwMsg = super::action::rmw::PointHead_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pointing_angle_error: msg.pointing_angle_error,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      pointing_angle_error: msg.pointing_angle_error,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pointing_angle_error: msg.pointing_angle_error,
    }
  }
}


// Corresponds to control_msgs__action__PointHead_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::PointHead_Feedback,

}



impl Default for PointHead_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_FeedbackMessage {
  type RmwMsg = super::action::rmw::PointHead_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::PointHead_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::PointHead_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::PointHead_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub min_duration: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_velocity: f64,

}



impl Default for SingleJointPosition_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_Goal {
  type RmwMsg = super::action::rmw::SingleJointPosition_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: msg.position,
        min_duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.min_duration)).into_owned(),
        max_velocity: msg.max_velocity,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      position: msg.position,
        min_duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.min_duration)).into_owned(),
      max_velocity: msg.max_velocity,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: msg.position,
      min_duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.min_duration),
      max_velocity: msg.max_velocity,
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SingleJointPosition_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_Result::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_Result {
  type RmwMsg = super::action::rmw::SingleJointPosition_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_Feedback {
  type RmwMsg = super::action::rmw::SingleJointPosition_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        position: msg.position,
        velocity: msg.velocity,
        error: msg.error,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      position: msg.position,
      velocity: msg.velocity,
      error: msg.error,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      position: msg.position,
      velocity: msg.velocity,
      error: msg.error,
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::SingleJointPosition_Feedback,

}



impl Default for SingleJointPosition_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_FeedbackMessage {
  type RmwMsg = super::action::rmw::SingleJointPosition_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::SingleJointPosition_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::SingleJointPosition_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::SingleJointPosition_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::ExecuteMotionPrimitiveSequence_Goal,

}



impl Default for ExecuteMotionPrimitiveSequence_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_SendGoal_Request {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::ExecuteMotionPrimitiveSequence_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::ExecuteMotionPrimitiveSequence_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::ExecuteMotionPrimitiveSequence_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for ExecuteMotionPrimitiveSequence_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_SendGoal_Response {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for ExecuteMotionPrimitiveSequence_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_GetResult_Request {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecuteMotionPrimitiveSequence_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::ExecuteMotionPrimitiveSequence_Result,

}



impl Default for ExecuteMotionPrimitiveSequence_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ExecuteMotionPrimitiveSequence_GetResult_Response {
  type RmwMsg = super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::ExecuteMotionPrimitiveSequence_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::ExecuteMotionPrimitiveSequence_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::ExecuteMotionPrimitiveSequence_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::FollowJointTrajectory_Goal,

}



impl Default for FollowJointTrajectory_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_SendGoal_Request {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::FollowJointTrajectory_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::FollowJointTrajectory_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::FollowJointTrajectory_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for FollowJointTrajectory_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_SendGoal_Response {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for FollowJointTrajectory_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_GetResult_Request {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to control_msgs__action__FollowJointTrajectory_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FollowJointTrajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::FollowJointTrajectory_Result,

}



impl Default for FollowJointTrajectory_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::FollowJointTrajectory_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FollowJointTrajectory_GetResult_Response {
  type RmwMsg = super::action::rmw::FollowJointTrajectory_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::FollowJointTrajectory_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::FollowJointTrajectory_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::FollowJointTrajectory_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::GripperCommand_Goal,

}



impl Default for GripperCommand_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_SendGoal_Request {
  type RmwMsg = super::action::rmw::GripperCommand_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::GripperCommand_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::GripperCommand_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::GripperCommand_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for GripperCommand_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_SendGoal_Response {
  type RmwMsg = super::action::rmw::GripperCommand_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for GripperCommand_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_GetResult_Request {
  type RmwMsg = super::action::rmw::GripperCommand_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to control_msgs__action__GripperCommand_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::GripperCommand_Result,

}



impl Default for GripperCommand_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::GripperCommand_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_GetResult_Response {
  type RmwMsg = super::action::rmw::GripperCommand_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::GripperCommand_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::GripperCommand_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::GripperCommand_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::JointTrajectory_Goal,

}



impl Default for JointTrajectory_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_SendGoal_Request {
  type RmwMsg = super::action::rmw::JointTrajectory_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::JointTrajectory_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::JointTrajectory_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::JointTrajectory_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for JointTrajectory_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_SendGoal_Response {
  type RmwMsg = super::action::rmw::JointTrajectory_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for JointTrajectory_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_GetResult_Request {
  type RmwMsg = super::action::rmw::JointTrajectory_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to control_msgs__action__JointTrajectory_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectory_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::JointTrajectory_Result,

}



impl Default for JointTrajectory_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointTrajectory_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectory_GetResult_Response {
  type RmwMsg = super::action::rmw::JointTrajectory_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::JointTrajectory_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::JointTrajectory_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::JointTrajectory_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::ParallelGripperCommand_Goal,

}



impl Default for ParallelGripperCommand_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_SendGoal_Request {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::ParallelGripperCommand_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::ParallelGripperCommand_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::ParallelGripperCommand_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for ParallelGripperCommand_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_SendGoal_Response {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for ParallelGripperCommand_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_GetResult_Request {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to control_msgs__action__ParallelGripperCommand_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParallelGripperCommand_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::ParallelGripperCommand_Result,

}



impl Default for ParallelGripperCommand_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::ParallelGripperCommand_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ParallelGripperCommand_GetResult_Response {
  type RmwMsg = super::action::rmw::ParallelGripperCommand_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::ParallelGripperCommand_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::ParallelGripperCommand_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::ParallelGripperCommand_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to control_msgs__action__PointHead_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::PointHead_Goal,

}



impl Default for PointHead_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_SendGoal_Request {
  type RmwMsg = super::action::rmw::PointHead_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::PointHead_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::PointHead_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::PointHead_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to control_msgs__action__PointHead_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for PointHead_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_SendGoal_Response {
  type RmwMsg = super::action::rmw::PointHead_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to control_msgs__action__PointHead_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for PointHead_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_GetResult_Request {
  type RmwMsg = super::action::rmw::PointHead_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to control_msgs__action__PointHead_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointHead_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::PointHead_Result,

}



impl Default for PointHead_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::PointHead_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PointHead_GetResult_Response {
  type RmwMsg = super::action::rmw::PointHead_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::PointHead_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::PointHead_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::PointHead_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::SingleJointPosition_Goal,

}



impl Default for SingleJointPosition_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_SendGoal_Request {
  type RmwMsg = super::action::rmw::SingleJointPosition_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::SingleJointPosition_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::SingleJointPosition_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::SingleJointPosition_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for SingleJointPosition_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_SendGoal_Response {
  type RmwMsg = super::action::rmw::SingleJointPosition_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for SingleJointPosition_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_GetResult_Request {
  type RmwMsg = super::action::rmw::SingleJointPosition_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to control_msgs__action__SingleJointPosition_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleJointPosition_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::SingleJointPosition_Result,

}



impl Default for SingleJointPosition_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::SingleJointPosition_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SingleJointPosition_GetResult_Response {
  type RmwMsg = super::action::rmw::SingleJointPosition_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::SingleJointPosition_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::SingleJointPosition_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::SingleJointPosition_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__ExecuteMotionPrimitiveSequence
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecuteMotionPrimitiveSequence;

impl rosidl_runtime_rs::Action for ExecuteMotionPrimitiveSequence {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = ExecuteMotionPrimitiveSequence_Goal;

  /// The result message defined in the action definition.
  type Result = ExecuteMotionPrimitiveSequence_Result;

  /// The feedback message defined in the action definition.
  type Feedback = ExecuteMotionPrimitiveSequence_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::ExecuteMotionPrimitiveSequence_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::ExecuteMotionPrimitiveSequence_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::ExecuteMotionPrimitiveSequence_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__ExecuteMotionPrimitiveSequence() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::ExecuteMotionPrimitiveSequence_Goal,
  ) -> super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Request {
   super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::ExecuteMotionPrimitiveSequence_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Response {
   super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::ExecuteMotionPrimitiveSequence_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::ExecuteMotionPrimitiveSequence_Feedback,
  ) -> super::action::rmw::ExecuteMotionPrimitiveSequence_FeedbackMessage {
    let mut message = super::action::rmw::ExecuteMotionPrimitiveSequence_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::ExecuteMotionPrimitiveSequence_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::ExecuteMotionPrimitiveSequence_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Request {
   super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::ExecuteMotionPrimitiveSequence_Result,
  ) -> super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Response {
   super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::ExecuteMotionPrimitiveSequence_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::ExecuteMotionPrimitiveSequence_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__FollowJointTrajectory() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__FollowJointTrajectory
#[allow(missing_docs, non_camel_case_types)]
pub struct FollowJointTrajectory;

impl rosidl_runtime_rs::Action for FollowJointTrajectory {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = FollowJointTrajectory_Goal;

  /// The result message defined in the action definition.
  type Result = FollowJointTrajectory_Result;

  /// The feedback message defined in the action definition.
  type Feedback = FollowJointTrajectory_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::FollowJointTrajectory_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::FollowJointTrajectory_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::FollowJointTrajectory_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__FollowJointTrajectory() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::FollowJointTrajectory_Goal,
  ) -> super::action::rmw::FollowJointTrajectory_SendGoal_Request {
   super::action::rmw::FollowJointTrajectory_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::FollowJointTrajectory_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::FollowJointTrajectory_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::FollowJointTrajectory_SendGoal_Response {
   super::action::rmw::FollowJointTrajectory_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::FollowJointTrajectory_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::FollowJointTrajectory_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::FollowJointTrajectory_Feedback,
  ) -> super::action::rmw::FollowJointTrajectory_FeedbackMessage {
    let mut message = super::action::rmw::FollowJointTrajectory_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::FollowJointTrajectory_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::FollowJointTrajectory_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::FollowJointTrajectory_GetResult_Request {
   super::action::rmw::FollowJointTrajectory_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::FollowJointTrajectory_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::FollowJointTrajectory_Result,
  ) -> super::action::rmw::FollowJointTrajectory_GetResult_Response {
   super::action::rmw::FollowJointTrajectory_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::FollowJointTrajectory_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::FollowJointTrajectory_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__GripperCommand() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__GripperCommand
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperCommand;

impl rosidl_runtime_rs::Action for GripperCommand {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = GripperCommand_Goal;

  /// The result message defined in the action definition.
  type Result = GripperCommand_Result;

  /// The feedback message defined in the action definition.
  type Feedback = GripperCommand_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::GripperCommand_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::GripperCommand_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::GripperCommand_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__GripperCommand() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::GripperCommand_Goal,
  ) -> super::action::rmw::GripperCommand_SendGoal_Request {
   super::action::rmw::GripperCommand_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::GripperCommand_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::GripperCommand_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::GripperCommand_SendGoal_Response {
   super::action::rmw::GripperCommand_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::GripperCommand_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::GripperCommand_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::GripperCommand_Feedback,
  ) -> super::action::rmw::GripperCommand_FeedbackMessage {
    let mut message = super::action::rmw::GripperCommand_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::GripperCommand_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::GripperCommand_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::GripperCommand_GetResult_Request {
   super::action::rmw::GripperCommand_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::GripperCommand_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::GripperCommand_Result,
  ) -> super::action::rmw::GripperCommand_GetResult_Response {
   super::action::rmw::GripperCommand_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::GripperCommand_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::GripperCommand_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__JointTrajectory() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__JointTrajectory
#[allow(missing_docs, non_camel_case_types)]
pub struct JointTrajectory;

impl rosidl_runtime_rs::Action for JointTrajectory {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = JointTrajectory_Goal;

  /// The result message defined in the action definition.
  type Result = JointTrajectory_Result;

  /// The feedback message defined in the action definition.
  type Feedback = JointTrajectory_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::JointTrajectory_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::JointTrajectory_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::JointTrajectory_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__JointTrajectory() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::JointTrajectory_Goal,
  ) -> super::action::rmw::JointTrajectory_SendGoal_Request {
   super::action::rmw::JointTrajectory_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::JointTrajectory_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::JointTrajectory_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::JointTrajectory_SendGoal_Response {
   super::action::rmw::JointTrajectory_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::JointTrajectory_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::JointTrajectory_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::JointTrajectory_Feedback,
  ) -> super::action::rmw::JointTrajectory_FeedbackMessage {
    let mut message = super::action::rmw::JointTrajectory_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::JointTrajectory_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::JointTrajectory_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::JointTrajectory_GetResult_Request {
   super::action::rmw::JointTrajectory_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::JointTrajectory_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::JointTrajectory_Result,
  ) -> super::action::rmw::JointTrajectory_GetResult_Response {
   super::action::rmw::JointTrajectory_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::JointTrajectory_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::JointTrajectory_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__ParallelGripperCommand() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__ParallelGripperCommand
#[allow(missing_docs, non_camel_case_types)]
pub struct ParallelGripperCommand;

impl rosidl_runtime_rs::Action for ParallelGripperCommand {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = ParallelGripperCommand_Goal;

  /// The result message defined in the action definition.
  type Result = ParallelGripperCommand_Result;

  /// The feedback message defined in the action definition.
  type Feedback = ParallelGripperCommand_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::ParallelGripperCommand_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::ParallelGripperCommand_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::ParallelGripperCommand_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__ParallelGripperCommand() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::ParallelGripperCommand_Goal,
  ) -> super::action::rmw::ParallelGripperCommand_SendGoal_Request {
   super::action::rmw::ParallelGripperCommand_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::ParallelGripperCommand_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::ParallelGripperCommand_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::ParallelGripperCommand_SendGoal_Response {
   super::action::rmw::ParallelGripperCommand_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::ParallelGripperCommand_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::ParallelGripperCommand_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::ParallelGripperCommand_Feedback,
  ) -> super::action::rmw::ParallelGripperCommand_FeedbackMessage {
    let mut message = super::action::rmw::ParallelGripperCommand_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::ParallelGripperCommand_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::ParallelGripperCommand_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::ParallelGripperCommand_GetResult_Request {
   super::action::rmw::ParallelGripperCommand_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::ParallelGripperCommand_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::ParallelGripperCommand_Result,
  ) -> super::action::rmw::ParallelGripperCommand_GetResult_Response {
   super::action::rmw::ParallelGripperCommand_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::ParallelGripperCommand_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::ParallelGripperCommand_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__PointHead() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__PointHead
#[allow(missing_docs, non_camel_case_types)]
pub struct PointHead;

impl rosidl_runtime_rs::Action for PointHead {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = PointHead_Goal;

  /// The result message defined in the action definition.
  type Result = PointHead_Result;

  /// The feedback message defined in the action definition.
  type Feedback = PointHead_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::PointHead_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::PointHead_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::PointHead_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__PointHead() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::PointHead_Goal,
  ) -> super::action::rmw::PointHead_SendGoal_Request {
   super::action::rmw::PointHead_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::PointHead_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::PointHead_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::PointHead_SendGoal_Response {
   super::action::rmw::PointHead_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::PointHead_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::PointHead_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::PointHead_Feedback,
  ) -> super::action::rmw::PointHead_FeedbackMessage {
    let mut message = super::action::rmw::PointHead_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::PointHead_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::PointHead_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::PointHead_GetResult_Request {
   super::action::rmw::PointHead_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::PointHead_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::PointHead_Result,
  ) -> super::action::rmw::PointHead_GetResult_Response {
   super::action::rmw::PointHead_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::PointHead_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::PointHead_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__SingleJointPosition() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__action__SingleJointPosition
#[allow(missing_docs, non_camel_case_types)]
pub struct SingleJointPosition;

impl rosidl_runtime_rs::Action for SingleJointPosition {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = SingleJointPosition_Goal;

  /// The result message defined in the action definition.
  type Result = SingleJointPosition_Result;

  /// The feedback message defined in the action definition.
  type Feedback = SingleJointPosition_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::SingleJointPosition_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::SingleJointPosition_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::SingleJointPosition_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__control_msgs__action__SingleJointPosition() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::SingleJointPosition_Goal,
  ) -> super::action::rmw::SingleJointPosition_SendGoal_Request {
   super::action::rmw::SingleJointPosition_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::SingleJointPosition_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::SingleJointPosition_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::SingleJointPosition_SendGoal_Response {
   super::action::rmw::SingleJointPosition_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::SingleJointPosition_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::SingleJointPosition_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::SingleJointPosition_Feedback,
  ) -> super::action::rmw::SingleJointPosition_FeedbackMessage {
    let mut message = super::action::rmw::SingleJointPosition_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::SingleJointPosition_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::SingleJointPosition_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::SingleJointPosition_GetResult_Request {
   super::action::rmw::SingleJointPosition_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::SingleJointPosition_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::SingleJointPosition_Result,
  ) -> super::action::rmw::SingleJointPosition_GetResult_Response {
   super::action::rmw::SingleJointPosition_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::SingleJointPosition_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::SingleJointPosition_Result,
  ) {
    (response.status, response.result)
  }
}


