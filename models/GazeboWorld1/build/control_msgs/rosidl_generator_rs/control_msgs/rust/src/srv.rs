#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to control_msgs__srv__QueryCalibrationState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryCalibrationState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for QueryCalibrationState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::QueryCalibrationState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for QueryCalibrationState_Request {
  type RmwMsg = super::srv::rmw::QueryCalibrationState_Request;

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


// Corresponds to control_msgs__srv__QueryCalibrationState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryCalibrationState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_calibrated: bool,

}



impl Default for QueryCalibrationState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::QueryCalibrationState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for QueryCalibrationState_Response {
  type RmwMsg = super::srv::rmw::QueryCalibrationState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        is_calibrated: msg.is_calibrated,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      is_calibrated: msg.is_calibrated,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      is_calibrated: msg.is_calibrated,
    }
  }
}


// Corresponds to control_msgs__srv__QueryTrajectoryState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryTrajectoryState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time: builtin_interfaces::msg::Time,

}



impl Default for QueryTrajectoryState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::QueryTrajectoryState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for QueryTrajectoryState_Request {
  type RmwMsg = super::srv::rmw::QueryTrajectoryState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.time)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.time)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      time: builtin_interfaces::msg::Time::from_rmw_message(msg.time),
    }
  }
}


// Corresponds to control_msgs__srv__QueryTrajectoryState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryTrajectoryState_Response {
    /// indicate successful run of triggered service
    pub success: bool,

    /// informational, e.g. for error messages
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub name: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acceleration: Vec<f64>,

}



impl Default for QueryTrajectoryState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::QueryTrajectoryState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for QueryTrajectoryState_Response {
  type RmwMsg = super::srv::rmw::QueryTrajectoryState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
        name: msg.name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        position: msg.position.as_slice().into(),
        velocity: msg.velocity.as_slice().into(),
        acceleration: msg.acceleration.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
        name: msg.name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        position: msg.position.as_slice().into(),
        velocity: msg.velocity.as_slice().into(),
        acceleration: msg.acceleration.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
      name: msg.name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      position: msg.position.into(),
      velocity: msg.velocity.into(),
      acceleration: msg.acceleration.into(),
    }
  }
}


// Corresponds to control_msgs__srv__SetOdometry_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOdometry_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub roll: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,

}



impl Default for SetOdometry_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOdometry_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetOdometry_Request {
  type RmwMsg = super::srv::rmw::SetOdometry_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        roll: msg.roll,
        pitch: msg.pitch,
        yaw: msg.yaw,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
    }
  }
}


// Corresponds to control_msgs__srv__SetOdometry_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOdometry_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetOdometry_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOdometry_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetOdometry_Response {
  type RmwMsg = super::srv::rmw::SetOdometry_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}






#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__srv__QueryCalibrationState() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__srv__QueryCalibrationState
#[allow(missing_docs, non_camel_case_types)]
pub struct QueryCalibrationState;

impl rosidl_runtime_rs::Service for QueryCalibrationState {
    type Request = QueryCalibrationState_Request;
    type Response = QueryCalibrationState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__srv__QueryCalibrationState() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__srv__QueryTrajectoryState() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__srv__QueryTrajectoryState
#[allow(missing_docs, non_camel_case_types)]
pub struct QueryTrajectoryState;

impl rosidl_runtime_rs::Service for QueryTrajectoryState {
    type Request = QueryTrajectoryState_Request;
    type Response = QueryTrajectoryState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__srv__QueryTrajectoryState() }
    }
}




#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__control_msgs__srv__SetOdometry() -> *const std::ffi::c_void;
}

// Corresponds to control_msgs__srv__SetOdometry
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOdometry;

impl rosidl_runtime_rs::Service for SetOdometry {
    type Request = SetOdometry_Request;
    type Response = SetOdometry_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__control_msgs__srv__SetOdometry() }
    }
}


