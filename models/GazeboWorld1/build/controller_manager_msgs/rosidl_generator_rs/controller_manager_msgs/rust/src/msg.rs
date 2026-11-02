#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to controller_manager_msgs__msg__ControllerState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControllerState {
    /// controller name
    pub name: std::string::String,

    /// controller state: unconfigured, inactive, active, or finalized
    pub state: std::string::String,

    /// the controller class name, e.g. joint_trajectory_controller/JointTrajectoryController
    pub type_: std::string::String,

    /// true, if controller runs asynchronously. false, if controller runs synchronously
    pub is_async: bool,

    /// update rate of the controller in Hz
    pub update_rate: u16,

    /// command interfaces currently owned by controller
    pub claimed_interfaces: Vec<std::string::String>,

    /// command interfaces required by controller
    pub required_command_interfaces: Vec<std::string::String>,

    /// state interfaces required by controller
    pub required_state_interfaces: Vec<std::string::String>,

    /// specifies whether or not controller can export references for a controller chain
    pub is_chainable: bool,

    /// specifies whether or not controller's exported references are claimed by another controller
    pub is_chained: bool,

    /// state interfaces to be exported (only applicable if is_chainable is true)
    pub exported_state_interfaces: Vec<std::string::String>,

    /// references to be exported (only applicable if is_chainable is true)
    pub reference_interfaces: Vec<std::string::String>,

    /// specifies list of controllers and their exported references that the controller is chained to
    pub chain_connections: Vec<super::msg::ChainConnection>,

}



impl Default for ControllerState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ControllerState::default())
  }
}

impl rosidl_runtime_rs::Message for ControllerState {
  type RmwMsg = super::msg::rmw::ControllerState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        state: msg.state.as_str().into(),
        type_: msg.type_.as_str().into(),
        is_async: msg.is_async,
        update_rate: msg.update_rate,
        claimed_interfaces: msg.claimed_interfaces
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        required_command_interfaces: msg.required_command_interfaces
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        required_state_interfaces: msg.required_state_interfaces
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        is_chainable: msg.is_chainable,
        is_chained: msg.is_chained,
        exported_state_interfaces: msg.exported_state_interfaces
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        reference_interfaces: msg.reference_interfaces
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        chain_connections: msg.chain_connections
          .into_iter()
          .map(|elem| super::msg::ChainConnection::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        state: msg.state.as_str().into(),
        type_: msg.type_.as_str().into(),
      is_async: msg.is_async,
      update_rate: msg.update_rate,
        claimed_interfaces: msg.claimed_interfaces
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        required_command_interfaces: msg.required_command_interfaces
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        required_state_interfaces: msg.required_state_interfaces
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      is_chainable: msg.is_chainable,
      is_chained: msg.is_chained,
        exported_state_interfaces: msg.exported_state_interfaces
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        reference_interfaces: msg.reference_interfaces
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        chain_connections: msg.chain_connections
          .iter()
          .map(|elem| super::msg::ChainConnection::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      state: msg.state.to_string(),
      type_: msg.type_.to_string(),
      is_async: msg.is_async,
      update_rate: msg.update_rate,
      claimed_interfaces: msg.claimed_interfaces
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      required_command_interfaces: msg.required_command_interfaces
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      required_state_interfaces: msg.required_state_interfaces
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      is_chainable: msg.is_chainable,
      is_chained: msg.is_chained,
      exported_state_interfaces: msg.exported_state_interfaces
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      reference_interfaces: msg.reference_interfaces
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      chain_connections: msg.chain_connections
          .into_iter()
          .map(super::msg::ChainConnection::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to controller_manager_msgs__msg__ChainConnection

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChainConnection {
    /// name of controller exporting reference_interfaces
    pub name: std::string::String,

    /// list of reference exported by controller
    pub reference_interfaces: Vec<std::string::String>,

}



impl Default for ChainConnection {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ChainConnection::default())
  }
}

impl rosidl_runtime_rs::Message for ChainConnection {
  type RmwMsg = super::msg::rmw::ChainConnection;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        reference_interfaces: msg.reference_interfaces
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        reference_interfaces: msg.reference_interfaces
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      reference_interfaces: msg.reference_interfaces
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to controller_manager_msgs__msg__HardwareComponentState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareComponentState {
    /// Name of the hardware component
    pub name: std::string::String,

    /// Type of the hardware component
    pub type_: std::string::String,

    /// If the hardware component is running asynchronously
    pub is_async: bool,

    /// read/write rate of the hardware component in Hz
    pub rw_rate: u16,

    /// DEPRECATED
    pub class_type: std::string::String,

    /// The name of the plugin that is used to load the hardware component
    pub plugin_name: std::string::String,

    /// State of the hardware component
    pub state: lifecycle_msgs::msg::State,

    /// Command interfaces of the hardware component
    pub command_interfaces: Vec<super::msg::HardwareInterface>,

    /// State interfaces of the hardware component
    pub state_interfaces: Vec<super::msg::HardwareInterface>,

}



impl Default for HardwareComponentState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HardwareComponentState::default())
  }
}

impl rosidl_runtime_rs::Message for HardwareComponentState {
  type RmwMsg = super::msg::rmw::HardwareComponentState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        type_: msg.type_.as_str().into(),
        is_async: msg.is_async,
        rw_rate: msg.rw_rate,
        class_type: msg.class_type.as_str().into(),
        plugin_name: msg.plugin_name.as_str().into(),
        state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
        command_interfaces: msg.command_interfaces
          .into_iter()
          .map(|elem| super::msg::HardwareInterface::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        state_interfaces: msg.state_interfaces
          .into_iter()
          .map(|elem| super::msg::HardwareInterface::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        type_: msg.type_.as_str().into(),
      is_async: msg.is_async,
      rw_rate: msg.rw_rate,
        class_type: msg.class_type.as_str().into(),
        plugin_name: msg.plugin_name.as_str().into(),
        state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
        command_interfaces: msg.command_interfaces
          .iter()
          .map(|elem| super::msg::HardwareInterface::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        state_interfaces: msg.state_interfaces
          .iter()
          .map(|elem| super::msg::HardwareInterface::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      type_: msg.type_.to_string(),
      is_async: msg.is_async,
      rw_rate: msg.rw_rate,
      class_type: msg.class_type.to_string(),
      plugin_name: msg.plugin_name.to_string(),
      state: lifecycle_msgs::msg::State::from_rmw_message(msg.state),
      command_interfaces: msg.command_interfaces
          .into_iter()
          .map(super::msg::HardwareInterface::from_rmw_message)
          .collect(),
      state_interfaces: msg.state_interfaces
          .into_iter()
          .map(super::msg::HardwareInterface::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to controller_manager_msgs__msg__HardwareInterface

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HardwareInterface {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_type: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_available: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_claimed: bool,

}



impl Default for HardwareInterface {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HardwareInterface::default())
  }
}

impl rosidl_runtime_rs::Message for HardwareInterface {
  type RmwMsg = super::msg::rmw::HardwareInterface;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        data_type: msg.data_type.as_str().into(),
        is_available: msg.is_available,
        is_claimed: msg.is_claimed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        data_type: msg.data_type.as_str().into(),
      is_available: msg.is_available,
      is_claimed: msg.is_claimed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      data_type: msg.data_type.to_string(),
      is_available: msg.is_available,
      is_claimed: msg.is_claimed,
    }
  }
}


// Corresponds to controller_manager_msgs__msg__NamedLifecycleState
/// This message is used to provide information about the lifecycle state of the controller or the hardware components

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NamedLifecycleState {
    /// The name of the controller or hardware interface
    pub name: std::string::String,

    /// The current lifecycle state of the controller or hardware components
    pub state: lifecycle_msgs::msg::State,

}



impl Default for NamedLifecycleState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::NamedLifecycleState::default())
  }
}

impl rosidl_runtime_rs::Message for NamedLifecycleState {
  type RmwMsg = super::msg::rmw::NamedLifecycleState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      state: lifecycle_msgs::msg::State::from_rmw_message(msg.state),
    }
  }
}


// Corresponds to controller_manager_msgs__msg__ControllerManagerActivity
/// This message is used to provide the activity within the controller manager regarding the change in state of controllers and hardware interfaces

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControllerManagerActivity {
    /// The header is used to provide timestamp information
    pub header: std_msgs::msg::Header,

    /// The current state of the controllers
    pub controllers: Vec<super::msg::NamedLifecycleState>,

    /// The current state of the hardware components
    pub hardware_components: Vec<super::msg::NamedLifecycleState>,

}



impl Default for ControllerManagerActivity {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ControllerManagerActivity::default())
  }
}

impl rosidl_runtime_rs::Message for ControllerManagerActivity {
  type RmwMsg = super::msg::rmw::ControllerManagerActivity;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        controllers: msg.controllers
          .into_iter()
          .map(|elem| super::msg::NamedLifecycleState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        hardware_components: msg.hardware_components
          .into_iter()
          .map(|elem| super::msg::NamedLifecycleState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        controllers: msg.controllers
          .iter()
          .map(|elem| super::msg::NamedLifecycleState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        hardware_components: msg.hardware_components
          .iter()
          .map(|elem| super::msg::NamedLifecycleState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      controllers: msg.controllers
          .into_iter()
          .map(super::msg::NamedLifecycleState::from_rmw_message)
          .collect(),
      hardware_components: msg.hardware_components
          .into_iter()
          .map(super::msg::NamedLifecycleState::from_rmw_message)
          .collect(),
    }
  }
}


