#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryCalibrationState_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__srv__QueryCalibrationState_Request__init(msg: *mut QueryCalibrationState_Request) -> bool;
    fn control_msgs__srv__QueryCalibrationState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<QueryCalibrationState_Request>, size: usize) -> bool;
    fn control_msgs__srv__QueryCalibrationState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<QueryCalibrationState_Request>);
    fn control_msgs__srv__QueryCalibrationState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<QueryCalibrationState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<QueryCalibrationState_Request>) -> bool;
}

// Corresponds to control_msgs__srv__QueryCalibrationState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryCalibrationState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for QueryCalibrationState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__srv__QueryCalibrationState_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__srv__QueryCalibrationState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for QueryCalibrationState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryCalibrationState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryCalibrationState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryCalibrationState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for QueryCalibrationState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for QueryCalibrationState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/srv/QueryCalibrationState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryCalibrationState_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryCalibrationState_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__srv__QueryCalibrationState_Response__init(msg: *mut QueryCalibrationState_Response) -> bool;
    fn control_msgs__srv__QueryCalibrationState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<QueryCalibrationState_Response>, size: usize) -> bool;
    fn control_msgs__srv__QueryCalibrationState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<QueryCalibrationState_Response>);
    fn control_msgs__srv__QueryCalibrationState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<QueryCalibrationState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<QueryCalibrationState_Response>) -> bool;
}

// Corresponds to control_msgs__srv__QueryCalibrationState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryCalibrationState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_calibrated: bool,

}



impl Default for QueryCalibrationState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__srv__QueryCalibrationState_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__srv__QueryCalibrationState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for QueryCalibrationState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryCalibrationState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryCalibrationState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryCalibrationState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for QueryCalibrationState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for QueryCalibrationState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/srv/QueryCalibrationState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryCalibrationState_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryTrajectoryState_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__srv__QueryTrajectoryState_Request__init(msg: *mut QueryTrajectoryState_Request) -> bool;
    fn control_msgs__srv__QueryTrajectoryState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<QueryTrajectoryState_Request>, size: usize) -> bool;
    fn control_msgs__srv__QueryTrajectoryState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<QueryTrajectoryState_Request>);
    fn control_msgs__srv__QueryTrajectoryState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<QueryTrajectoryState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<QueryTrajectoryState_Request>) -> bool;
}

// Corresponds to control_msgs__srv__QueryTrajectoryState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryTrajectoryState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub time: builtin_interfaces::msg::rmw::Time,

}



impl Default for QueryTrajectoryState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__srv__QueryTrajectoryState_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__srv__QueryTrajectoryState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for QueryTrajectoryState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryTrajectoryState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryTrajectoryState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryTrajectoryState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for QueryTrajectoryState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for QueryTrajectoryState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/srv/QueryTrajectoryState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryTrajectoryState_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryTrajectoryState_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__srv__QueryTrajectoryState_Response__init(msg: *mut QueryTrajectoryState_Response) -> bool;
    fn control_msgs__srv__QueryTrajectoryState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<QueryTrajectoryState_Response>, size: usize) -> bool;
    fn control_msgs__srv__QueryTrajectoryState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<QueryTrajectoryState_Response>);
    fn control_msgs__srv__QueryTrajectoryState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<QueryTrajectoryState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<QueryTrajectoryState_Response>) -> bool;
}

// Corresponds to control_msgs__srv__QueryTrajectoryState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QueryTrajectoryState_Response {
    /// indicate successful run of triggered service
    pub success: bool,

    /// informational, e.g. for error messages
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acceleration: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for QueryTrajectoryState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__srv__QueryTrajectoryState_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__srv__QueryTrajectoryState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for QueryTrajectoryState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryTrajectoryState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryTrajectoryState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__QueryTrajectoryState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for QueryTrajectoryState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for QueryTrajectoryState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/srv/QueryTrajectoryState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__QueryTrajectoryState_Response() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__SetOdometry_Request() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__srv__SetOdometry_Request__init(msg: *mut SetOdometry_Request) -> bool;
    fn control_msgs__srv__SetOdometry_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOdometry_Request>, size: usize) -> bool;
    fn control_msgs__srv__SetOdometry_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOdometry_Request>);
    fn control_msgs__srv__SetOdometry_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOdometry_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOdometry_Request>) -> bool;
}

// Corresponds to control_msgs__srv__SetOdometry_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__srv__SetOdometry_Request__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__srv__SetOdometry_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOdometry_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__SetOdometry_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__SetOdometry_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__SetOdometry_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOdometry_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOdometry_Request where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/srv/SetOdometry_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__SetOdometry_Request() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__SetOdometry_Response() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__srv__SetOdometry_Response__init(msg: *mut SetOdometry_Response) -> bool;
    fn control_msgs__srv__SetOdometry_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOdometry_Response>, size: usize) -> bool;
    fn control_msgs__srv__SetOdometry_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOdometry_Response>);
    fn control_msgs__srv__SetOdometry_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOdometry_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOdometry_Response>) -> bool;
}

// Corresponds to control_msgs__srv__SetOdometry_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOdometry_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetOdometry_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__srv__SetOdometry_Response__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__srv__SetOdometry_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOdometry_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__SetOdometry_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__SetOdometry_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__srv__SetOdometry_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOdometry_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOdometry_Response where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/srv/SetOdometry_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__srv__SetOdometry_Response() }
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


