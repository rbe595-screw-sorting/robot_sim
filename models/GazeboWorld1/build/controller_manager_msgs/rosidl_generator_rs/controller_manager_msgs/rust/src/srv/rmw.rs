#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureController_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ConfigureController_Request__init(msg: *mut ConfigureController_Request) -> bool;
    fn controller_manager_msgs__srv__ConfigureController_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConfigureController_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ConfigureController_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConfigureController_Request>);
    fn controller_manager_msgs__srv__ConfigureController_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConfigureController_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ConfigureController_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ConfigureController_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConfigureController_Request {
    /// To configure a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// configured or not.
    pub name: rosidl_runtime_rs::String,

}



impl Default for ConfigureController_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ConfigureController_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ConfigureController_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConfigureController_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ConfigureController_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ConfigureController_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ConfigureController_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConfigureController_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConfigureController_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ConfigureController_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureController_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureController_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ConfigureController_Response__init(msg: *mut ConfigureController_Response) -> bool;
    fn controller_manager_msgs__srv__ConfigureController_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConfigureController_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ConfigureController_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConfigureController_Response>);
    fn controller_manager_msgs__srv__ConfigureController_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConfigureController_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ConfigureController_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ConfigureController_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConfigureController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for ConfigureController_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ConfigureController_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ConfigureController_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConfigureController_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ConfigureController_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ConfigureController_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ConfigureController_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConfigureController_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConfigureController_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ConfigureController_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureController_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllers_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListControllers_Request__init(msg: *mut ListControllers_Request) -> bool;
    fn controller_manager_msgs__srv__ListControllers_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListControllers_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListControllers_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListControllers_Request>);
    fn controller_manager_msgs__srv__ListControllers_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListControllers_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListControllers_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListControllers_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllers_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListControllers_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListControllers_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListControllers_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListControllers_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllers_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllers_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllers_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListControllers_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListControllers_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListControllers_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllers_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllers_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListControllers_Response__init(msg: *mut ListControllers_Response) -> bool;
    fn controller_manager_msgs__srv__ListControllers_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListControllers_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListControllers_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListControllers_Response>);
    fn controller_manager_msgs__srv__ListControllers_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListControllers_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListControllers_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListControllers_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllers_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub controller: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ControllerState>,

}



impl Default for ListControllers_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListControllers_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListControllers_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListControllers_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllers_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllers_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllers_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListControllers_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListControllers_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListControllers_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllers_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllerTypes_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListControllerTypes_Request__init(msg: *mut ListControllerTypes_Request) -> bool;
    fn controller_manager_msgs__srv__ListControllerTypes_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListControllerTypes_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListControllerTypes_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListControllerTypes_Request>);
    fn controller_manager_msgs__srv__ListControllerTypes_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListControllerTypes_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListControllerTypes_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListControllerTypes_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllerTypes_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListControllerTypes_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListControllerTypes_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListControllerTypes_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListControllerTypes_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllerTypes_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllerTypes_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllerTypes_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListControllerTypes_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListControllerTypes_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListControllerTypes_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllerTypes_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllerTypes_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListControllerTypes_Response__init(msg: *mut ListControllerTypes_Response) -> bool;
    fn controller_manager_msgs__srv__ListControllerTypes_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListControllerTypes_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListControllerTypes_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListControllerTypes_Response>);
    fn controller_manager_msgs__srv__ListControllerTypes_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListControllerTypes_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListControllerTypes_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListControllerTypes_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllerTypes_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub types: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub base_classes: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ListControllerTypes_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListControllerTypes_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListControllerTypes_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListControllerTypes_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllerTypes_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllerTypes_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListControllerTypes_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListControllerTypes_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListControllerTypes_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListControllerTypes_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllerTypes_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareComponents_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListHardwareComponents_Request__init(msg: *mut ListHardwareComponents_Request) -> bool;
    fn controller_manager_msgs__srv__ListHardwareComponents_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareComponents_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListHardwareComponents_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareComponents_Request>);
    fn controller_manager_msgs__srv__ListHardwareComponents_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListHardwareComponents_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListHardwareComponents_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListHardwareComponents_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareComponents_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListHardwareComponents_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListHardwareComponents_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListHardwareComponents_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListHardwareComponents_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareComponents_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareComponents_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareComponents_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListHardwareComponents_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListHardwareComponents_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListHardwareComponents_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareComponents_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareComponents_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListHardwareComponents_Response__init(msg: *mut ListHardwareComponents_Response) -> bool;
    fn controller_manager_msgs__srv__ListHardwareComponents_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareComponents_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListHardwareComponents_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareComponents_Response>);
    fn controller_manager_msgs__srv__ListHardwareComponents_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListHardwareComponents_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListHardwareComponents_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListHardwareComponents_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareComponents_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub component: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HardwareComponentState>,

}



impl Default for ListHardwareComponents_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListHardwareComponents_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListHardwareComponents_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListHardwareComponents_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareComponents_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareComponents_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareComponents_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListHardwareComponents_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListHardwareComponents_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListHardwareComponents_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareComponents_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Request__init(msg: *mut ListHardwareInterfaces_Request) -> bool;
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Request>);
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListHardwareInterfaces_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareInterfaces_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListHardwareInterfaces_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListHardwareInterfaces_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListHardwareInterfaces_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListHardwareInterfaces_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListHardwareInterfaces_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListHardwareInterfaces_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListHardwareInterfaces_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Response__init(msg: *mut ListHardwareInterfaces_Response) -> bool;
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Response>);
    fn controller_manager_msgs__srv__ListHardwareInterfaces_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListHardwareInterfaces_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ListHardwareInterfaces_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareInterfaces_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command_interfaces: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HardwareInterface>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_interfaces: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HardwareInterface>,

}



impl Default for ListHardwareInterfaces_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ListHardwareInterfaces_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ListHardwareInterfaces_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListHardwareInterfaces_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListHardwareInterfaces_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListHardwareInterfaces_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ListHardwareInterfaces_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadController_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__LoadController_Request__init(msg: *mut LoadController_Request) -> bool;
    fn controller_manager_msgs__srv__LoadController_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadController_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__LoadController_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadController_Request>);
    fn controller_manager_msgs__srv__LoadController_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadController_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadController_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__LoadController_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadController_Request {
    /// To load a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// constructed and initialized or not.
    pub name: rosidl_runtime_rs::String,

}



impl Default for LoadController_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__LoadController_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__LoadController_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadController_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__LoadController_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__LoadController_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__LoadController_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadController_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadController_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/LoadController_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadController_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadController_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__LoadController_Response__init(msg: *mut LoadController_Response) -> bool;
    fn controller_manager_msgs__srv__LoadController_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadController_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__LoadController_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadController_Response>);
    fn controller_manager_msgs__srv__LoadController_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadController_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadController_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__LoadController_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for LoadController_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__LoadController_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__LoadController_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadController_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__LoadController_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__LoadController_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__LoadController_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadController_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadController_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/LoadController_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadController_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Request__init(msg: *mut ReloadControllerLibraries_Request) -> bool;
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Request>);
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ReloadControllerLibraries_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReloadControllerLibraries_Request {
    /// Reloading libraries only works if there are no controllers loaded. If there
    /// are still some controllers loaded, the reloading will fail.
    /// If this bool is set to true, all loaded controllers will get
    /// killed automatically, and the reloading can succeed.
    pub force_kill: bool,

}



impl Default for ReloadControllerLibraries_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ReloadControllerLibraries_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ReloadControllerLibraries_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ReloadControllerLibraries_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ReloadControllerLibraries_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ReloadControllerLibraries_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ReloadControllerLibraries_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Response__init(msg: *mut ReloadControllerLibraries_Response) -> bool;
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Response>);
    fn controller_manager_msgs__srv__ReloadControllerLibraries_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ReloadControllerLibraries_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__ReloadControllerLibraries_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReloadControllerLibraries_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for ReloadControllerLibraries_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__ReloadControllerLibraries_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__ReloadControllerLibraries_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ReloadControllerLibraries_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ReloadControllerLibraries_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ReloadControllerLibraries_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/ReloadControllerLibraries_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SetHardwareComponentState_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__SetHardwareComponentState_Request__init(msg: *mut SetHardwareComponentState_Request) -> bool;
    fn controller_manager_msgs__srv__SetHardwareComponentState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetHardwareComponentState_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__SetHardwareComponentState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetHardwareComponentState_Request>);
    fn controller_manager_msgs__srv__SetHardwareComponentState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetHardwareComponentState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetHardwareComponentState_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__SetHardwareComponentState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetHardwareComponentState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_state: lifecycle_msgs::msg::rmw::State,

}



impl Default for SetHardwareComponentState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__SetHardwareComponentState_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__SetHardwareComponentState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetHardwareComponentState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SetHardwareComponentState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SetHardwareComponentState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SetHardwareComponentState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetHardwareComponentState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetHardwareComponentState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/SetHardwareComponentState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SetHardwareComponentState_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SetHardwareComponentState_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__SetHardwareComponentState_Response__init(msg: *mut SetHardwareComponentState_Response) -> bool;
    fn controller_manager_msgs__srv__SetHardwareComponentState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetHardwareComponentState_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__SetHardwareComponentState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetHardwareComponentState_Response>);
    fn controller_manager_msgs__srv__SetHardwareComponentState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetHardwareComponentState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetHardwareComponentState_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__SetHardwareComponentState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetHardwareComponentState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: lifecycle_msgs::msg::rmw::State,

}



impl Default for SetHardwareComponentState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__SetHardwareComponentState_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__SetHardwareComponentState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetHardwareComponentState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SetHardwareComponentState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SetHardwareComponentState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SetHardwareComponentState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetHardwareComponentState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetHardwareComponentState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/SetHardwareComponentState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SetHardwareComponentState_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SwitchController_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__SwitchController_Request__init(msg: *mut SwitchController_Request) -> bool;
    fn controller_manager_msgs__srv__SwitchController_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchController_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__SwitchController_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchController_Request>);
    fn controller_manager_msgs__srv__SwitchController_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchController_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchController_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__SwitchController_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchController_Request {
    /// To switch controllers, specify
    ///  * the list of controller names to activate,
    ///  * the list of controller names to deactivate, and
    ///  * the strictness (STRICT, BEST_EFFORT, AUTO, or FORCE_AUTO)
    ///    * STRICT means that switching will fail if anything goes wrong (an invalid
    ///      controller name, a controller that failed to activate, etc.).
    ///    * BEST_EFFORT:
    ///      Transitions are only triggered if the controller is not yet in the target state.
    ///      If it is already in the target state, no error occurs.
    ///      Returns 'false' if all controllers of 'activate_controllers' or 'deactivate_controllers' are not yet loaded or configured,
    ///      or the pending transitions of all existing controllers of 'activate_controllers'
    ///      or 'deactivate_controllers' fail.
    ///      Returns 'true' otherwise.
    ///    * AUTO means that the controller manager will automatically resolve the controller
    ///      chain in order to activate and/or deactivate the specified controllers.
    ///      This is useful in complex systems when you want all dependent controllers to start
    ///      within the same update iteration.
    ///    * FORCE_AUTO means that the controller manager will take all necessary steps to activate
    ///      the specified controllers without requiring you to explicitly list the controllers
    ///      to be deactivated. This is useful when the controller being activated depends on
    ///      another unknown controller that is currently running. The controller manager will
    ///      deactivate any controllers that block the activation of the requested controller,
    ///      following the mutually exclusive joint interface switching principle. For example,
    ///      to activate a controller that uses a joint's position interface, the controller manager
    ///      will automatically deactivate any controllers that use conflicting interfaces for
    ///      the same joint.
    ///  * activate the controllers as soon as their hardware dependencies are ready, will
    ///    wait for all interfaces to be ready otherwise
    ///  * the timeout before aborting pending controllers. Zero for infinite
    /// The return value "ok" indicates if the controllers were switched
    /// successfully or not. The meaning of success depends on the
    /// specified strictness.
    /// The return value "message" provides some human-readable information.
    pub activate_controllers: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub deactivate_controllers: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub strictness: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub activate_asap: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timeout: builtin_interfaces::msg::rmw::Duration,

}

impl SwitchController_Request {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BEST_EFFORT: i32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STRICT: i32 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AUTO: i32 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FORCE_AUTO: i32 = 4;

}


impl Default for SwitchController_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__SwitchController_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__SwitchController_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchController_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SwitchController_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SwitchController_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SwitchController_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchController_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchController_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/SwitchController_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SwitchController_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SwitchController_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__SwitchController_Response__init(msg: *mut SwitchController_Response) -> bool;
    fn controller_manager_msgs__srv__SwitchController_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchController_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__SwitchController_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchController_Response>);
    fn controller_manager_msgs__srv__SwitchController_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchController_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchController_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__SwitchController_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SwitchController_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__SwitchController_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__SwitchController_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchController_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SwitchController_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SwitchController_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__SwitchController_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchController_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchController_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/SwitchController_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SwitchController_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__UnloadController_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__UnloadController_Request__init(msg: *mut UnloadController_Request) -> bool;
    fn controller_manager_msgs__srv__UnloadController_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UnloadController_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__UnloadController_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UnloadController_Request>);
    fn controller_manager_msgs__srv__UnloadController_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UnloadController_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UnloadController_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__UnloadController_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UnloadController_Request {
    /// To unload a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// unloaded or not
    pub name: rosidl_runtime_rs::String,

}



impl Default for UnloadController_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__UnloadController_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__UnloadController_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UnloadController_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__UnloadController_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__UnloadController_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__UnloadController_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UnloadController_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UnloadController_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/UnloadController_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__UnloadController_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__UnloadController_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__UnloadController_Response__init(msg: *mut UnloadController_Response) -> bool;
    fn controller_manager_msgs__srv__UnloadController_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UnloadController_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__UnloadController_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UnloadController_Response>);
    fn controller_manager_msgs__srv__UnloadController_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UnloadController_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UnloadController_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__UnloadController_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UnloadController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for UnloadController_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__UnloadController_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__UnloadController_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UnloadController_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__UnloadController_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__UnloadController_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__UnloadController_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UnloadController_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UnloadController_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/UnloadController_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__UnloadController_Response() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__CleanupController_Request() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__CleanupController_Request__init(msg: *mut CleanupController_Request) -> bool;
    fn controller_manager_msgs__srv__CleanupController_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CleanupController_Request>, size: usize) -> bool;
    fn controller_manager_msgs__srv__CleanupController_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CleanupController_Request>);
    fn controller_manager_msgs__srv__CleanupController_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CleanupController_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CleanupController_Request>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__CleanupController_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CleanupController_Request {
    /// To cleanup a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// cleaned up or not
    pub name: rosidl_runtime_rs::String,

}



impl Default for CleanupController_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__CleanupController_Request__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__CleanupController_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CleanupController_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__CleanupController_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__CleanupController_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__CleanupController_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CleanupController_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CleanupController_Request where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/CleanupController_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__CleanupController_Request() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__CleanupController_Response() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__srv__CleanupController_Response__init(msg: *mut CleanupController_Response) -> bool;
    fn controller_manager_msgs__srv__CleanupController_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CleanupController_Response>, size: usize) -> bool;
    fn controller_manager_msgs__srv__CleanupController_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CleanupController_Response>);
    fn controller_manager_msgs__srv__CleanupController_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CleanupController_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CleanupController_Response>) -> bool;
}

// Corresponds to controller_manager_msgs__srv__CleanupController_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CleanupController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for CleanupController_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__srv__CleanupController_Response__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__srv__CleanupController_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CleanupController_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__CleanupController_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__CleanupController_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__srv__CleanupController_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CleanupController_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CleanupController_Response where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/srv/CleanupController_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__CleanupController_Response() }
  }
}






#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ConfigureController() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__ConfigureController
#[allow(missing_docs, non_camel_case_types)]
pub struct ConfigureController;

impl rosidl_runtime_rs::Service for ConfigureController {
    type Request = ConfigureController_Request;
    type Response = ConfigureController_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ConfigureController() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListControllers() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__ListControllers
#[allow(missing_docs, non_camel_case_types)]
pub struct ListControllers;

impl rosidl_runtime_rs::Service for ListControllers {
    type Request = ListControllers_Request;
    type Response = ListControllers_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListControllers() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListControllerTypes() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__ListControllerTypes
#[allow(missing_docs, non_camel_case_types)]
pub struct ListControllerTypes;

impl rosidl_runtime_rs::Service for ListControllerTypes {
    type Request = ListControllerTypes_Request;
    type Response = ListControllerTypes_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListControllerTypes() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListHardwareComponents() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__ListHardwareComponents
#[allow(missing_docs, non_camel_case_types)]
pub struct ListHardwareComponents;

impl rosidl_runtime_rs::Service for ListHardwareComponents {
    type Request = ListHardwareComponents_Request;
    type Response = ListHardwareComponents_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListHardwareComponents() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__ListHardwareInterfaces
#[allow(missing_docs, non_camel_case_types)]
pub struct ListHardwareInterfaces;

impl rosidl_runtime_rs::Service for ListHardwareInterfaces {
    type Request = ListHardwareInterfaces_Request;
    type Response = ListHardwareInterfaces_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__LoadController() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__LoadController
#[allow(missing_docs, non_camel_case_types)]
pub struct LoadController;

impl rosidl_runtime_rs::Service for LoadController {
    type Request = LoadController_Request;
    type Response = LoadController_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__LoadController() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__ReloadControllerLibraries
#[allow(missing_docs, non_camel_case_types)]
pub struct ReloadControllerLibraries;

impl rosidl_runtime_rs::Service for ReloadControllerLibraries {
    type Request = ReloadControllerLibraries_Request;
    type Response = ReloadControllerLibraries_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__SetHardwareComponentState() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__SetHardwareComponentState
#[allow(missing_docs, non_camel_case_types)]
pub struct SetHardwareComponentState;

impl rosidl_runtime_rs::Service for SetHardwareComponentState {
    type Request = SetHardwareComponentState_Request;
    type Response = SetHardwareComponentState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__SetHardwareComponentState() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__SwitchController() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__SwitchController
#[allow(missing_docs, non_camel_case_types)]
pub struct SwitchController;

impl rosidl_runtime_rs::Service for SwitchController {
    type Request = SwitchController_Request;
    type Response = SwitchController_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__SwitchController() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__UnloadController() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__UnloadController
#[allow(missing_docs, non_camel_case_types)]
pub struct UnloadController;

impl rosidl_runtime_rs::Service for UnloadController {
    type Request = UnloadController_Request;
    type Response = UnloadController_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__UnloadController() }
    }
}




#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__CleanupController() -> *const std::ffi::c_void;
}

// Corresponds to controller_manager_msgs__srv__CleanupController
#[allow(missing_docs, non_camel_case_types)]
pub struct CleanupController;

impl rosidl_runtime_rs::Service for CleanupController {
    type Request = CleanupController_Request;
    type Response = CleanupController_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__CleanupController() }
    }
}


