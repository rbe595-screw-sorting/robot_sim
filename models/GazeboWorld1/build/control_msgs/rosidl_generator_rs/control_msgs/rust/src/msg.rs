#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to control_msgs__msg__AdmittanceControllerState
/// Admittance parameters

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AdmittanceControllerState {
    /// 6-vector of mass terms used in the admittance calculation
    pub mass: std_msgs::msg::Float64MultiArray,

    /// 6-vector of damping terms used in the admittance calculation
    pub damping: std_msgs::msg::Float64MultiArray,

    /// 6-vector of stiffness terms used in the admittance calculation
    pub stiffness: std_msgs::msg::Float64MultiArray,

    /// Frame information
    /// quaternion describing the orientation of the control frame
    pub rot_base_control: geometry_msgs::msg::Quaternion,

    /// force torque sensor transform at the reference joint configuration
    pub ref_trans_base_ft: geometry_msgs::msg::TransformStamped,

    /// 6-vector of 0/1 describing if admittance is enable in the corresponding control frame axis
    pub selected_axes: std_msgs::msg::Int8MultiArray,

    /// name of the force torque frame
    pub ft_sensor_frame: std_msgs::msg::String,

    /// State information
    /// calculated admittance position in cartesian space
    pub admittance_position: geometry_msgs::msg::TransformStamped,

    /// calculated admittance acceleration in cartesian space
    pub admittance_acceleration: geometry_msgs::msg::TwistStamped,

    /// calculated admittance velocity in cartesian space
    pub admittance_velocity: geometry_msgs::msg::TwistStamped,

    /// wrench used in the admittance calculation
    pub wrench_base: geometry_msgs::msg::WrenchStamped,

    /// calculated admittance offsets in joint space
    pub joint_state: sensor_msgs::msg::JointState,

}



impl Default for AdmittanceControllerState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AdmittanceControllerState::default())
  }
}

impl rosidl_runtime_rs::Message for AdmittanceControllerState {
  type RmwMsg = super::msg::rmw::AdmittanceControllerState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mass: std_msgs::msg::Float64MultiArray::into_rmw_message(std::borrow::Cow::Owned(msg.mass)).into_owned(),
        damping: std_msgs::msg::Float64MultiArray::into_rmw_message(std::borrow::Cow::Owned(msg.damping)).into_owned(),
        stiffness: std_msgs::msg::Float64MultiArray::into_rmw_message(std::borrow::Cow::Owned(msg.stiffness)).into_owned(),
        rot_base_control: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Owned(msg.rot_base_control)).into_owned(),
        ref_trans_base_ft: geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Owned(msg.ref_trans_base_ft)).into_owned(),
        selected_axes: std_msgs::msg::Int8MultiArray::into_rmw_message(std::borrow::Cow::Owned(msg.selected_axes)).into_owned(),
        ft_sensor_frame: std_msgs::msg::String::into_rmw_message(std::borrow::Cow::Owned(msg.ft_sensor_frame)).into_owned(),
        admittance_position: geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Owned(msg.admittance_position)).into_owned(),
        admittance_acceleration: geometry_msgs::msg::TwistStamped::into_rmw_message(std::borrow::Cow::Owned(msg.admittance_acceleration)).into_owned(),
        admittance_velocity: geometry_msgs::msg::TwistStamped::into_rmw_message(std::borrow::Cow::Owned(msg.admittance_velocity)).into_owned(),
        wrench_base: geometry_msgs::msg::WrenchStamped::into_rmw_message(std::borrow::Cow::Owned(msg.wrench_base)).into_owned(),
        joint_state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(msg.joint_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mass: std_msgs::msg::Float64MultiArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.mass)).into_owned(),
        damping: std_msgs::msg::Float64MultiArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.damping)).into_owned(),
        stiffness: std_msgs::msg::Float64MultiArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stiffness)).into_owned(),
        rot_base_control: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Borrowed(&msg.rot_base_control)).into_owned(),
        ref_trans_base_ft: geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.ref_trans_base_ft)).into_owned(),
        selected_axes: std_msgs::msg::Int8MultiArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.selected_axes)).into_owned(),
        ft_sensor_frame: std_msgs::msg::String::into_rmw_message(std::borrow::Cow::Borrowed(&msg.ft_sensor_frame)).into_owned(),
        admittance_position: geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.admittance_position)).into_owned(),
        admittance_acceleration: geometry_msgs::msg::TwistStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.admittance_acceleration)).into_owned(),
        admittance_velocity: geometry_msgs::msg::TwistStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.admittance_velocity)).into_owned(),
        wrench_base: geometry_msgs::msg::WrenchStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.wrench_base)).into_owned(),
        joint_state: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.joint_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mass: std_msgs::msg::Float64MultiArray::from_rmw_message(msg.mass),
      damping: std_msgs::msg::Float64MultiArray::from_rmw_message(msg.damping),
      stiffness: std_msgs::msg::Float64MultiArray::from_rmw_message(msg.stiffness),
      rot_base_control: geometry_msgs::msg::Quaternion::from_rmw_message(msg.rot_base_control),
      ref_trans_base_ft: geometry_msgs::msg::TransformStamped::from_rmw_message(msg.ref_trans_base_ft),
      selected_axes: std_msgs::msg::Int8MultiArray::from_rmw_message(msg.selected_axes),
      ft_sensor_frame: std_msgs::msg::String::from_rmw_message(msg.ft_sensor_frame),
      admittance_position: geometry_msgs::msg::TransformStamped::from_rmw_message(msg.admittance_position),
      admittance_acceleration: geometry_msgs::msg::TwistStamped::from_rmw_message(msg.admittance_acceleration),
      admittance_velocity: geometry_msgs::msg::TwistStamped::from_rmw_message(msg.admittance_velocity),
      wrench_base: geometry_msgs::msg::WrenchStamped::from_rmw_message(msg.wrench_base),
      joint_state: sensor_msgs::msg::JointState::from_rmw_message(msg.joint_state),
    }
  }
}


// Corresponds to control_msgs__msg__BatteryStateArray
/// This message publishes values for multiple batteries at once.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BatteryStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_states: Vec<sensor_msgs::msg::BatteryState>,

}



impl Default for BatteryStateArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BatteryStateArray::default())
  }
}

impl rosidl_runtime_rs::Message for BatteryStateArray {
  type RmwMsg = super::msg::rmw::BatteryStateArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        battery_states: msg.battery_states
          .into_iter()
          .map(|elem| sensor_msgs::msg::BatteryState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        battery_states: msg.battery_states
          .iter()
          .map(|elem| sensor_msgs::msg::BatteryState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      battery_states: msg.battery_states
          .into_iter()
          .map(sensor_msgs::msg::BatteryState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to control_msgs__msg__DynamicInterfaceGroupValues

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicInterfaceGroupValues {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// List of interface group names , e.g. ["flange_analog_IOs", "flange_vacuum"]
    pub interface_groups: Vec<std::string::String>,

    /// Key-value pairs representing interfaces and their corresponding values for each interface group listed in `interface_groups`
    pub interface_values: Vec<super::msg::InterfaceValue>,

}



impl Default for DynamicInterfaceGroupValues {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DynamicInterfaceGroupValues::default())
  }
}

impl rosidl_runtime_rs::Message for DynamicInterfaceGroupValues {
  type RmwMsg = super::msg::rmw::DynamicInterfaceGroupValues;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        interface_groups: msg.interface_groups
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        interface_values: msg.interface_values
          .into_iter()
          .map(|elem| super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        interface_groups: msg.interface_groups
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        interface_values: msg.interface_values
          .iter()
          .map(|elem| super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      interface_groups: msg.interface_groups
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      interface_values: msg.interface_values
          .into_iter()
          .map(super::msg::InterfaceValue::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to control_msgs__msg__DynamicInterfaceValues

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicInterfaceValues {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub states: super::msg::InterfaceValue,


    // This member is not documented.
    #[allow(missing_docs)]
    pub commands: super::msg::InterfaceValue,

}



impl Default for DynamicInterfaceValues {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DynamicInterfaceValues::default())
  }
}

impl rosidl_runtime_rs::Message for DynamicInterfaceValues {
  type RmwMsg = super::msg::rmw::DynamicInterfaceValues;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        states: super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Owned(msg.states)).into_owned(),
        commands: super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Owned(msg.commands)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        states: super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Borrowed(&msg.states)).into_owned(),
        commands: super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Borrowed(&msg.commands)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      states: super::msg::InterfaceValue::from_rmw_message(msg.states),
      commands: super::msg::InterfaceValue::from_rmw_message(msg.commands),
    }
  }
}


// Corresponds to control_msgs__msg__DynamicJointState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DynamicJointState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// List of resource names, e.g. ["arm_joint_1", "arm_joint_2", "gripper_joint"]
    pub joint_names: Vec<std::string::String>,

    /// Key-value pairs representing interfaces and their corresponding values for each joint listed in `joint_names`
    pub interface_values: Vec<super::msg::InterfaceValue>,

}



impl Default for DynamicJointState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DynamicJointState::default())
  }
}

impl rosidl_runtime_rs::Message for DynamicJointState {
  type RmwMsg = super::msg::rmw::DynamicJointState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        interface_values: msg.interface_values
          .into_iter()
          .map(|elem| super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        interface_values: msg.interface_values
          .iter()
          .map(|elem| super::msg::InterfaceValue::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
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
      interface_values: msg.interface_values
          .into_iter()
          .map(super::msg::InterfaceValue::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to control_msgs__msg__GripperCommand

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GripperCommand::default())
  }
}

impl rosidl_runtime_rs::Message for GripperCommand {
  type RmwMsg = super::msg::rmw::GripperCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: msg.position,
        max_effort: msg.max_effort,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      position: msg.position,
      max_effort: msg.max_effort,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: msg.position,
      max_effort: msg.max_effort,
    }
  }
}


// Corresponds to control_msgs__msg__InterfaceValue
/// List of resource interface names

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterfaceValue {

    // This member is not documented.
    #[allow(missing_docs)]
    pub interface_names: Vec<std::string::String>,

    /// Values corresponding to the list of interfaces in `interface_names`, [1.0, 0.0] for example
    pub values: Vec<f64>,

}



impl Default for InterfaceValue {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::InterfaceValue::default())
  }
}

impl rosidl_runtime_rs::Message for InterfaceValue {
  type RmwMsg = super::msg::rmw::InterfaceValue;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        interface_names: msg.interface_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        values: msg.values.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        interface_names: msg.interface_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        values: msg.values.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      interface_names: msg.interface_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      values: msg.values.into(),
    }
  }
}


// Corresponds to control_msgs__msg__Keys

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Keys {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// List of names defining interfaces or joints or other string-based entities
    /// being published with Keys.msg
    pub keys: Vec<std::string::String>,

}



impl Default for Keys {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Keys::default())
  }
}

impl rosidl_runtime_rs::Message for Keys {
  type RmwMsg = super::msg::rmw::Keys;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        keys: msg.keys
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        keys: msg.keys
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      keys: msg.keys
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to control_msgs__msg__Float64Values

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Float64Values {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

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
    pub values: Vec<f64>,

}



impl Default for Float64Values {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Float64Values::default())
  }
}

impl rosidl_runtime_rs::Message for Float64Values {
  type RmwMsg = super::msg::rmw::Float64Values;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        values: msg.values.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        values: msg.values.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      values: msg.values.into(),
    }
  }
}


// Corresponds to control_msgs__msg__JointComponentTolerance
/// Version of JointTolerance.msg with added component field for joints with multiple degrees of freedom
/// The difference between two MultiDOFJointTrajectoryPoint cannot be represented as a single number,
/// hence we use the component field to represent how to calculate the difference in a way that can
/// be represented as a single number.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointComponentTolerance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_name: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointComponentTolerance::default())
  }
}

impl rosidl_runtime_rs::Message for JointComponentTolerance {
  type RmwMsg = super::msg::rmw::JointComponentTolerance;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
        component: msg.component,
        position: msg.position,
        velocity: msg.velocity,
        acceleration: msg.acceleration,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_name: msg.joint_name.as_str().into(),
      component: msg.component,
      position: msg.position,
      velocity: msg.velocity,
      acceleration: msg.acceleration,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_name: msg.joint_name.to_string(),
      component: msg.component,
      position: msg.position,
      velocity: msg.velocity,
      acceleration: msg.acceleration,
    }
  }
}


// Corresponds to control_msgs__msg__JointCommand
/// This message contains the command for a specific interface across multiple joints.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// List of joint names to command.
    pub joint_names: Vec<std::string::String>,

    /// The specific interface type to command (e.g., "position", "velocity", "effort").
    pub interface_name: std::string::String,

    /// The commanded values. The length of this array must exactly match the length of joint_names.
    pub values: Vec<f64>,

}



impl Default for JointCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointCommand::default())
  }
}

impl rosidl_runtime_rs::Message for JointCommand {
  type RmwMsg = super::msg::rmw::JointCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        interface_name: msg.interface_name.as_str().into(),
        values: msg.values.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        interface_name: msg.interface_name.as_str().into(),
        values: msg.values.as_slice().into(),
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
      interface_name: msg.interface_name.to_string(),
      values: msg.values.into(),
    }
  }
}


// Corresponds to control_msgs__msg__JointControllerState
/// This message presents current controller state of one joint.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointControllerState {
    /// It is deprecated as of Humble in favor of SingleDOFStateStamped.msg
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::Header,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointControllerState::default())
  }
}

impl rosidl_runtime_rs::Message for JointControllerState {
  type RmwMsg = super::msg::rmw::JointControllerState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        set_point: msg.set_point,
        process_value: msg.process_value,
        process_value_dot: msg.process_value_dot,
        error: msg.error,
        time_step: msg.time_step,
        command: msg.command,
        p: msg.p,
        i: msg.i,
        d: msg.d,
        i_clamp: msg.i_clamp,
        antiwindup: msg.antiwindup,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      set_point: msg.set_point,
      process_value: msg.process_value,
      process_value_dot: msg.process_value_dot,
      error: msg.error,
      time_step: msg.time_step,
      command: msg.command,
      p: msg.p,
      i: msg.i,
      d: msg.d,
      i_clamp: msg.i_clamp,
      antiwindup: msg.antiwindup,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      set_point: msg.set_point,
      process_value: msg.process_value,
      process_value_dot: msg.process_value_dot,
      error: msg.error,
      time_step: msg.time_step,
      command: msg.command,
      p: msg.p,
      i: msg.i,
      d: msg.d,
      i_clamp: msg.i_clamp,
      antiwindup: msg.antiwindup,
    }
  }
}


// Corresponds to control_msgs__msg__JointJog
/// Used in time-stamping the message.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointJog {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Name list of the joints. You don't need to specify all joints of the
    /// robot. Joint names are case-sensitive.
    pub joint_names: Vec<std::string::String>,

    /// A position command to the joints listed in joint_names.
    /// The order must be identical.
    /// Units are meters or radians.
    /// If displacements and velocities are filled, a profiled motion is requested.
    /// or position_deltas
    pub displacements: Vec<f64>,

    /// A velocity command to the joints listed in joint_names.
    /// The order must be identical.
    /// Units are m/s or rad/s.
    /// If displacements and velocities are filled, a profiled motion is requested.
    pub velocities: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: f64,

}



impl Default for JointJog {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointJog::default())
  }
}

impl rosidl_runtime_rs::Message for JointJog {
  type RmwMsg = super::msg::rmw::JointJog;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        displacements: msg.displacements.as_slice().into(),
        velocities: msg.velocities.as_slice().into(),
        duration: msg.duration,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        displacements: msg.displacements.as_slice().into(),
        velocities: msg.velocities.as_slice().into(),
      duration: msg.duration,
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
      displacements: msg.displacements.into(),
      velocities: msg.velocities.into(),
      duration: msg.duration,
    }
  }
}


// Corresponds to control_msgs__msg__JointTolerance
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTolerance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

    /// in radians or meters (for a revolute or prismatic joint, respectively)
    pub position: f64,

    /// in rad/sec or m/sec
    pub velocity: f64,

    /// in rad/sec^2 or m/sec^2
    pub acceleration: f64,

}



impl Default for JointTolerance {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointTolerance::default())
  }
}

impl rosidl_runtime_rs::Message for JointTolerance {
  type RmwMsg = super::msg::rmw::JointTolerance;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        position: msg.position,
        velocity: msg.velocity,
        acceleration: msg.acceleration,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      position: msg.position,
      velocity: msg.velocity,
      acceleration: msg.acceleration,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      position: msg.position,
      velocity: msg.velocity,
      acceleration: msg.acceleration,
    }
  }
}


// Corresponds to control_msgs__msg__JointTrajectoryControllerState
/// This message presents current controller state of JTC

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointTrajectoryControllerState {
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_names: Vec<std::string::String>,

    /// The set point, that is, desired state.
    pub reference: trajectory_msgs::msg::JointTrajectoryPoint,

    /// Current value of the process (ie: latest sensor measurement on the controlled value).
    pub feedback: trajectory_msgs::msg::JointTrajectoryPoint,

    /// The error of the controlled value, essentially reference - feedback (for a regular PID implementation).
    pub error: trajectory_msgs::msg::JointTrajectoryPoint,

    /// Current output of the controller.
    pub output: trajectory_msgs::msg::JointTrajectoryPoint,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multi_dof_joint_names: Vec<std::string::String>,

    /// The set point, that is, desired state.
    pub multi_dof_reference: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint,

    /// Current value of the process (ie: latest sensor measurement on the controlled value).
    pub multi_dof_feedback: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint,

    /// The error of the controlled value, essentially reference - feedback (for a regular PID implementation).
    pub multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint,

    /// Current output of the controller.
    pub multi_dof_output: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint,

    /// The speed scaling factor the trajectory is currently being executed with
    pub speed_scaling_factor: f64,

}



impl Default for JointTrajectoryControllerState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointTrajectoryControllerState::default())
  }
}

impl rosidl_runtime_rs::Message for JointTrajectoryControllerState {
  type RmwMsg = super::msg::rmw::JointTrajectoryControllerState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        joint_names: msg.joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        reference: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.reference)).into_owned(),
        feedback: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
        error: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.error)).into_owned(),
        output: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.output)).into_owned(),
        multi_dof_joint_names: msg.multi_dof_joint_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        multi_dof_reference: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_reference)).into_owned(),
        multi_dof_feedback: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_feedback)).into_owned(),
        multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_error)).into_owned(),
        multi_dof_output: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Owned(msg.multi_dof_output)).into_owned(),
        speed_scaling_factor: msg.speed_scaling_factor,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        joint_names: msg.joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        reference: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.reference)).into_owned(),
        feedback: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
        error: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error)).into_owned(),
        output: trajectory_msgs::msg::JointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.output)).into_owned(),
        multi_dof_joint_names: msg.multi_dof_joint_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        multi_dof_reference: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_reference)).into_owned(),
        multi_dof_feedback: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_feedback)).into_owned(),
        multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_error)).into_owned(),
        multi_dof_output: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::into_rmw_message(std::borrow::Cow::Borrowed(&msg.multi_dof_output)).into_owned(),
      speed_scaling_factor: msg.speed_scaling_factor,
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
      reference: trajectory_msgs::msg::JointTrajectoryPoint::from_rmw_message(msg.reference),
      feedback: trajectory_msgs::msg::JointTrajectoryPoint::from_rmw_message(msg.feedback),
      error: trajectory_msgs::msg::JointTrajectoryPoint::from_rmw_message(msg.error),
      output: trajectory_msgs::msg::JointTrajectoryPoint::from_rmw_message(msg.output),
      multi_dof_joint_names: msg.multi_dof_joint_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      multi_dof_reference: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_rmw_message(msg.multi_dof_reference),
      multi_dof_feedback: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_rmw_message(msg.multi_dof_feedback),
      multi_dof_error: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_rmw_message(msg.multi_dof_error),
      multi_dof_output: trajectory_msgs::msg::MultiDOFJointTrajectoryPoint::from_rmw_message(msg.multi_dof_output),
      speed_scaling_factor: msg.speed_scaling_factor,
    }
  }
}


// Corresponds to control_msgs__msg__MecanumDriveControllerState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MecanumDriveControllerState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    pub reference_velocity: geometry_msgs::msg::Twist,

}



impl Default for MecanumDriveControllerState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MecanumDriveControllerState::default())
  }
}

impl rosidl_runtime_rs::Message for MecanumDriveControllerState {
  type RmwMsg = super::msg::rmw::MecanumDriveControllerState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        front_left_wheel_velocity: msg.front_left_wheel_velocity,
        back_left_wheel_velocity: msg.back_left_wheel_velocity,
        back_right_wheel_velocity: msg.back_right_wheel_velocity,
        front_right_wheel_velocity: msg.front_right_wheel_velocity,
        reference_velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.reference_velocity)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      front_left_wheel_velocity: msg.front_left_wheel_velocity,
      back_left_wheel_velocity: msg.back_left_wheel_velocity,
      back_right_wheel_velocity: msg.back_right_wheel_velocity,
      front_right_wheel_velocity: msg.front_right_wheel_velocity,
        reference_velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.reference_velocity)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      front_left_wheel_velocity: msg.front_left_wheel_velocity,
      back_left_wheel_velocity: msg.back_left_wheel_velocity,
      back_right_wheel_velocity: msg.back_right_wheel_velocity,
      front_right_wheel_velocity: msg.front_right_wheel_velocity,
      reference_velocity: geometry_msgs::msg::Twist::from_rmw_message(msg.reference_velocity),
    }
  }
}


// Corresponds to control_msgs__msg__MotionArgument
/// Key Value pair to be used in MotionPrimitive messages

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionArgument {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: f64,

}



impl Default for MotionArgument {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionArgument::default())
  }
}

impl rosidl_runtime_rs::Message for MotionArgument {
  type RmwMsg = super::msg::rmw::MotionArgument;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      value: msg.value,
    }
  }
}


// Corresponds to control_msgs__msg__MotionPrimitive

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub additional_arguments: Vec<super::msg::MotionArgument>,

    /// Targets should be either specified through joint configurations or Cartesian poses.
    /// Depending on the motion type and implementation, there might be multiple poses allowed.
    /// For example, circular motions are often specified as via and target
    /// Usually one of these is empty, in some edge cases even both can be empty, when all input is given
    /// in the more flexible MotionArgument[] field.
    pub poses: Vec<geometry_msgs::msg::PoseStamped>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_positions: Vec<f64>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionPrimitive::default())
  }
}

impl rosidl_runtime_rs::Message for MotionPrimitive {
  type RmwMsg = super::msg::rmw::MotionPrimitive;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_,
        blend_radius: msg.blend_radius,
        additional_arguments: msg.additional_arguments
          .into_iter()
          .map(|elem| super::msg::MotionArgument::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        poses: msg.poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        joint_positions: msg.joint_positions.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      type_: msg.type_,
      blend_radius: msg.blend_radius,
        additional_arguments: msg.additional_arguments
          .iter()
          .map(|elem| super::msg::MotionArgument::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        poses: msg.poses
          .iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        joint_positions: msg.joint_positions.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      type_: msg.type_,
      blend_radius: msg.blend_radius,
      additional_arguments: msg.additional_arguments
          .into_iter()
          .map(super::msg::MotionArgument::from_rmw_message)
          .collect(),
      poses: msg.poses
          .into_iter()
          .map(geometry_msgs::msg::PoseStamped::from_rmw_message)
          .collect(),
      joint_positions: msg.joint_positions.into(),
    }
  }
}


// Corresponds to control_msgs__msg__MotionPrimitiveSequence

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotionPrimitiveSequence {

    // This member is not documented.
    #[allow(missing_docs)]
    pub motions: Vec<super::msg::MotionPrimitive>,

}



impl Default for MotionPrimitiveSequence {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotionPrimitiveSequence::default())
  }
}

impl rosidl_runtime_rs::Message for MotionPrimitiveSequence {
  type RmwMsg = super::msg::rmw::MotionPrimitiveSequence;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motions: msg.motions
          .into_iter()
          .map(|elem| super::msg::MotionPrimitive::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        motions: msg.motions
          .iter()
          .map(|elem| super::msg::MotionPrimitive::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      motions: msg.motions
          .into_iter()
          .map(super::msg::MotionPrimitive::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to control_msgs__msg__MultiDOFCommand
/// The message defines command for multiple degrees of freedom (DoF) typically used by many controllers.
/// The message intentionally avoids 'joint' nomenclature because it can be generally use for command with
/// different semantic meanings, e.g., joints, Cartesian axes, or have abstract meaning like GPIO interface.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MultiDOFCommand {
    /// names of degrees of freedom
    pub dof_names: Vec<std::string::String>,

    /// values used by most of the controller
    pub values: Vec<f64>,

    /// First derivation of the values, e.g., velocity if values are positions.
    /// This is useful for PID and similar controllers.
    pub values_dot: Vec<f64>,

}



impl Default for MultiDOFCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MultiDOFCommand::default())
  }
}

impl rosidl_runtime_rs::Message for MultiDOFCommand {
  type RmwMsg = super::msg::rmw::MultiDOFCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        dof_names: msg.dof_names
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        values: msg.values.as_slice().into(),
        values_dot: msg.values_dot.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        dof_names: msg.dof_names
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        values: msg.values.as_slice().into(),
        values_dot: msg.values_dot.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      dof_names: msg.dof_names
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      values: msg.values.into(),
      values_dot: msg.values_dot.into(),
    }
  }
}


// Corresponds to control_msgs__msg__MultiDOFStateStamped
/// This message presents current controller state of multiple degrees of freedom.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MultiDOFStateStamped {
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dof_states: Vec<super::msg::SingleDOFState>,

}



impl Default for MultiDOFStateStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MultiDOFStateStamped::default())
  }
}

impl rosidl_runtime_rs::Message for MultiDOFStateStamped {
  type RmwMsg = super::msg::rmw::MultiDOFStateStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        dof_states: msg.dof_states
          .into_iter()
          .map(|elem| super::msg::SingleDOFState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        dof_states: msg.dof_states
          .iter()
          .map(|elem| super::msg::SingleDOFState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      dof_states: msg.dof_states
          .into_iter()
          .map(super::msg::SingleDOFState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to control_msgs__msg__PidState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PidState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestep: builtin_interfaces::msg::Duration,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PidState::default())
  }
}

impl rosidl_runtime_rs::Message for PidState {
  type RmwMsg = super::msg::rmw::PidState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        timestep: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.timestep)).into_owned(),
        error: msg.error,
        error_dot: msg.error_dot,
        p_error: msg.p_error,
        i_error: msg.i_error,
        d_error: msg.d_error,
        p_term: msg.p_term,
        i_term: msg.i_term,
        d_term: msg.d_term,
        i_max: msg.i_max,
        i_min: msg.i_min,
        output: msg.output,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        timestep: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.timestep)).into_owned(),
      error: msg.error,
      error_dot: msg.error_dot,
      p_error: msg.p_error,
      i_error: msg.i_error,
      d_error: msg.d_error,
      p_term: msg.p_term,
      i_term: msg.i_term,
      d_term: msg.d_term,
      i_max: msg.i_max,
      i_min: msg.i_min,
      output: msg.output,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      timestep: builtin_interfaces::msg::Duration::from_rmw_message(msg.timestep),
      error: msg.error,
      error_dot: msg.error_dot,
      p_error: msg.p_error,
      i_error: msg.i_error,
      d_error: msg.d_error,
      p_term: msg.p_term,
      i_term: msg.i_term,
      d_term: msg.d_term,
      i_max: msg.i_max,
      i_min: msg.i_min,
      output: msg.output,
    }
  }
}


// Corresponds to control_msgs__msg__SingleDOFState
/// This message presents current controller state of one degree of freedom.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleDOFState {
    /// DoF name, e.g., joint or Cartesian axis name
    pub name: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SingleDOFState::default())
  }
}

impl rosidl_runtime_rs::Message for SingleDOFState {
  type RmwMsg = super::msg::rmw::SingleDOFState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        reference: msg.reference,
        feedback: msg.feedback,
        feedback_dot: msg.feedback_dot,
        error: msg.error,
        error_dot: msg.error_dot,
        time_step: msg.time_step,
        output: msg.output,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      reference: msg.reference,
      feedback: msg.feedback,
      feedback_dot: msg.feedback_dot,
      error: msg.error,
      error_dot: msg.error_dot,
      time_step: msg.time_step,
      output: msg.output,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      reference: msg.reference,
      feedback: msg.feedback,
      feedback_dot: msg.feedback_dot,
      error: msg.error,
      error_dot: msg.error_dot,
      time_step: msg.time_step,
      output: msg.output,
    }
  }
}


// Corresponds to control_msgs__msg__SingleDOFStateStamped
/// This message presents current controller state of one degree of freedom.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SingleDOFStateStamped {
    /// Header timestamp should be update time of controller state
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dof_state: super::msg::SingleDOFState,

}



impl Default for SingleDOFStateStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SingleDOFStateStamped::default())
  }
}

impl rosidl_runtime_rs::Message for SingleDOFStateStamped {
  type RmwMsg = super::msg::rmw::SingleDOFStateStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        dof_state: super::msg::SingleDOFState::into_rmw_message(std::borrow::Cow::Owned(msg.dof_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        dof_state: super::msg::SingleDOFState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.dof_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      dof_state: super::msg::SingleDOFState::from_rmw_message(msg.dof_state),
    }
  }
}


// Corresponds to control_msgs__msg__SteeringControllerStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SteeringControllerStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// positions of traction wheels if the robot is controlled by position
    pub traction_wheels_position: Vec<f64>,

    /// velocities of traction wheels if the robot is controlled by velocity
    pub traction_wheels_velocity: Vec<f64>,

    /// positions of steering joints
    pub steer_positions: Vec<f64>,

    /// value commanded to tractions joint
    pub linear_velocity_command: Vec<f64>,

    /// values commanded to steering joints
    pub steering_angle_command: Vec<f64>,

}



impl Default for SteeringControllerStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SteeringControllerStatus::default())
  }
}

impl rosidl_runtime_rs::Message for SteeringControllerStatus {
  type RmwMsg = super::msg::rmw::SteeringControllerStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        traction_wheels_position: msg.traction_wheels_position.as_slice().into(),
        traction_wheels_velocity: msg.traction_wheels_velocity.as_slice().into(),
        steer_positions: msg.steer_positions.as_slice().into(),
        linear_velocity_command: msg.linear_velocity_command.as_slice().into(),
        steering_angle_command: msg.steering_angle_command.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        traction_wheels_position: msg.traction_wheels_position.as_slice().into(),
        traction_wheels_velocity: msg.traction_wheels_velocity.as_slice().into(),
        steer_positions: msg.steer_positions.as_slice().into(),
        linear_velocity_command: msg.linear_velocity_command.as_slice().into(),
        steering_angle_command: msg.steering_angle_command.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      traction_wheels_position: msg.traction_wheels_position.into(),
      traction_wheels_velocity: msg.traction_wheels_velocity.into(),
      steer_positions: msg.steer_positions.into(),
      linear_velocity_command: msg.linear_velocity_command.into(),
      steering_angle_command: msg.steering_angle_command.into(),
    }
  }
}


// Corresponds to control_msgs__msg__SpeedScalingFactor
/// This message contains a scaling factor to scale trajectory execution. A factor of 1.0 means
/// execution at normal speed, a factor of 0.0 means a full pause.
/// Negative values are not allowed (Which should be checked by any instance consuming this message).

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedScalingFactor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub factor: f64,

}



impl Default for SpeedScalingFactor {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SpeedScalingFactor::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedScalingFactor {
  type RmwMsg = super::msg::rmw::SpeedScalingFactor;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        factor: msg.factor,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      factor: msg.factor,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      factor: msg.factor,
    }
  }
}


// Corresponds to control_msgs__msg__VDA5050SafetyState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VDA5050SafetyState {
    /// Enum {autoAck, manual, remote, none} Acknowledge-Type of eStop:
    /// autoAck: autoacknowledgeable e-stop is activated e.g. by bumper or protective field
    /// manual: e-stop has to be acknowledged manually at the vehicle
    /// remote: facility estop has to be acknowledged remotely
    /// none: no e-stop activated
    pub e_stop: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VDA5050SafetyState::default())
  }
}

impl rosidl_runtime_rs::Message for VDA5050SafetyState {
  type RmwMsg = super::msg::rmw::VDA5050SafetyState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        e_stop: msg.e_stop.as_str().into(),
        field_violation: msg.field_violation,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        e_stop: msg.e_stop.as_str().into(),
      field_violation: msg.field_violation,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      e_stop: msg.e_stop.to_string(),
      field_violation: msg.field_violation,
    }
  }
}


