#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__AdmittanceControllerState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__AdmittanceControllerState__init(msg: *mut AdmittanceControllerState) -> bool;
    fn control_msgs__msg__AdmittanceControllerState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AdmittanceControllerState>, size: usize) -> bool;
    fn control_msgs__msg__AdmittanceControllerState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AdmittanceControllerState>);
    fn control_msgs__msg__AdmittanceControllerState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AdmittanceControllerState>, out_seq: *mut rosidl_runtime_rs::Sequence<AdmittanceControllerState>) -> bool;
}

// Corresponds to control_msgs__msg__AdmittanceControllerState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Admittance parameters

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AdmittanceControllerState {
    /// 6-vector of mass terms used in the admittance calculation
    pub mass: std_msgs::msg::rmw::Float64MultiArray,

    /// 6-vector of damping terms used in the admittance calculation
    pub damping: std_msgs::msg::rmw::Float64MultiArray,

    /// 6-vector of stiffness terms used in the admittance calculation
    pub stiffness: std_msgs::msg::rmw::Float64MultiArray,

    /// Frame information
    /// quaternion describing the orientation of the control frame
    pub rot_base_control: geometry_msgs::msg::rmw::Quaternion,

    /// force torque sensor transform at the reference joint configuration
    pub ref_trans_base_ft: geometry_msgs::msg::rmw::TransformStamped,

    /// 6-vector of 0/1 describing if admittance is enable in the corresponding control frame axis
    pub selected_axes: std_msgs::msg::rmw::Int8MultiArray,

    /// name of the force torque frame
    pub ft_sensor_frame: std_msgs::msg::rmw::String,

    /// State information
    /// calculated admittance position in cartesian space
    pub admittance_position: geometry_msgs::msg::rmw::TransformStamped,

    /// calculated admittance acceleration in cartesian space
    pub admittance_acceleration: geometry_msgs::msg::rmw::TwistStamped,

    /// calculated admittance velocity in cartesian space
    pub admittance_velocity: geometry_msgs::msg::rmw::TwistStamped,

    /// wrench used in the admittance calculation
    pub wrench_base: geometry_msgs::msg::rmw::WrenchStamped,

    /// calculated admittance offsets in joint space
    pub joint_state: sensor_msgs::msg::rmw::JointState,

}



impl Default for AdmittanceControllerState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__AdmittanceControllerState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__AdmittanceControllerState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AdmittanceControllerState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__AdmittanceControllerState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__AdmittanceControllerState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__AdmittanceControllerState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AdmittanceControllerState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AdmittanceControllerState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/AdmittanceControllerState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__AdmittanceControllerState() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__BatteryStateArray() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__BatteryStateArray__init(msg: *mut BatteryStateArray) -> bool;
    fn control_msgs__msg__BatteryStateArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BatteryStateArray>, size: usize) -> bool;
    fn control_msgs__msg__BatteryStateArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BatteryStateArray>);
    fn control_msgs__msg__BatteryStateArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BatteryStateArray>, out_seq: *mut rosidl_runtime_rs::Sequence<BatteryStateArray>) -> bool;
}

// Corresponds to control_msgs__msg__BatteryStateArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message publishes values for multiple batteries at once.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BatteryStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_states: rosidl_runtime_rs::Sequence<sensor_msgs::msg::rmw::BatteryState>,

}



impl Default for BatteryStateArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__BatteryStateArray__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__BatteryStateArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BatteryStateArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__BatteryStateArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__BatteryStateArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__BatteryStateArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BatteryStateArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BatteryStateArray where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/BatteryStateArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__BatteryStateArray() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__DynamicInterfaceGroupValues() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__DynamicInterfaceGroupValues__init(msg: *mut DynamicInterfaceGroupValues) -> bool;
    fn control_msgs__msg__DynamicInterfaceGroupValues__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DynamicInterfaceGroupValues>, size: usize) -> bool;
    fn control_msgs__msg__DynamicInterfaceGroupValues__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DynamicInterfaceGroupValues>);
    fn control_msgs__msg__DynamicInterfaceGroupValues__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DynamicInterfaceGroupValues>, out_seq: *mut rosidl_runtime_rs::Sequence<DynamicInterfaceGroupValues>) -> bool;
}

// Corresponds to control_msgs__msg__DynamicInterfaceGroupValues
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicInterfaceGroupValues {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// List of interface group names , e.g. ["flange_analog_IOs", "flange_vacuum"]
    pub interface_groups: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// Key-value pairs representing interfaces and their corresponding values for each interface group listed in `interface_groups`
    pub interface_values: rosidl_runtime_rs::Sequence<super::super::msg::rmw::InterfaceValue>,

}



impl Default for DynamicInterfaceGroupValues {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__DynamicInterfaceGroupValues__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__DynamicInterfaceGroupValues__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DynamicInterfaceGroupValues {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicInterfaceGroupValues__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicInterfaceGroupValues__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicInterfaceGroupValues__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DynamicInterfaceGroupValues {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DynamicInterfaceGroupValues where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/DynamicInterfaceGroupValues";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__DynamicInterfaceGroupValues() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__DynamicInterfaceValues() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__DynamicInterfaceValues__init(msg: *mut DynamicInterfaceValues) -> bool;
    fn control_msgs__msg__DynamicInterfaceValues__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DynamicInterfaceValues>, size: usize) -> bool;
    fn control_msgs__msg__DynamicInterfaceValues__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DynamicInterfaceValues>);
    fn control_msgs__msg__DynamicInterfaceValues__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DynamicInterfaceValues>, out_seq: *mut rosidl_runtime_rs::Sequence<DynamicInterfaceValues>) -> bool;
}

// Corresponds to control_msgs__msg__DynamicInterfaceValues
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicInterfaceValues {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub states: super::super::msg::rmw::InterfaceValue,


    // This member is not documented.
    #[allow(missing_docs)]
    pub commands: super::super::msg::rmw::InterfaceValue,

}



impl Default for DynamicInterfaceValues {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__DynamicInterfaceValues__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__DynamicInterfaceValues__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DynamicInterfaceValues {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicInterfaceValues__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicInterfaceValues__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicInterfaceValues__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DynamicInterfaceValues {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DynamicInterfaceValues where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/DynamicInterfaceValues";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__DynamicInterfaceValues() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__DynamicJointState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__DynamicJointState__init(msg: *mut DynamicJointState) -> bool;
    fn control_msgs__msg__DynamicJointState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DynamicJointState>, size: usize) -> bool;
    fn control_msgs__msg__DynamicJointState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DynamicJointState>);
    fn control_msgs__msg__DynamicJointState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DynamicJointState>, out_seq: *mut rosidl_runtime_rs::Sequence<DynamicJointState>) -> bool;
}

// Corresponds to control_msgs__msg__DynamicJointState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicJointState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// List of resource names, e.g. ["arm_joint_1", "arm_joint_2", "gripper_joint"]
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// Key-value pairs representing interfaces and their corresponding values for each joint listed in `joint_names`
    pub interface_values: rosidl_runtime_rs::Sequence<super::super::msg::rmw::InterfaceValue>,

}



impl Default for DynamicJointState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__DynamicJointState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__DynamicJointState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DynamicJointState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicJointState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicJointState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__DynamicJointState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DynamicJointState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DynamicJointState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/DynamicJointState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__DynamicJointState() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__GripperCommand() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__GripperCommand__init(msg: *mut GripperCommand) -> bool;
    fn control_msgs__msg__GripperCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand>, size: usize) -> bool;
    fn control_msgs__msg__GripperCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand>);
    fn control_msgs__msg__GripperCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand>) -> bool;
}

// Corresponds to control_msgs__msg__GripperCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_effort: f64,

}



impl Default for GripperCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__GripperCommand__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__GripperCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__GripperCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__GripperCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__GripperCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/GripperCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__GripperCommand() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__InterfaceValue() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__InterfaceValue__init(msg: *mut InterfaceValue) -> bool;
    fn control_msgs__msg__InterfaceValue__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InterfaceValue>, size: usize) -> bool;
    fn control_msgs__msg__InterfaceValue__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InterfaceValue>);
    fn control_msgs__msg__InterfaceValue__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InterfaceValue>, out_seq: *mut rosidl_runtime_rs::Sequence<InterfaceValue>) -> bool;
}

// Corresponds to control_msgs__msg__InterfaceValue
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// List of resource interface names

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterfaceValue {

    // This member is not documented.
    #[allow(missing_docs)]
    pub interface_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// Values corresponding to the list of interfaces in `interface_names`, [1.0, 0.0] for example
    pub values: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for InterfaceValue {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__InterfaceValue__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__InterfaceValue__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InterfaceValue {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__InterfaceValue__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__InterfaceValue__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__InterfaceValue__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InterfaceValue {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InterfaceValue where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/InterfaceValue";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__InterfaceValue() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__Keys() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__Keys__init(msg: *mut Keys) -> bool;
    fn control_msgs__msg__Keys__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Keys>, size: usize) -> bool;
    fn control_msgs__msg__Keys__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Keys>);
    fn control_msgs__msg__Keys__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Keys>, out_seq: *mut rosidl_runtime_rs::Sequence<Keys>) -> bool;
}

// Corresponds to control_msgs__msg__Keys
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Keys {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// List of names defining interfaces or joints or other string-based entities
    /// being published with Keys.msg
    pub keys: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for Keys {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__Keys__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__Keys__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Keys {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__Keys__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__Keys__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__Keys__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Keys {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Keys where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/Keys";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__Keys() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__Float64Values() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__Float64Values__init(msg: *mut Float64Values) -> bool;
    fn control_msgs__msg__Float64Values__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Float64Values>, size: usize) -> bool;
    fn control_msgs__msg__Float64Values__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Float64Values>);
    fn control_msgs__msg__Float64Values__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Float64Values>, out_seq: *mut rosidl_runtime_rs::Sequence<Float64Values>) -> bool;
}

// Corresponds to control_msgs__msg__Float64Values
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Float64Values {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// List of values corresponding to the interfaces or joints defined in Keys.msg
    /// The size and order must match that of Keys.msg
    /// Each value corresponds to the key at the same index in Keys.msg
    ///
    /// For example. if Keys.msg has keys = ["joint1/position", "joint2/velocity"], then
    /// InterfacesValues.msg could have values = [1.57, 0.0]. The first value (1.57) corresponds to "joint1/position"
    /// and the second value (0.0) corresponds to "joint2/velocity"
    ///
    /// This message is used by the interfaces_state_broadcaster to publish the values of the interfaces via
    /// ROS 2 topics ~/names (Keys.msg) and ~/values (InterfacesValues.msg)
    pub values: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for Float64Values {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__Float64Values__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__Float64Values__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Float64Values {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__Float64Values__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__Float64Values__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__Float64Values__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Float64Values {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Float64Values where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/Float64Values";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__Float64Values() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointComponentTolerance() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__JointComponentTolerance__init(msg: *mut JointComponentTolerance) -> bool;
    fn control_msgs__msg__JointComponentTolerance__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointComponentTolerance>, size: usize) -> bool;
    fn control_msgs__msg__JointComponentTolerance__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointComponentTolerance>);
    fn control_msgs__msg__JointComponentTolerance__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointComponentTolerance>, out_seq: *mut rosidl_runtime_rs::Sequence<JointComponentTolerance>) -> bool;
}

// Corresponds to control_msgs__msg__JointComponentTolerance
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Version of JointTolerance.msg with added component field for joints with multiple degrees of freedom
/// The difference between two MultiDOFJointTrajectoryPoint cannot be represented as a single number,
/// hence we use the component field to represent how to calculate the difference in a way that can
/// be represented as a single number.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointComponentTolerance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub component: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acceleration: f64,

}

impl JointComponentTolerance {
    /// Since each joint has multiple degrees of freedom,
    /// there can be multiple tolerances for each joint, each looking
    /// at different components.
    /// If the component is X_AXIS, Y_AXIS, or Z_AXIS, then the tolerance
    /// is only applied for the specific axis.
    /// However, if the component is TRANSLATION, then the tolerance is for
    /// the overall Euclidean distance.
    /// For these components, the units are meters, meters/sec and meters/sec^2.
    /// Z_AXIS is only valid with a floating joint, not planar.
    /// If the component is ROTATION the tolerance is measured in
    /// radians, radians/sec and radians/sec^2, computed
    /// between the difference in quaternions.
    pub const X_AXIS: u16 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const Y_AXIS: u16 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const Z_AXIS: u16 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TRANSLATION: u16 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROTATION: u16 = 5;

}


impl Default for JointComponentTolerance {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__JointComponentTolerance__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__JointComponentTolerance__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointComponentTolerance {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointComponentTolerance__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointComponentTolerance__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointComponentTolerance__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointComponentTolerance {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointComponentTolerance where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/JointComponentTolerance";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointComponentTolerance() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointCommand() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__JointCommand__init(msg: *mut JointCommand) -> bool;
    fn control_msgs__msg__JointCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointCommand>, size: usize) -> bool;
    fn control_msgs__msg__JointCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointCommand>);
    fn control_msgs__msg__JointCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<JointCommand>) -> bool;
}

// Corresponds to control_msgs__msg__JointCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains the command for a specific interface across multiple joints.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// List of joint names to command.
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The specific interface type to command (e.g., "position", "velocity", "effort").
    pub interface_name: rosidl_runtime_rs::String,

    /// The commanded values. The length of this array must exactly match the length of joint_names.
    pub values: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for JointCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__JointCommand__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__JointCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointCommand where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/JointCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointCommand() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointControllerState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__JointControllerState__init(msg: *mut JointControllerState) -> bool;
    fn control_msgs__msg__JointControllerState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointControllerState>, size: usize) -> bool;
    fn control_msgs__msg__JointControllerState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointControllerState>);
    fn control_msgs__msg__JointControllerState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointControllerState>, out_seq: *mut rosidl_runtime_rs::Sequence<JointControllerState>) -> bool;
}

// Corresponds to control_msgs__msg__JointControllerState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message presents current controller state of one joint.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointControllerState {
    /// It is deprecated as of Humble in favor of SingleDOFStateStamped.msg
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::rmw::Header,

    /// The set point, that is, desired state.
    pub set_point: f64,

    /// Current value of the process (ie: latest sensor measurement on the controlled value).
    pub process_value: f64,

    /// First time-derivative of the process value.
    pub process_value_dot: f64,

    /// The error of the controlled value, essentially process_value - set_point (for a regular PID implementation).
    pub error: f64,

    /// Time between two consecutive updates/execution of the control law.
    pub time_step: f64,

    /// Current output of the controller.
    pub command: f64,

    /// Current PID parameters of the controller.
    pub p: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub i_clamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub antiwindup: bool,

}



impl Default for JointControllerState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__JointControllerState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__JointControllerState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointControllerState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointControllerState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointControllerState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointControllerState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointControllerState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointControllerState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/JointControllerState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointControllerState() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointJog() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__JointJog__init(msg: *mut JointJog) -> bool;
    fn control_msgs__msg__JointJog__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointJog>, size: usize) -> bool;
    fn control_msgs__msg__JointJog__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointJog>);
    fn control_msgs__msg__JointJog__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointJog>, out_seq: *mut rosidl_runtime_rs::Sequence<JointJog>) -> bool;
}

// Corresponds to control_msgs__msg__JointJog
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Used in time-stamping the message.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointJog {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Name list of the joints. You don't need to specify all joints of the
    /// robot. Joint names are case-sensitive.
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// A position command to the joints listed in joint_names.
    /// The order must be identical.
    /// Units are meters or radians.
    /// If displacements and velocities are filled, a profiled motion is requested.
    /// or position_deltas
    pub displacements: rosidl_runtime_rs::Sequence<f64>,

    /// A velocity command to the joints listed in joint_names.
    /// The order must be identical.
    /// Units are m/s or rad/s.
    /// If displacements and velocities are filled, a profiled motion is requested.
    pub velocities: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: f64,

}



impl Default for JointJog {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__JointJog__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__JointJog__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointJog {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointJog__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointJog__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointJog__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointJog {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointJog where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/JointJog";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointJog() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointTolerance() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__JointTolerance__init(msg: *mut JointTolerance) -> bool;
    fn control_msgs__msg__JointTolerance__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTolerance>, size: usize) -> bool;
    fn control_msgs__msg__JointTolerance__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTolerance>);
    fn control_msgs__msg__JointTolerance__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTolerance>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTolerance>) -> bool;
}

// Corresponds to control_msgs__msg__JointTolerance
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The tolerances specify the amount the position, velocity, and
/// accelerations can vary from the setpoints.  For example, in the case
/// of trajectory control, when the actual position varies beyond
/// (desired position + position tolerance), the trajectory goal may
/// abort.
///
/// There are two special values for tolerances:
///  * 0 - The tolerance is unspecified and will remain at whatever the default is
///  * -1 - The tolerance is "erased".  If there was a default, the joint will be
///         allowed to move without restriction.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTolerance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

    /// in radians or meters (for a revolute or prismatic joint, respectively)
    pub position: f64,

    /// in rad/sec or m/sec
    pub velocity: f64,

    /// in rad/sec^2 or m/sec^2
    pub acceleration: f64,

}



impl Default for JointTolerance {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__JointTolerance__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__JointTolerance__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTolerance {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointTolerance__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointTolerance__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointTolerance__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTolerance {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTolerance where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/JointTolerance";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointTolerance() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointTrajectoryControllerState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__JointTrajectoryControllerState__init(msg: *mut JointTrajectoryControllerState) -> bool;
    fn control_msgs__msg__JointTrajectoryControllerState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectoryControllerState>, size: usize) -> bool;
    fn control_msgs__msg__JointTrajectoryControllerState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointTrajectoryControllerState>);
    fn control_msgs__msg__JointTrajectoryControllerState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointTrajectoryControllerState>, out_seq: *mut rosidl_runtime_rs::Sequence<JointTrajectoryControllerState>) -> bool;
}

// Corresponds to control_msgs__msg__JointTrajectoryControllerState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message presents current controller state of JTC

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectoryControllerState {
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The set point, that is, desired state.
    pub reference: trajectory_msgs::msg::rmw::JointTrajectoryPoint,

    /// Current value of the process (ie: latest sensor measurement on the controlled value).
    pub feedback: trajectory_msgs::msg::rmw::JointTrajectoryPoint,

    /// The error of the controlled value, essentially reference - feedback (for a regular PID implementation).
    pub error: trajectory_msgs::msg::rmw::JointTrajectoryPoint,

    /// Current output of the controller.
    pub output: trajectory_msgs::msg::rmw::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_joint_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The set point, that is, desired state.
    pub multi_dof_reference: trajectory_msgs::msg::rmw::MultiDOFJointTrajectoryPoint,

    /// Current value of the process (ie: latest sensor measurement on the controlled value).
    pub multi_dof_feedback: trajectory_msgs::msg::rmw::MultiDOFJointTrajectoryPoint,

    /// The error of the controlled value, essentially reference - feedback (for a regular PID implementation).
    pub multi_dof_error: trajectory_msgs::msg::rmw::MultiDOFJointTrajectoryPoint,

    /// Current output of the controller.
    pub multi_dof_output: trajectory_msgs::msg::rmw::MultiDOFJointTrajectoryPoint,

    /// The speed scaling factor the trajectory is currently being executed with
    pub speed_scaling_factor: f64,

}



impl Default for JointTrajectoryControllerState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__JointTrajectoryControllerState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__JointTrajectoryControllerState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointTrajectoryControllerState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointTrajectoryControllerState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointTrajectoryControllerState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__JointTrajectoryControllerState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointTrajectoryControllerState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointTrajectoryControllerState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/JointTrajectoryControllerState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__JointTrajectoryControllerState() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MecanumDriveControllerState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__MecanumDriveControllerState__init(msg: *mut MecanumDriveControllerState) -> bool;
    fn control_msgs__msg__MecanumDriveControllerState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MecanumDriveControllerState>, size: usize) -> bool;
    fn control_msgs__msg__MecanumDriveControllerState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MecanumDriveControllerState>);
    fn control_msgs__msg__MecanumDriveControllerState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MecanumDriveControllerState>, out_seq: *mut rosidl_runtime_rs::Sequence<MecanumDriveControllerState>) -> bool;
}

// Corresponds to control_msgs__msg__MecanumDriveControllerState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecanumDriveControllerState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_left_wheel_velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub back_left_wheel_velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub back_right_wheel_velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_right_wheel_velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reference_velocity: geometry_msgs::msg::rmw::Twist,

}



impl Default for MecanumDriveControllerState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__MecanumDriveControllerState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__MecanumDriveControllerState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MecanumDriveControllerState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MecanumDriveControllerState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MecanumDriveControllerState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MecanumDriveControllerState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MecanumDriveControllerState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MecanumDriveControllerState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/MecanumDriveControllerState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MecanumDriveControllerState() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MotionArgument() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__MotionArgument__init(msg: *mut MotionArgument) -> bool;
    fn control_msgs__msg__MotionArgument__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionArgument>, size: usize) -> bool;
    fn control_msgs__msg__MotionArgument__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionArgument>);
    fn control_msgs__msg__MotionArgument__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionArgument>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionArgument>) -> bool;
}

// Corresponds to control_msgs__msg__MotionArgument
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Key Value pair to be used in MotionPrimitive messages

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionArgument {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: f64,

}



impl Default for MotionArgument {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__MotionArgument__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__MotionArgument__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionArgument {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionArgument__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionArgument__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionArgument__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionArgument {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionArgument where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/MotionArgument";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MotionArgument() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MotionPrimitive() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__MotionPrimitive__init(msg: *mut MotionPrimitive) -> bool;
    fn control_msgs__msg__MotionPrimitive__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionPrimitive>, size: usize) -> bool;
    fn control_msgs__msg__MotionPrimitive__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionPrimitive>);
    fn control_msgs__msg__MotionPrimitive__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionPrimitive>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionPrimitive>) -> bool;
}

// Corresponds to control_msgs__msg__MotionPrimitive
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPrimitive {
    /// potentially more, such as spline motion
    /// one of the above
    pub type_: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blend_radius: f64,

    /// MotionArguments are meant to be used as vendor-specific or use-case-specific arguments to
    /// MotionPrimitive messages. This is to avoid bloating the MotionPrimitive message with all possible
    /// uses cases but rather add specific data using as a list of key-value pairs, which is what this
    /// message represents.
    /// (max) velocity, (max) acceleration, efficiency
    pub additional_arguments: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotionArgument>,

    /// Targets should be either specified through joint configurations or Cartesian poses.
    /// Depending on the motion type and implementation, there might be multiple poses allowed.
    /// For example, circular motions are often specified as via and target
    /// Usually one of these is empty, in some edge cases even both can be empty, when all input is given
    /// in the more flexible MotionArgument[] field.
    pub poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::PoseStamped>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_positions: rosidl_runtime_rs::Sequence<f64>,

}

impl MotionPrimitive {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNKNOWN: i8 = -1;

    /// Often referred as PTP
    pub const LINEAR_JOINT: i8 = 0;

    /// Often referred as LIN
    pub const LINEAR_CARTESIAN: i8 = 50;

    /// Often referred as CIRC
    pub const CIRCULAR_CARTESIAN: i8 = 51;

}


impl Default for MotionPrimitive {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__MotionPrimitive__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__MotionPrimitive__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionPrimitive {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionPrimitive__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionPrimitive__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionPrimitive__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionPrimitive {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionPrimitive where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/MotionPrimitive";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MotionPrimitive() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MotionPrimitiveSequence() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__MotionPrimitiveSequence__init(msg: *mut MotionPrimitiveSequence) -> bool;
    fn control_msgs__msg__MotionPrimitiveSequence__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotionPrimitiveSequence>, size: usize) -> bool;
    fn control_msgs__msg__MotionPrimitiveSequence__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotionPrimitiveSequence>);
    fn control_msgs__msg__MotionPrimitiveSequence__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotionPrimitiveSequence>, out_seq: *mut rosidl_runtime_rs::Sequence<MotionPrimitiveSequence>) -> bool;
}

// Corresponds to control_msgs__msg__MotionPrimitiveSequence
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPrimitiveSequence {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motions: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotionPrimitive>,

}



impl Default for MotionPrimitiveSequence {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__MotionPrimitiveSequence__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__MotionPrimitiveSequence__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotionPrimitiveSequence {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionPrimitiveSequence__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionPrimitiveSequence__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MotionPrimitiveSequence__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotionPrimitiveSequence {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotionPrimitiveSequence where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/MotionPrimitiveSequence";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MotionPrimitiveSequence() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MultiDOFCommand() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__MultiDOFCommand__init(msg: *mut MultiDOFCommand) -> bool;
    fn control_msgs__msg__MultiDOFCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MultiDOFCommand>, size: usize) -> bool;
    fn control_msgs__msg__MultiDOFCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MultiDOFCommand>);
    fn control_msgs__msg__MultiDOFCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MultiDOFCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<MultiDOFCommand>) -> bool;
}

// Corresponds to control_msgs__msg__MultiDOFCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The message defines command for multiple degrees of freedom (DoF) typically used by many controllers.
/// The message intentionally avoids 'joint' nomenclature because it can be generally use for command with
/// different semantic meanings, e.g., joints, Cartesian axes, or have abstract meaning like GPIO interface.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MultiDOFCommand {
    /// names of degrees of freedom
    pub dof_names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// values used by most of the controller
    pub values: rosidl_runtime_rs::Sequence<f64>,

    /// First derivation of the values, e.g., velocity if values are positions.
    /// This is useful for PID and similar controllers.
    pub values_dot: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for MultiDOFCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__MultiDOFCommand__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__MultiDOFCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MultiDOFCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MultiDOFCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MultiDOFCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MultiDOFCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MultiDOFCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MultiDOFCommand where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/MultiDOFCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MultiDOFCommand() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MultiDOFStateStamped() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__MultiDOFStateStamped__init(msg: *mut MultiDOFStateStamped) -> bool;
    fn control_msgs__msg__MultiDOFStateStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MultiDOFStateStamped>, size: usize) -> bool;
    fn control_msgs__msg__MultiDOFStateStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MultiDOFStateStamped>);
    fn control_msgs__msg__MultiDOFStateStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MultiDOFStateStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<MultiDOFStateStamped>) -> bool;
}

// Corresponds to control_msgs__msg__MultiDOFStateStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message presents current controller state of multiple degrees of freedom.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MultiDOFStateStamped {
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dof_states: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SingleDOFState>,

}



impl Default for MultiDOFStateStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__MultiDOFStateStamped__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__MultiDOFStateStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MultiDOFStateStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MultiDOFStateStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MultiDOFStateStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__MultiDOFStateStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MultiDOFStateStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MultiDOFStateStamped where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/MultiDOFStateStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__MultiDOFStateStamped() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__PidState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__PidState__init(msg: *mut PidState) -> bool;
    fn control_msgs__msg__PidState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PidState>, size: usize) -> bool;
    fn control_msgs__msg__PidState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PidState>);
    fn control_msgs__msg__PidState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PidState>, out_seq: *mut rosidl_runtime_rs::Sequence<PidState>) -> bool;
}

// Corresponds to control_msgs__msg__PidState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PidState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestep: builtin_interfaces::msg::rmw::Duration,

    /// error = target - state
    pub error: f64,

    /// derivative of error
    pub error_dot: f64,

    /// equals error
    pub p_error: f64,

    /// weighted integral of error
    pub i_error: f64,

    /// equals derivative of error
    pub d_error: f64,

    /// proportional gain
    pub p_term: f64,

    /// integral gain
    pub i_term: f64,

    /// derivative gain
    pub d_term: f64,

    /// upper integral clamp.
    pub i_max: f64,

    /// lower integral clamp.
    pub i_min: f64,

    /// output of the PID controller
    pub output: f64,

}



impl Default for PidState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__PidState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__PidState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PidState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__PidState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__PidState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__PidState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PidState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PidState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/PidState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__PidState() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SingleDOFState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__SingleDOFState__init(msg: *mut SingleDOFState) -> bool;
    fn control_msgs__msg__SingleDOFState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleDOFState>, size: usize) -> bool;
    fn control_msgs__msg__SingleDOFState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleDOFState>);
    fn control_msgs__msg__SingleDOFState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleDOFState>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleDOFState>) -> bool;
}

// Corresponds to control_msgs__msg__SingleDOFState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message presents current controller state of one degree of freedom.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleDOFState {
    /// DoF name, e.g., joint or Cartesian axis name
    pub name: rosidl_runtime_rs::String,

    /// The set point, that is, desired state.
    pub reference: f64,

    /// Current value of the process (ie: latest sensor measurement on the controlled value).
    pub feedback: f64,

    /// First time-derivative of the process value. E.g., velocity.
    pub feedback_dot: f64,

    /// The error of the controlled value, essentially reference - feedback (for a regular PID implementation).
    pub error: f64,

    /// First time-derivative of the error of the controlled value.
    pub error_dot: f64,

    /// Time between two consecutive updates/execution of the control law.
    pub time_step: f64,

    /// Current output of the controller.
    pub output: f64,

}



impl Default for SingleDOFState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__SingleDOFState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__SingleDOFState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleDOFState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SingleDOFState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SingleDOFState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SingleDOFState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleDOFState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleDOFState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/SingleDOFState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SingleDOFState() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SingleDOFStateStamped() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__SingleDOFStateStamped__init(msg: *mut SingleDOFStateStamped) -> bool;
    fn control_msgs__msg__SingleDOFStateStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SingleDOFStateStamped>, size: usize) -> bool;
    fn control_msgs__msg__SingleDOFStateStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SingleDOFStateStamped>);
    fn control_msgs__msg__SingleDOFStateStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SingleDOFStateStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<SingleDOFStateStamped>) -> bool;
}

// Corresponds to control_msgs__msg__SingleDOFStateStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message presents current controller state of one degree of freedom.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleDOFStateStamped {
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dof_state: super::super::msg::rmw::SingleDOFState,

}



impl Default for SingleDOFStateStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__SingleDOFStateStamped__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__SingleDOFStateStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SingleDOFStateStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SingleDOFStateStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SingleDOFStateStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SingleDOFStateStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SingleDOFStateStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SingleDOFStateStamped where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/SingleDOFStateStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SingleDOFStateStamped() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SteeringControllerStatus() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__SteeringControllerStatus__init(msg: *mut SteeringControllerStatus) -> bool;
    fn control_msgs__msg__SteeringControllerStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SteeringControllerStatus>, size: usize) -> bool;
    fn control_msgs__msg__SteeringControllerStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SteeringControllerStatus>);
    fn control_msgs__msg__SteeringControllerStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SteeringControllerStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<SteeringControllerStatus>) -> bool;
}

// Corresponds to control_msgs__msg__SteeringControllerStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SteeringControllerStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// positions of traction wheels if the robot is controlled by position
    pub traction_wheels_position: rosidl_runtime_rs::Sequence<f64>,

    /// velocities of traction wheels if the robot is controlled by velocity
    pub traction_wheels_velocity: rosidl_runtime_rs::Sequence<f64>,

    /// positions of steering joints
    pub steer_positions: rosidl_runtime_rs::Sequence<f64>,

    /// value commanded to tractions joint
    pub linear_velocity_command: rosidl_runtime_rs::Sequence<f64>,

    /// values commanded to steering joints
    pub steering_angle_command: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for SteeringControllerStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__SteeringControllerStatus__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__SteeringControllerStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SteeringControllerStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SteeringControllerStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SteeringControllerStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SteeringControllerStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SteeringControllerStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SteeringControllerStatus where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/SteeringControllerStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SteeringControllerStatus() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SpeedScalingFactor() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__SpeedScalingFactor__init(msg: *mut SpeedScalingFactor) -> bool;
    fn control_msgs__msg__SpeedScalingFactor__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedScalingFactor>, size: usize) -> bool;
    fn control_msgs__msg__SpeedScalingFactor__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedScalingFactor>);
    fn control_msgs__msg__SpeedScalingFactor__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedScalingFactor>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedScalingFactor>) -> bool;
}

// Corresponds to control_msgs__msg__SpeedScalingFactor
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains a scaling factor to scale trajectory execution. A factor of 1.0 means
/// execution at normal speed, a factor of 0.0 means a full pause.
/// Negative values are not allowed (Which should be checked by any instance consuming this message).

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedScalingFactor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub factor: f64,

}



impl Default for SpeedScalingFactor {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__SpeedScalingFactor__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__SpeedScalingFactor__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedScalingFactor {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SpeedScalingFactor__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SpeedScalingFactor__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__SpeedScalingFactor__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedScalingFactor {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedScalingFactor where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/SpeedScalingFactor";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__SpeedScalingFactor() }
  }
}


#[link(name = "control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__VDA5050SafetyState() -> *const std::ffi::c_void;
}

#[link(name = "control_msgs__rosidl_generator_c")]
extern "C" {
    fn control_msgs__msg__VDA5050SafetyState__init(msg: *mut VDA5050SafetyState) -> bool;
    fn control_msgs__msg__VDA5050SafetyState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VDA5050SafetyState>, size: usize) -> bool;
    fn control_msgs__msg__VDA5050SafetyState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VDA5050SafetyState>);
    fn control_msgs__msg__VDA5050SafetyState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VDA5050SafetyState>, out_seq: *mut rosidl_runtime_rs::Sequence<VDA5050SafetyState>) -> bool;
}

// Corresponds to control_msgs__msg__VDA5050SafetyState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VDA5050SafetyState {
    /// Enum {autoAck, manual, remote, none} Acknowledge-Type of eStop:
    /// autoAck: autoacknowledgeable e-stop is activated e.g. by bumper or protective field
    /// manual: e-stop has to be acknowledged manually at the vehicle
    /// remote: facility estop has to be acknowledged remotely
    /// none: no e-stop activated
    pub e_stop: rosidl_runtime_rs::String,

    /// Protective field violation. True: field is violated False: field is not violated
    pub field_violation: bool,

}

impl VDA5050SafetyState {
    /// Enums for eStop
    pub const AUTO_ACK: &'static str = "autoAck";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MANUAL: &'static str = "manual";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const REMOTE: &'static str = "remote";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NONE: &'static str = "none";

}


impl Default for VDA5050SafetyState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !control_msgs__msg__VDA5050SafetyState__init(&mut msg as *mut _) {
        panic!("Call to control_msgs__msg__VDA5050SafetyState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VDA5050SafetyState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__VDA5050SafetyState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__VDA5050SafetyState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { control_msgs__msg__VDA5050SafetyState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VDA5050SafetyState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VDA5050SafetyState where Self: Sized {
  const TYPE_NAME: &'static str = "control_msgs/msg/VDA5050SafetyState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__control_msgs__msg__VDA5050SafetyState() }
  }
}


