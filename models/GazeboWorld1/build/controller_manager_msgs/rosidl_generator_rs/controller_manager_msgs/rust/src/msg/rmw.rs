#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__ControllerState() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__msg__ControllerState__init(msg: *mut ControllerState) -> bool;
    fn controller_manager_msgs__msg__ControllerState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControllerState>, size: usize) -> bool;
    fn controller_manager_msgs__msg__ControllerState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControllerState>);
    fn controller_manager_msgs__msg__ControllerState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControllerState>, out_seq: *mut rosidl_runtime_rs::Sequence<ControllerState>) -> bool;
}

// Corresponds to controller_manager_msgs__msg__ControllerState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControllerState {
    /// controller name
    pub name: rosidl_runtime_rs::String,

    /// controller state: unconfigured, inactive, active, or finalized
    pub state: rosidl_runtime_rs::String,

    /// the controller class name, e.g. joint_trajectory_controller/JointTrajectoryController
    pub type_: rosidl_runtime_rs::String,

    /// true, if controller runs asynchronously. false, if controller runs synchronously
    pub is_async: bool,

    /// update rate of the controller in Hz
    pub update_rate: u16,

    /// command interfaces currently owned by controller
    pub claimed_interfaces: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// command interfaces required by controller
    pub required_command_interfaces: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// state interfaces required by controller
    pub required_state_interfaces: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// specifies whether or not controller can export references for a controller chain
    pub is_chainable: bool,

    /// specifies whether or not controller's exported references are claimed by another controller
    pub is_chained: bool,

    /// state interfaces to be exported (only applicable if is_chainable is true)
    pub exported_state_interfaces: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// references to be exported (only applicable if is_chainable is true)
    pub reference_interfaces: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// specifies list of controllers and their exported references that the controller is chained to
    pub chain_connections: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ChainConnection>,

}



impl Default for ControllerState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__msg__ControllerState__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__msg__ControllerState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControllerState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ControllerState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ControllerState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ControllerState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControllerState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControllerState where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/msg/ControllerState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__ControllerState() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__ChainConnection() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__msg__ChainConnection__init(msg: *mut ChainConnection) -> bool;
    fn controller_manager_msgs__msg__ChainConnection__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChainConnection>, size: usize) -> bool;
    fn controller_manager_msgs__msg__ChainConnection__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChainConnection>);
    fn controller_manager_msgs__msg__ChainConnection__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChainConnection>, out_seq: *mut rosidl_runtime_rs::Sequence<ChainConnection>) -> bool;
}

// Corresponds to controller_manager_msgs__msg__ChainConnection
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChainConnection {
    /// name of controller exporting reference_interfaces
    pub name: rosidl_runtime_rs::String,

    /// list of reference exported by controller
    pub reference_interfaces: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ChainConnection {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__msg__ChainConnection__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__msg__ChainConnection__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChainConnection {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ChainConnection__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ChainConnection__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ChainConnection__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChainConnection {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChainConnection where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/msg/ChainConnection";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__ChainConnection() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__HardwareComponentState() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__msg__HardwareComponentState__init(msg: *mut HardwareComponentState) -> bool;
    fn controller_manager_msgs__msg__HardwareComponentState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HardwareComponentState>, size: usize) -> bool;
    fn controller_manager_msgs__msg__HardwareComponentState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HardwareComponentState>);
    fn controller_manager_msgs__msg__HardwareComponentState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HardwareComponentState>, out_seq: *mut rosidl_runtime_rs::Sequence<HardwareComponentState>) -> bool;
}

// Corresponds to controller_manager_msgs__msg__HardwareComponentState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareComponentState {
    /// Name of the hardware component
    pub name: rosidl_runtime_rs::String,

    /// Type of the hardware component
    pub type_: rosidl_runtime_rs::String,

    /// If the hardware component is running asynchronously
    pub is_async: bool,

    /// read/write rate of the hardware component in Hz
    pub rw_rate: u16,

    /// DEPRECATED
    pub class_type: rosidl_runtime_rs::String,

    /// The name of the plugin that is used to load the hardware component
    pub plugin_name: rosidl_runtime_rs::String,

    /// State of the hardware component
    pub state: lifecycle_msgs::msg::rmw::State,

    /// Command interfaces of the hardware component
    pub command_interfaces: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HardwareInterface>,

    /// State interfaces of the hardware component
    pub state_interfaces: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HardwareInterface>,

}



impl Default for HardwareComponentState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__msg__HardwareComponentState__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__msg__HardwareComponentState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HardwareComponentState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__HardwareComponentState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__HardwareComponentState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__HardwareComponentState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HardwareComponentState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HardwareComponentState where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/msg/HardwareComponentState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__HardwareComponentState() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__HardwareInterface() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__msg__HardwareInterface__init(msg: *mut HardwareInterface) -> bool;
    fn controller_manager_msgs__msg__HardwareInterface__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HardwareInterface>, size: usize) -> bool;
    fn controller_manager_msgs__msg__HardwareInterface__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HardwareInterface>);
    fn controller_manager_msgs__msg__HardwareInterface__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HardwareInterface>, out_seq: *mut rosidl_runtime_rs::Sequence<HardwareInterface>) -> bool;
}

// Corresponds to controller_manager_msgs__msg__HardwareInterface
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareInterface {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_type: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_available: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_claimed: bool,

}



impl Default for HardwareInterface {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__msg__HardwareInterface__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__msg__HardwareInterface__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HardwareInterface {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__HardwareInterface__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__HardwareInterface__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__HardwareInterface__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HardwareInterface {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HardwareInterface where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/msg/HardwareInterface";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__HardwareInterface() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__NamedLifecycleState() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__msg__NamedLifecycleState__init(msg: *mut NamedLifecycleState) -> bool;
    fn controller_manager_msgs__msg__NamedLifecycleState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NamedLifecycleState>, size: usize) -> bool;
    fn controller_manager_msgs__msg__NamedLifecycleState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NamedLifecycleState>);
    fn controller_manager_msgs__msg__NamedLifecycleState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NamedLifecycleState>, out_seq: *mut rosidl_runtime_rs::Sequence<NamedLifecycleState>) -> bool;
}

// Corresponds to controller_manager_msgs__msg__NamedLifecycleState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message is used to provide information about the lifecycle state of the controller or the hardware components

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NamedLifecycleState {
    /// The name of the controller or hardware interface
    pub name: rosidl_runtime_rs::String,

    /// The current lifecycle state of the controller or hardware components
    pub state: lifecycle_msgs::msg::rmw::State,

}



impl Default for NamedLifecycleState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__msg__NamedLifecycleState__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__msg__NamedLifecycleState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NamedLifecycleState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__NamedLifecycleState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__NamedLifecycleState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__NamedLifecycleState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NamedLifecycleState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NamedLifecycleState where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/msg/NamedLifecycleState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__NamedLifecycleState() }
  }
}


#[link(name = "controller_manager_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__ControllerManagerActivity() -> *const std::ffi::c_void;
}

#[link(name = "controller_manager_msgs__rosidl_generator_c")]
extern "C" {
    fn controller_manager_msgs__msg__ControllerManagerActivity__init(msg: *mut ControllerManagerActivity) -> bool;
    fn controller_manager_msgs__msg__ControllerManagerActivity__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControllerManagerActivity>, size: usize) -> bool;
    fn controller_manager_msgs__msg__ControllerManagerActivity__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControllerManagerActivity>);
    fn controller_manager_msgs__msg__ControllerManagerActivity__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControllerManagerActivity>, out_seq: *mut rosidl_runtime_rs::Sequence<ControllerManagerActivity>) -> bool;
}

// Corresponds to controller_manager_msgs__msg__ControllerManagerActivity
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message is used to provide the activity within the controller manager regarding the change in state of controllers and hardware interfaces

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControllerManagerActivity {
    /// The header is used to provide timestamp information
    pub header: std_msgs::msg::rmw::Header,

    /// The current state of the controllers
    pub controllers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::NamedLifecycleState>,

    /// The current state of the hardware components
    pub hardware_components: rosidl_runtime_rs::Sequence<super::super::msg::rmw::NamedLifecycleState>,

}



impl Default for ControllerManagerActivity {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_manager_msgs__msg__ControllerManagerActivity__init(&mut msg as *mut _) {
        panic!("Call to controller_manager_msgs__msg__ControllerManagerActivity__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControllerManagerActivity {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ControllerManagerActivity__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ControllerManagerActivity__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_manager_msgs__msg__ControllerManagerActivity__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControllerManagerActivity {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControllerManagerActivity where Self: Sized {
  const TYPE_NAME: &'static str = "controller_manager_msgs/msg/ControllerManagerActivity";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__ControllerManagerActivity() }
  }
}


