#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to controller_manager_msgs__srv__ConfigureController_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConfigureController_Request {
    /// To configure a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// configured or not.
    pub name: std::string::String,

}



impl Default for ConfigureController_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ConfigureController_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ConfigureController_Request {
  type RmwMsg = super::srv::rmw::ConfigureController_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__ConfigureController_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConfigureController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for ConfigureController_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ConfigureController_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ConfigureController_Response {
  type RmwMsg = super::srv::rmw::ConfigureController_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ok: msg.ok,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ok: msg.ok,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ok: msg.ok,
    }
  }
}


// Corresponds to controller_manager_msgs__srv__ListControllers_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllers_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListControllers_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListControllers_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListControllers_Request {
  type RmwMsg = super::srv::rmw::ListControllers_Request;

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


// Corresponds to controller_manager_msgs__srv__ListControllers_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllers_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub controller: Vec<super::msg::ControllerState>,

}



impl Default for ListControllers_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListControllers_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListControllers_Response {
  type RmwMsg = super::srv::rmw::ListControllers_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        controller: msg.controller
          .into_iter()
          .map(|elem| super::msg::ControllerState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        controller: msg.controller
          .iter()
          .map(|elem| super::msg::ControllerState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      controller: msg.controller
          .into_iter()
          .map(super::msg::ControllerState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__ListControllerTypes_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllerTypes_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListControllerTypes_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListControllerTypes_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListControllerTypes_Request {
  type RmwMsg = super::srv::rmw::ListControllerTypes_Request;

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


// Corresponds to controller_manager_msgs__srv__ListControllerTypes_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListControllerTypes_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub types: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub base_classes: Vec<std::string::String>,

}



impl Default for ListControllerTypes_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListControllerTypes_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListControllerTypes_Response {
  type RmwMsg = super::srv::rmw::ListControllerTypes_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        types: msg.types
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        base_classes: msg.base_classes
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        types: msg.types
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        base_classes: msg.base_classes
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      types: msg.types
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      base_classes: msg.base_classes
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__ListHardwareComponents_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareComponents_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListHardwareComponents_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListHardwareComponents_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListHardwareComponents_Request {
  type RmwMsg = super::srv::rmw::ListHardwareComponents_Request;

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


// Corresponds to controller_manager_msgs__srv__ListHardwareComponents_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareComponents_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub component: Vec<super::msg::HardwareComponentState>,

}



impl Default for ListHardwareComponents_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListHardwareComponents_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListHardwareComponents_Response {
  type RmwMsg = super::srv::rmw::ListHardwareComponents_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        component: msg.component
          .into_iter()
          .map(|elem| super::msg::HardwareComponentState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        component: msg.component
          .iter()
          .map(|elem| super::msg::HardwareComponentState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      component: msg.component
          .into_iter()
          .map(super::msg::HardwareComponentState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__ListHardwareInterfaces_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareInterfaces_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListHardwareInterfaces_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListHardwareInterfaces_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListHardwareInterfaces_Request {
  type RmwMsg = super::srv::rmw::ListHardwareInterfaces_Request;

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


// Corresponds to controller_manager_msgs__srv__ListHardwareInterfaces_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListHardwareInterfaces_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command_interfaces: Vec<super::msg::HardwareInterface>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_interfaces: Vec<super::msg::HardwareInterface>,

}



impl Default for ListHardwareInterfaces_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListHardwareInterfaces_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListHardwareInterfaces_Response {
  type RmwMsg = super::srv::rmw::ListHardwareInterfaces_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
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


// Corresponds to controller_manager_msgs__srv__LoadController_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadController_Request {
    /// To load a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// constructed and initialized or not.
    pub name: std::string::String,

}



impl Default for LoadController_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadController_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LoadController_Request {
  type RmwMsg = super::srv::rmw::LoadController_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__LoadController_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for LoadController_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadController_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LoadController_Response {
  type RmwMsg = super::srv::rmw::LoadController_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ok: msg.ok,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ok: msg.ok,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ok: msg.ok,
    }
  }
}


// Corresponds to controller_manager_msgs__srv__ReloadControllerLibraries_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ReloadControllerLibraries_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ReloadControllerLibraries_Request {
  type RmwMsg = super::srv::rmw::ReloadControllerLibraries_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        force_kill: msg.force_kill,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      force_kill: msg.force_kill,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      force_kill: msg.force_kill,
    }
  }
}


// Corresponds to controller_manager_msgs__srv__ReloadControllerLibraries_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReloadControllerLibraries_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for ReloadControllerLibraries_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ReloadControllerLibraries_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ReloadControllerLibraries_Response {
  type RmwMsg = super::srv::rmw::ReloadControllerLibraries_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ok: msg.ok,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ok: msg.ok,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ok: msg.ok,
    }
  }
}


// Corresponds to controller_manager_msgs__srv__SetHardwareComponentState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetHardwareComponentState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_state: lifecycle_msgs::msg::State,

}



impl Default for SetHardwareComponentState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetHardwareComponentState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetHardwareComponentState_Request {
  type RmwMsg = super::srv::rmw::SetHardwareComponentState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        target_state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Owned(msg.target_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        target_state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      target_state: lifecycle_msgs::msg::State::from_rmw_message(msg.target_state),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__SetHardwareComponentState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetHardwareComponentState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: lifecycle_msgs::msg::State,

}



impl Default for SetHardwareComponentState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetHardwareComponentState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetHardwareComponentState_Response {
  type RmwMsg = super::srv::rmw::SetHardwareComponentState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ok: msg.ok,
        state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ok: msg.ok,
        state: lifecycle_msgs::msg::State::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ok: msg.ok,
      state: lifecycle_msgs::msg::State::from_rmw_message(msg.state),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__SwitchController_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub activate_controllers: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub deactivate_controllers: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub strictness: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub activate_asap: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timeout: builtin_interfaces::msg::Duration,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SwitchController_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SwitchController_Request {
  type RmwMsg = super::srv::rmw::SwitchController_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        activate_controllers: msg.activate_controllers
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        deactivate_controllers: msg.deactivate_controllers
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        strictness: msg.strictness,
        activate_asap: msg.activate_asap,
        timeout: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.timeout)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        activate_controllers: msg.activate_controllers
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        deactivate_controllers: msg.deactivate_controllers
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      strictness: msg.strictness,
      activate_asap: msg.activate_asap,
        timeout: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.timeout)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      activate_controllers: msg.activate_controllers
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      deactivate_controllers: msg.deactivate_controllers
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      strictness: msg.strictness,
      activate_asap: msg.activate_asap,
      timeout: builtin_interfaces::msg::Duration::from_rmw_message(msg.timeout),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__SwitchController_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SwitchController_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SwitchController_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SwitchController_Response {
  type RmwMsg = super::srv::rmw::SwitchController_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ok: msg.ok,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ok: msg.ok,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ok: msg.ok,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__UnloadController_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UnloadController_Request {
    /// To unload a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// unloaded or not
    pub name: std::string::String,

}



impl Default for UnloadController_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UnloadController_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UnloadController_Request {
  type RmwMsg = super::srv::rmw::UnloadController_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__UnloadController_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UnloadController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for UnloadController_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UnloadController_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UnloadController_Response {
  type RmwMsg = super::srv::rmw::UnloadController_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ok: msg.ok,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ok: msg.ok,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ok: msg.ok,
    }
  }
}


// Corresponds to controller_manager_msgs__srv__CleanupController_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CleanupController_Request {
    /// To cleanup a controller, specify the "name" of the controller.
    /// The return value "ok" indicates if the controller was successfully
    /// cleaned up or not
    pub name: std::string::String,

}



impl Default for CleanupController_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CleanupController_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CleanupController_Request {
  type RmwMsg = super::srv::rmw::CleanupController_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to controller_manager_msgs__srv__CleanupController_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CleanupController_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ok: bool,

}



impl Default for CleanupController_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CleanupController_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CleanupController_Response {
  type RmwMsg = super::srv::rmw::CleanupController_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ok: msg.ok,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ok: msg.ok,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ok: msg.ok,
    }
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


