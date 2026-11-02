// auto-generated DO NOT EDIT

#pragma once

#include <algorithm>
#include <array>
#include <functional>
#include <limits>
#include <mutex>
#include <rclcpp/node.hpp>
#include <rclcpp_lifecycle/lifecycle_node.hpp>
#include <rclcpp/logger.hpp>
#include <set>
#include <sstream>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include <fmt/core.h>
#include <fmt/format.h>
#include <fmt/ranges.h>

#include <rsl/static_string.hpp>
#include <rsl/static_vector.hpp>
#include <rsl/parameter_validators.hpp>

#include "example_validators.hpp"


namespace admittance_controller {

// Use validators from RSL
using rsl::unique;
using rsl::subset_of;
using rsl::fixed_size;
using rsl::size_gt;
using rsl::size_lt;
using rsl::not_empty;
using rsl::element_bounds;
using rsl::lower_element_bounds;
using rsl::upper_element_bounds;
using rsl::bounds;
using rsl::lt;
using rsl::gt;
using rsl::lt_eq;
using rsl::gt_eq;
using rsl::one_of;
using rsl::to_parameter_result_msg;

template <typename T>
[[nodiscard]] auto to_parameter_value(T value) {
    return rclcpp::ParameterValue(value);
}

template <size_t capacity>
[[nodiscard]] auto to_parameter_value(rsl::StaticString<capacity> const& value) {
    return rclcpp::ParameterValue(rsl::to_string(value));
}

template <typename T, size_t capacity>
[[nodiscard]] auto to_parameter_value(rsl::StaticVector<T, capacity> const& value) {
    return rclcpp::ParameterValue(rsl::to_vector(value));
}
    struct Params {
        double scientific_notation_num = 1e-11;
        std::string interpolation_mode = "spline";
        std::vector<std::string> subset_selection = {"A", "B"};
        std::vector<std::string> joints = {"joint1", "joint2", "joint3"};
        std::vector<std::string> dof_names = {"x", "y", "rz"};
        std::vector<std::string> nested_map_struct_entries = {"A", "B"};
        rsl::StaticString<25> fixed_string = {"string_value"};
        rsl::StaticVector<double, 10> fixed_array = {{1.0, 2.3, 4.0, 5.4, 3.3}};
        rsl::StaticString<25> fixed_string_no_default;
        rsl::StaticVector<std::string, 5> fixed_string_array = {{"alpha", "beta", "gamma", "delta", "epsilon"}};
        rsl::StaticVector<std::string, 5> fixed_string_array_no_default;
        std::vector<std::string> command_interfaces;
        std::vector<std::string> state_interfaces;
        std::vector<std::string> chainable_command_interfaces;
        std::vector<double> acceleration_limits = {0.0, 0.0, 0.0};
        bool enable_parameter_update_without_reactivation = true;
        bool use_feedforward_commanded_input = false;
        int64_t lt_eq_fifteen = 1;
        int64_t gt_fifteen = 16;
        int64_t gt_fifteen_lt_eq_twenty = 20;
        int64_t gt_fifteen_lt_twenty = 16;
        double lt_eq_fifteen_double = 1.0;
        double gt_fifteen_double = 16.0;
        double gt_fifteen_lt_eq_twenty_double = 20.0;
        double gt_fifteen_lt_twenty_double = 16.0;
        int64_t one_number = 14540;
        std::vector<int64_t> three_numbers = {3, 4, 5};
        rsl::StaticVector<int64_t, 5> three_numbers_of_five = {{3, 3, 3}};
        int64_t hover_override = 1;
        bool angle_wraparound = false;
        bool open_loop_control = false;
        struct MapJoints {
            struct MapDofNames {
                double weight = 1.0;
                std::vector<double> limit = {0.0, 0.0, 0.0};
            };
            std::map<std::string, MapDofNames> dof_names_map;
        };
        std::map<std::string, MapJoints> joints_map;
        struct NestedDynamic {
            struct MapJoints {
                struct MapDofNames {
                    double nested = 1.0;
                    struct MapJoints {
                        struct MapDofNames {
                            double nested_deep = 1.0;
                        };
                        std::map<std::string, MapDofNames> dof_names_map;
                    };
                    std::map<std::string, MapJoints> joints_map;
                };
                std::map<std::string, MapDofNames> dof_names_map;
            };
            std::map<std::string, MapJoints> joints_map;
        } nested_dynamic;
        struct Pid {
            double rate = 0.005;
            struct MapJoints {
                double p = 1.0;
                double i = 1.0;
                double d = 1.0;
            };
            std::map<std::string, MapJoints> joints_map;
        } pid;
        struct Gains {
            struct MapDofNames {
                double k = 2.0;
            };
            std::map<std::string, MapDofNames> dof_names_map;
        } gains;
        struct NestedMap {
            std::vector<std::string> entries = {"entry1", "entry2"};
            struct MapEntries {
                double value = 1.0;
            };
            std::map<std::string, MapEntries> entries_map;
        } nested_map;
        struct NestedMapStruct {
            struct MapNestedMapStructEntries {
                struct NestedStruct {
                    std::string nested_struct_field = "nested_struct_field";
                } nested_struct;
            };
            std::map<std::string, MapNestedMapStructEntries> nested_map_struct_entries_map;
        } nested_map_struct;
        struct Kinematics {
            std::string plugin_name;
            std::string plugin_package;
            std::string base;
            std::string tip;
            double alpha = 0.0005;
            std::string group_name;
        } kinematics;
        struct FtSensor {
            std::string name;
            double filter_coefficient = 0.005;
            struct Frame {
                std::string id;
                bool external = false;
            } frame;
        } ft_sensor;
        struct Control {
            struct Frame {
                std::string id;
                bool external = false;
            } frame;
        } control;
        struct FixedWorldFrame {
            struct Frame {
                std::string id;
                bool external = false;
            } frame;
        } fixed_world_frame;
        struct GravityCompensation {
            struct Frame {
                std::string id;
                bool external = false;
            } frame;
            struct Cog {
                std::vector<double> pos;
                double force = std::numeric_limits<double>::quiet_NaN();
            } CoG;
        } gravity_compensation;
        struct Admittance {
            std::vector<bool> selected_axes;
            std::vector<double> mass;
            std::vector<double> damping_ratio;
            std::vector<double> stiffness;
        } admittance;
        // for detecting if the parameter struct has been updated
        rclcpp::Time __stamp;
    };
    struct StackParams {
        double scientific_notation_num = 1e-11;
        rsl::StaticString<25> fixed_string = {"string_value"};
        rsl::StaticVector<double, 10> fixed_array = {{1.0, 2.3, 4.0, 5.4, 3.3}};
        rsl::StaticString<25> fixed_string_no_default;
        rsl::StaticVector<std::string, 5> fixed_string_array = {{"alpha", "beta", "gamma", "delta", "epsilon"}};
        rsl::StaticVector<std::string, 5> fixed_string_array_no_default;
        bool enable_parameter_update_without_reactivation = true;
        bool use_feedforward_commanded_input = false;
        int64_t lt_eq_fifteen = 1;
        int64_t gt_fifteen = 16;
        int64_t gt_fifteen_lt_eq_twenty = 20;
        int64_t gt_fifteen_lt_twenty = 16;
        double lt_eq_fifteen_double = 1.0;
        double gt_fifteen_double = 16.0;
        double gt_fifteen_lt_eq_twenty_double = 20.0;
        double gt_fifteen_lt_twenty_double = 16.0;
        int64_t one_number = 14540;
        rsl::StaticVector<int64_t, 5> three_numbers_of_five = {{3, 3, 3}};
        int64_t hover_override = 1;
        bool angle_wraparound = false;
        bool open_loop_control = false;
        struct Pid {
            double rate = 0.005;
        } pid;
        struct Kinematics {
            double alpha = 0.0005;
        } kinematics;
        struct FtSensor {
            double filter_coefficient = 0.005;
            struct Frame {
                bool external = false;
            } frame;
        } ft_sensor;
        struct Control {
            struct Frame {
                bool external = false;
            } frame;
        } control;
        struct FixedWorldFrame {
            struct Frame {
                bool external = false;
            } frame;
        } fixed_world_frame;
        struct GravityCompensation {
            struct Frame {
                bool external = false;
            } frame;
            struct Cog {
                double force = std::numeric_limits<double>::quiet_NaN();
            } CoG;
        } gravity_compensation;
    };

  class ParamListener{
  public:
    // throws rclcpp::exceptions::InvalidParameterValueException on initialization if invalid parameter are loaded
    template <typename NodeT>
    ParamListener(NodeT node, std::string const& prefix = "")
    : ParamListener(node->get_node_parameters_interface(), node->get_logger(), prefix) {}

    ParamListener(const std::shared_ptr<rclcpp::node_interfaces::NodeParametersInterface>& parameters_interface,
                  std::string const& prefix = "")
    : ParamListener(parameters_interface, rclcpp::get_logger("admittance_controller"), prefix) {
      RCLCPP_DEBUG(logger_, "ParameterListener: Not using node logger, recommend using other constructors to use a node logger");
    }

    ParamListener(const std::shared_ptr<rclcpp::node_interfaces::NodeParametersInterface>& parameters_interface,
                  rclcpp::Logger logger, std::string const& prefix = "")
    : prefix_{prefix},
      logger_{std::move(logger)} {
      if (!prefix_.empty() && prefix_.back() != '.') {
        prefix_ += ".";
      }

      parameters_interface_ = parameters_interface;
      declare_params();
      auto update_param_cb = [this](const std::vector<rclcpp::Parameter> &parameters){return this->update(parameters);};
      handle_ = parameters_interface_->add_on_set_parameters_callback(update_param_cb);
      clock_ = rclcpp::Clock();
    }

    Params get_params() const{
      std::lock_guard<std::mutex> lock(mutex_);
      return params_;
    }

    /**
     * @brief Tries to update the parsed Params object
     * @param params_in The Params object to update
     * @return true if the Params object was updated, false if it was already up to date or the mutex could not be locked
     * @note This function tries to lock the mutex without blocking, so it can be used in a RT loop
     */
    bool try_update_params(Params & params_in) const {
      std::unique_lock<std::mutex> lock(mutex_, std::try_to_lock);
      if (lock.owns_lock()) {
        if (const bool is_old = params_in.__stamp != params_.__stamp; is_old) {
          params_in = params_;
          return true;
        }
      }
      return false;
    }

    /**
     * @brief Tries to get the current Params object
     * @param params_in The Params object to fill with the current parameters
     * @return true if mutex can be locked, false if mutex could not be locked
     * @note The parameters are only filled, when the mutex can be locked and the params timestamp is different
     * @note This function tries to lock the mutex without blocking, so it can be used in a RT loop
     */
    bool try_get_params(Params & params_in) const {
      if (mutex_.try_lock()) {
        if (const bool is_old = params_in.__stamp != params_.__stamp; is_old) {
          params_in = params_;
        }
        mutex_.unlock();
        return true;
      }
      return false;
    }

    bool is_old(Params const& other) const {
      std::lock_guard<std::mutex> lock(mutex_);
      return params_.__stamp != other.__stamp;
    }

    StackParams get_stack_params() {
      Params params = get_params();
      StackParams output;
      output.scientific_notation_num = params.scientific_notation_num;
      output.pid.rate = params.pid.rate;
      output.fixed_string = params.fixed_string;
      output.fixed_array = params.fixed_array;
      output.fixed_string_no_default = params.fixed_string_no_default;
      output.fixed_string_array = params.fixed_string_array;
      output.fixed_string_array_no_default = params.fixed_string_array_no_default;
      output.kinematics.alpha = params.kinematics.alpha;
      output.ft_sensor.frame.external = params.ft_sensor.frame.external;
      output.ft_sensor.filter_coefficient = params.ft_sensor.filter_coefficient;
      output.control.frame.external = params.control.frame.external;
      output.fixed_world_frame.frame.external = params.fixed_world_frame.frame.external;
      output.gravity_compensation.frame.external = params.gravity_compensation.frame.external;
      output.gravity_compensation.CoG.force = params.gravity_compensation.CoG.force;
      output.enable_parameter_update_without_reactivation = params.enable_parameter_update_without_reactivation;
      output.use_feedforward_commanded_input = params.use_feedforward_commanded_input;
      output.lt_eq_fifteen = params.lt_eq_fifteen;
      output.gt_fifteen = params.gt_fifteen;
      output.gt_fifteen_lt_eq_twenty = params.gt_fifteen_lt_eq_twenty;
      output.gt_fifteen_lt_twenty = params.gt_fifteen_lt_twenty;
      output.lt_eq_fifteen_double = params.lt_eq_fifteen_double;
      output.gt_fifteen_double = params.gt_fifteen_double;
      output.gt_fifteen_lt_eq_twenty_double = params.gt_fifteen_lt_eq_twenty_double;
      output.gt_fifteen_lt_twenty_double = params.gt_fifteen_lt_twenty_double;
      output.one_number = params.one_number;
      output.three_numbers_of_five = params.three_numbers_of_five;
      output.hover_override = params.hover_override;
      output.angle_wraparound = params.angle_wraparound;
      output.open_loop_control = params.open_loop_control;

      return output;
    }

    void refresh_dynamic_parameters() {
      auto updated_params = get_params();
      // TODO remove any destroyed dynamic parameters

      // declare any new dynamic parameters
      rclcpp::Parameter param;
      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {

          auto& entry = updated_params.joints_map[value_1].dof_names_map[value_2];
          std::string value = fmt::format("{}.{}", value_1, value_2);

          auto param_name = fmt::format("{}{}.{}", prefix_, value, "weight");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "map parameter without struct name";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = std::nextafter(0.0, std::numeric_limits<double>::infinity());
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.weight);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt<double>(param, 0.0);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.weight': {}", validation_result.error()));
          }
          entry.weight = param.as_double();}
      }

      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {

          auto& entry = updated_params.joints_map[value_1].dof_names_map[value_2];
          std::string value = fmt::format("{}.{}", value_1, value_2);

          auto param_name = fmt::format("{}{}.{}", prefix_, value, "limit");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "specifies limit for x, y and z axis";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = -10.0;
              descriptor.floating_point_range.at(0).to_value = 10.0;
              auto parameter = rclcpp::ParameterValue(entry.limit);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = fixed_size<double>(param, 3);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.limit': {}", validation_result.error()));
          }
          if(auto validation_result = lower_element_bounds<double>(param, -10.0);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.limit': {}", validation_result.error()));
          }
          if(auto validation_result = upper_element_bounds<double>(param, 10.0);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.limit': {}", validation_result.error()));
          }
          entry.limit = param.as_double_array();}
      }

      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {

          auto& entry = updated_params.nested_dynamic.joints_map[value_1].dof_names_map[value_2];
          std::string value = fmt::format("{}.{}", value_1, value_2);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_dynamic", value, "nested");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "test nested map params";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = 0.0001;
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.nested);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt_eq<double>(param, 0.0001);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'nested_dynamic.__map_joints.__map_dof_names.nested': {}", validation_result.error()));
          }
          entry.nested = param.as_double();}
      }

      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {
      for (const auto & value_3 : updated_params.joints) {
      for (const auto & value_4 : updated_params.dof_names) {

          auto& entry = updated_params.nested_dynamic.joints_map[value_1].dof_names_map[value_2].joints_map[value_3].dof_names_map[value_4];
          std::string value = fmt::format("{}.{}.{}.{}", value_1, value_2, value_3, value_4);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_dynamic", value, "nested_deep");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "test deep nested map params";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = 0.0001;
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.nested_deep);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt_eq<double>(param, 0.0001);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'nested_dynamic.__map_joints.__map_dof_names.__map_joints.__map_dof_names.nested_deep': {}", validation_result.error()));
          }
          entry.nested_deep = param.as_double();}
      }
      }
      }

      for (const auto & value_1 : updated_params.joints) {

          auto& entry = updated_params.pid.joints_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "p");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "proportional gain term";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = 0.0001;
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.p);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt_eq<double>(param, 0.0001);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'pid.__map_joints.p': {}", validation_result.error()));
          }
          entry.p = param.as_double();}

      for (const auto & value_1 : updated_params.joints) {

          auto& entry = updated_params.pid.joints_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "i");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "integral gain term";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.i);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.i = param.as_double();}

      for (const auto & value_1 : updated_params.joints) {

          auto& entry = updated_params.pid.joints_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "d");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "derivative gain term";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.d);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.d = param.as_double();}

      for (const auto & value_1 : updated_params.dof_names) {

          auto& entry = updated_params.gains.dof_names_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "gains", value, "k");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "general gain";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.k);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.k = param.as_double();}

      for (const auto & value_1 : updated_params.nested_map.entries) {

          auto& entry = updated_params.nested_map.entries_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_map", value, "value");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "A value in the nested map with sibling keys";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.value);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.value = param.as_double();}

      for (const auto & value_1 : updated_params.nested_map_struct_entries) {

          auto& entry = updated_params.nested_map_struct.nested_map_struct_entries_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_map_struct", value, "nested_struct.nested_struct_field");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "test fields in deeply nested structs";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.nested_struct.nested_struct_field);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.nested_struct.nested_struct_field = param.as_string();}

    }

    rcl_interfaces::msg::SetParametersResult update(const std::vector<rclcpp::Parameter> &parameters) {
      auto updated_params = get_params();
      bool any_param_updated = false;

      for (const auto &param: parameters) {
        if (param.get_name() == (prefix_ + "scientific_notation_num")) {
            updated_params.scientific_notation_num = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "interpolation_mode")) {
            if(auto validation_result = one_of<std::string>(param, {"spline", "linear"});
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = custom_validators::no_args_validator(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.interpolation_mode = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "subset_selection")) {
            if(auto validation_result = subset_of<std::string>(param, {"A", "B", "C"});
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.subset_selection = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "joints")) {
            updated_params.joints = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "dof_names")) {
            updated_params.dof_names = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "pid.rate")) {
            updated_params.pid.rate = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "nested_map.entries")) {
            updated_params.nested_map.entries = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "nested_map_struct_entries")) {
            updated_params.nested_map_struct_entries = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "fixed_string")) {
            if(auto validation_result = size_lt<std::string>(param, 26);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.fixed_string = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "fixed_array")) {
            if(auto validation_result = size_lt<double>(param, 11);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.fixed_array = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "fixed_string_no_default")) {
            if(auto validation_result = size_lt<std::string>(param, 26);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.fixed_string_no_default = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "fixed_string_array")) {
            if(auto validation_result = size_lt<std::string>(param, 6);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.fixed_string_array = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "fixed_string_array_no_default")) {
            if(auto validation_result = size_lt<std::string>(param, 6);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.fixed_string_array_no_default = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "command_interfaces")) {
            updated_params.command_interfaces = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "state_interfaces")) {
            updated_params.state_interfaces = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "chainable_command_interfaces")) {
            updated_params.chainable_command_interfaces = param.as_string_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.plugin_name")) {
            updated_params.kinematics.plugin_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.plugin_package")) {
            updated_params.kinematics.plugin_package = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.base")) {
            updated_params.kinematics.base = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.tip")) {
            updated_params.kinematics.tip = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.alpha")) {
            updated_params.kinematics.alpha = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.group_name")) {
            updated_params.kinematics.group_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "ft_sensor.name")) {
            updated_params.ft_sensor.name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "ft_sensor.frame.id")) {
            updated_params.ft_sensor.frame.id = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "ft_sensor.frame.external")) {
            updated_params.ft_sensor.frame.external = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "ft_sensor.filter_coefficient")) {
            updated_params.ft_sensor.filter_coefficient = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "control.frame.id")) {
            updated_params.control.frame.id = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "control.frame.external")) {
            updated_params.control.frame.external = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "fixed_world_frame.frame.id")) {
            updated_params.fixed_world_frame.frame.id = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "fixed_world_frame.frame.external")) {
            updated_params.fixed_world_frame.frame.external = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gravity_compensation.frame.id")) {
            updated_params.gravity_compensation.frame.id = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gravity_compensation.frame.external")) {
            updated_params.gravity_compensation.frame.external = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gravity_compensation.CoG.pos")) {
            if(auto validation_result = fixed_size<double>(param, 3);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.gravity_compensation.CoG.pos = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gravity_compensation.CoG.force")) {
            updated_params.gravity_compensation.CoG.force = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "admittance.selected_axes")) {
            if(auto validation_result = fixed_size<bool>(param, 6);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.admittance.selected_axes = param.as_bool_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "admittance.mass")) {
            if(auto validation_result = fixed_size<double>(param, 6);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = element_bounds<double>(param, 0.0001, 1000000.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.admittance.mass = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "admittance.damping_ratio")) {
            if(auto validation_result = fixed_size<double>(param, 6);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = custom_validators::validate_double_array_custom_func(param, 20.3, 5.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = element_bounds<double>(param, 0.1, 10.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.admittance.damping_ratio = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "admittance.stiffness")) {
            if(auto validation_result = element_bounds(param, 0.0001, 100000.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.admittance.stiffness = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "acceleration_limits")) {
            if(auto validation_result = fixed_size<double>(param, 3);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = lower_element_bounds<double>(param, -10.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = upper_element_bounds<double>(param, 10.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.acceleration_limits = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "enable_parameter_update_without_reactivation")) {
            updated_params.enable_parameter_update_without_reactivation = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "use_feedforward_commanded_input")) {
            updated_params.use_feedforward_commanded_input = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "lt_eq_fifteen")) {
            if(auto validation_result = lt_eq<int64_t>(param, 15);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.lt_eq_fifteen = param.as_int();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gt_fifteen")) {
            if(auto validation_result = gt<int64_t>(param, 15);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.gt_fifteen = param.as_int();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gt_fifteen_lt_eq_twenty")) {
            if(auto validation_result = gt<int64_t>(param, 15);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = lt_eq<int64_t>(param, 20);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.gt_fifteen_lt_eq_twenty = param.as_int();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gt_fifteen_lt_twenty")) {
            if(auto validation_result = gt<int64_t>(param, 15);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = lt<int64_t>(param, 20);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.gt_fifteen_lt_twenty = param.as_int();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "lt_eq_fifteen_double")) {
            if(auto validation_result = lt_eq<double>(param, 15.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.lt_eq_fifteen_double = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gt_fifteen_double")) {
            if(auto validation_result = gt<double>(param, 15.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.gt_fifteen_double = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gt_fifteen_lt_eq_twenty_double")) {
            if(auto validation_result = gt<double>(param, 15.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = lt_eq<double>(param, 20.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.gt_fifteen_lt_eq_twenty_double = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "gt_fifteen_lt_twenty_double")) {
            if(auto validation_result = gt<double>(param, 15.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            if(auto validation_result = lt<double>(param, 20.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.gt_fifteen_lt_twenty_double = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "one_number")) {
            if(auto validation_result = bounds<int64_t>(param, 1024, 65535);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.one_number = param.as_int();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "three_numbers")) {
            updated_params.three_numbers = param.as_integer_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "three_numbers_of_five")) {
            if(auto validation_result = size_lt<int64_t>(param, 6);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.three_numbers_of_five = param.as_integer_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "hover_override")) {
            if(auto validation_result = one_of<int64_t>(param, {0, 1, 2, -1});
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.hover_override = param.as_int();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angle_wraparound")) {
            updated_params.angle_wraparound = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "open_loop_control")) {
            updated_params.open_loop_control = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
      }

      // update dynamic parameters
      for (const auto &param: parameters) {
        for (const auto & value_1 : updated_params.joints) {
        for (const auto & value_2 : updated_params.dof_names) {
        std::string value = fmt::format("{}.{}", value_1, value_2);

            auto param_name = fmt::format("{}{}.{}", prefix_, value, "weight");
            if (param.get_name() == param_name) {
                if(auto validation_result = gt<double>(param, 0.0);
                  !validation_result) {
                    return rsl::to_parameter_result_msg(validation_result);
                }

                updated_params.joints_map[value_1].dof_names_map[value_2].weight = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }
        }

        for (const auto & value_1 : updated_params.joints) {
        for (const auto & value_2 : updated_params.dof_names) {
        std::string value = fmt::format("{}.{}", value_1, value_2);

            auto param_name = fmt::format("{}{}.{}", prefix_, value, "limit");
            if (param.get_name() == param_name) {
                if(auto validation_result = fixed_size<double>(param, 3);
                  !validation_result) {
                    return rsl::to_parameter_result_msg(validation_result);
                }
                if(auto validation_result = lower_element_bounds<double>(param, -10.0);
                  !validation_result) {
                    return rsl::to_parameter_result_msg(validation_result);
                }
                if(auto validation_result = upper_element_bounds<double>(param, 10.0);
                  !validation_result) {
                    return rsl::to_parameter_result_msg(validation_result);
                }

                updated_params.joints_map[value_1].dof_names_map[value_2].limit = param.as_double_array();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }
        }

        for (const auto & value_1 : updated_params.joints) {
        for (const auto & value_2 : updated_params.dof_names) {
        std::string value = fmt::format("{}.{}", value_1, value_2);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_dynamic", value, "nested");
            if (param.get_name() == param_name) {
                if(auto validation_result = gt_eq<double>(param, 0.0001);
                  !validation_result) {
                    return rsl::to_parameter_result_msg(validation_result);
                }

                updated_params.nested_dynamic.joints_map[value_1].dof_names_map[value_2].nested = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }
        }

        for (const auto & value_1 : updated_params.joints) {
        for (const auto & value_2 : updated_params.dof_names) {
        for (const auto & value_3 : updated_params.joints) {
        for (const auto & value_4 : updated_params.dof_names) {
        std::string value = fmt::format("{}.{}.{}.{}", value_1, value_2, value_3, value_4);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_dynamic", value, "nested_deep");
            if (param.get_name() == param_name) {
                if(auto validation_result = gt_eq<double>(param, 0.0001);
                  !validation_result) {
                    return rsl::to_parameter_result_msg(validation_result);
                }

                updated_params.nested_dynamic.joints_map[value_1].dof_names_map[value_2].joints_map[value_3].dof_names_map[value_4].nested_deep = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }
        }
        }
        }

        for (const auto & value_1 : updated_params.joints) {
        std::string value = fmt::format("{}", value_1);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "p");
            if (param.get_name() == param_name) {
                if(auto validation_result = gt_eq<double>(param, 0.0001);
                  !validation_result) {
                    return rsl::to_parameter_result_msg(validation_result);
                }

                updated_params.pid.joints_map[value_1].p = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }

        for (const auto & value_1 : updated_params.joints) {
        std::string value = fmt::format("{}", value_1);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "i");
            if (param.get_name() == param_name) {

                updated_params.pid.joints_map[value_1].i = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }

        for (const auto & value_1 : updated_params.joints) {
        std::string value = fmt::format("{}", value_1);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "d");
            if (param.get_name() == param_name) {

                updated_params.pid.joints_map[value_1].d = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }

        for (const auto & value_1 : updated_params.dof_names) {
        std::string value = fmt::format("{}", value_1);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "gains", value, "k");
            if (param.get_name() == param_name) {

                updated_params.gains.dof_names_map[value_1].k = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }

        for (const auto & value_1 : updated_params.nested_map.entries) {
        std::string value = fmt::format("{}", value_1);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_map", value, "value");
            if (param.get_name() == param_name) {

                updated_params.nested_map.entries_map[value_1].value = param.as_double();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }

        for (const auto & value_1 : updated_params.nested_map_struct_entries) {
        std::string value = fmt::format("{}", value_1);

            auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_map_struct", value, "nested_struct.nested_struct_field");
            if (param.get_name() == param_name) {

                updated_params.nested_map_struct.nested_map_struct_entries_map[value_1].nested_struct.nested_struct_field = param.as_string();
                any_param_updated = true;
                RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
            }
        }

      }
      if (any_param_updated) {
        updated_params.__stamp = clock_.now();
        update_internal_params(updated_params);
        if (user_callback_) {
          user_callback_(updated_params);
        }
      }
      return rsl::to_parameter_result_msg({});
    }

    void declare_params(){
      auto updated_params = get_params();
      // declare all parameters and give default values to non-required ones
      if (!parameters_interface_->has_parameter(prefix_ + "scientific_notation_num")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Test scientific notation";
          descriptor.read_only = false;
          descriptor.additional_constraints = "Any string can be here. For example, you might want to embed JSON schema";

          auto parameter = to_parameter_value(updated_params.scientific_notation_num);
          parameters_interface_->declare_parameter(prefix_ + "scientific_notation_num", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "interpolation_mode")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies which algorithm to use for interpolation.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.interpolation_mode);
          parameters_interface_->declare_parameter(prefix_ + "interpolation_mode", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "subset_selection")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "test subset of validator.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.subset_selection);
          parameters_interface_->declare_parameter(prefix_ + "subset_selection", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "joints")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies which joints will be used by the controller";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.joints);
          parameters_interface_->declare_parameter(prefix_ + "joints", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "dof_names")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies which joints will be used by the controller";
          descriptor.read_only = false;
          descriptor.additional_constraints = "Any string can be here. For example, you might want to embed JSON schema";

          auto parameter = to_parameter_value(updated_params.dof_names);
          parameters_interface_->declare_parameter(prefix_ + "dof_names", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "pid.rate")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "update loop period in seconds";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.pid.rate);
          parameters_interface_->declare_parameter(prefix_ + "pid.rate", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "nested_map.entries")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Keys for nested mapped parameters";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.nested_map.entries);
          parameters_interface_->declare_parameter(prefix_ + "nested_map.entries", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "nested_map_struct_entries")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Keys for nested map structs";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.nested_map_struct_entries);
          parameters_interface_->declare_parameter(prefix_ + "nested_map_struct_entries", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "fixed_string")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "test code generation for fixed sized string";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.fixed_string);
          parameters_interface_->declare_parameter(prefix_ + "fixed_string", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "fixed_array")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "test code generation for fixed sized array";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.fixed_array);
          parameters_interface_->declare_parameter(prefix_ + "fixed_array", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "fixed_string_no_default")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "test code generation for fixed sized string with no default";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "fixed_string_no_default", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "fixed_string_array")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "test code generation for fixed sized string array";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.fixed_string_array);
          parameters_interface_->declare_parameter(prefix_ + "fixed_string_array", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "fixed_string_array_no_default")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "test code generation for fixed sized string array with no default";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "fixed_string_array_no_default", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "command_interfaces")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies which command interfaces to claim";
          descriptor.read_only = true;
          descriptor.additional_constraints = "some additional constraints";

          auto parameter = rclcpp::ParameterType::PARAMETER_STRING_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "command_interfaces", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "state_interfaces")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies which state interfaces to claim";
          descriptor.read_only = true;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "state_interfaces", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "chainable_command_interfaces")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies which chainable interfaces to claim";
          descriptor.read_only = true;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "chainable_command_interfaces", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.plugin_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies which kinematics plugin to load";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "kinematics.plugin_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.plugin_package")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies the package to load the kinematics plugin from";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "kinematics.plugin_package", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.base")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies the base link of the robot description used by the kinematics plugin";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "kinematics.base", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.tip")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies the end effector link of the robot description used by the kinematics plugin";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "kinematics.tip", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.alpha")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies the damping coefficient for the Jacobian pseudo inverse";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.kinematics.alpha);
          parameters_interface_->declare_parameter(prefix_ + "kinematics.alpha", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.group_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies the group name for planning with Moveit";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "kinematics.group_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "ft_sensor.name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "name of the force torque sensor in the robot description";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "ft_sensor.name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "ft_sensor.frame.id")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "frame of the force torque sensor";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "ft_sensor.frame.id", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "ft_sensor.frame.external")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies if the force torque sensor is contained in the kinematics chain from the base to the tip";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.ft_sensor.frame.external);
          parameters_interface_->declare_parameter(prefix_ + "ft_sensor.frame.external", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "ft_sensor.filter_coefficient")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies the coefficient for the sensor's exponential filter";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.ft_sensor.filter_coefficient);
          parameters_interface_->declare_parameter(prefix_ + "ft_sensor.filter_coefficient", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "control.frame.id")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "control frame used for admittance control";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "control.frame.id", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "control.frame.external")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies if the control frame is contained in the kinematics chain from the base to the tip";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.control.frame.external);
          parameters_interface_->declare_parameter(prefix_ + "control.frame.external", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "fixed_world_frame.frame.id")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Define the \"world frame\".\n    Gravity points down (neg. Z) in this frame.";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "fixed_world_frame.frame.id", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "fixed_world_frame.frame.external")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies if the world frame is contained in the kinematics chain from the base to the tip";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.fixed_world_frame.frame.external);
          parameters_interface_->declare_parameter(prefix_ + "fixed_world_frame.frame.external", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gravity_compensation.frame.id")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "frame which center of gravity (CoG) is defined in";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_STRING;
          parameters_interface_->declare_parameter(prefix_ + "gravity_compensation.frame.id", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gravity_compensation.frame.external")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies if the center of gravity frame is contained in the kinematics chain from the base to the tip";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.gravity_compensation.frame.external);
          parameters_interface_->declare_parameter(prefix_ + "gravity_compensation.frame.external", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gravity_compensation.CoG.pos")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "position of the center of gravity (CoG) in its frame";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_DOUBLE_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "gravity_compensation.CoG.pos", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gravity_compensation.CoG.force")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "weight of the end effector, e.g mass * 9.81";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.gravity_compensation.CoG.force);
          parameters_interface_->declare_parameter(prefix_ + "gravity_compensation.CoG.force", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "admittance.selected_axes")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies if the axes x, y, z, rx, ry, and rz are enabled";
          descriptor.read_only = false;
          auto parameter = rclcpp::ParameterType::PARAMETER_BOOL_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "admittance.selected_axes", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "admittance.mass")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies mass values for x, y, z, rx, ry, and rz used in the admittance calculation";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = 0.0001;
          descriptor.floating_point_range.at(0).to_value = 1000000.0;
          auto parameter = rclcpp::ParameterType::PARAMETER_DOUBLE_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "admittance.mass", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "admittance.damping_ratio")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies damping ratio values for x, y, z, rx, ry, and rz used in the admittance calculation. The values are calculated as damping can be used instead: zeta = D / (2 * sqrt( M * S ))";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = 0.1;
          descriptor.floating_point_range.at(0).to_value = 10.0;
          auto parameter = rclcpp::ParameterType::PARAMETER_DOUBLE_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "admittance.damping_ratio", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "admittance.stiffness")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies stiffness values for x, y, z, rx, ry, and rz used in the admittance calculation";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = 0.0001;
          descriptor.floating_point_range.at(0).to_value = 100000.0;
          auto parameter = rclcpp::ParameterType::PARAMETER_DOUBLE_ARRAY;
          parameters_interface_->declare_parameter(prefix_ + "admittance.stiffness", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "acceleration_limits")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "specifies maximum acceleration limits for x, y and z axis";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = -10.0;
          descriptor.floating_point_range.at(0).to_value = 10.0;
          auto parameter = to_parameter_value(updated_params.acceleration_limits);
          parameters_interface_->declare_parameter(prefix_ + "acceleration_limits", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "enable_parameter_update_without_reactivation")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "if enabled, read_only parameters will be dynamically updated in the control loop";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.enable_parameter_update_without_reactivation);
          parameters_interface_->declare_parameter(prefix_ + "enable_parameter_update_without_reactivation", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "use_feedforward_commanded_input")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "if enabled, the velocity commanded to the admittance controller is added to its calculated admittance velocity";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.use_feedforward_commanded_input);
          parameters_interface_->declare_parameter(prefix_ + "use_feedforward_commanded_input", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "lt_eq_fifteen")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number less than or equal to 15";
          descriptor.read_only = false;
          descriptor.integer_range.resize(1);
          descriptor.integer_range.at(0).from_value = std::numeric_limits<int64_t>::lowest();
          descriptor.integer_range.at(0).to_value = 15;
          auto parameter = to_parameter_value(updated_params.lt_eq_fifteen);
          parameters_interface_->declare_parameter(prefix_ + "lt_eq_fifteen", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gt_fifteen")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number greater than 15";
          descriptor.read_only = false;
          descriptor.integer_range.resize(1);
          descriptor.integer_range.at(0).from_value = (15 + 1);
          descriptor.integer_range.at(0).to_value = std::numeric_limits<int64_t>::max();
          auto parameter = to_parameter_value(updated_params.gt_fifteen);
          parameters_interface_->declare_parameter(prefix_ + "gt_fifteen", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gt_fifteen_lt_eq_twenty")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number greater than 15 and less than or equal to 20";
          descriptor.read_only = false;
          descriptor.integer_range.resize(1);
          descriptor.integer_range.at(0).from_value = (15 + 1);
          descriptor.integer_range.at(0).to_value = 20;
          auto parameter = to_parameter_value(updated_params.gt_fifteen_lt_eq_twenty);
          parameters_interface_->declare_parameter(prefix_ + "gt_fifteen_lt_eq_twenty", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gt_fifteen_lt_twenty")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number greater than 15 and less than 20";
          descriptor.read_only = false;
          descriptor.integer_range.resize(1);
          descriptor.integer_range.at(0).from_value = (15 + 1);
          descriptor.integer_range.at(0).to_value = (20 - 1);
          auto parameter = to_parameter_value(updated_params.gt_fifteen_lt_twenty);
          parameters_interface_->declare_parameter(prefix_ + "gt_fifteen_lt_twenty", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "lt_eq_fifteen_double")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number less than or equal to 15";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = std::numeric_limits<double>::lowest();
          descriptor.floating_point_range.at(0).to_value = 15.0;
          auto parameter = to_parameter_value(updated_params.lt_eq_fifteen_double);
          parameters_interface_->declare_parameter(prefix_ + "lt_eq_fifteen_double", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gt_fifteen_double")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number greater than 15";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = std::nextafter(15.0, std::numeric_limits<double>::infinity());
          descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
          auto parameter = to_parameter_value(updated_params.gt_fifteen_double);
          parameters_interface_->declare_parameter(prefix_ + "gt_fifteen_double", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gt_fifteen_lt_eq_twenty_double")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number greater than 15 and less than or equal to 20";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = std::nextafter(15.0, std::numeric_limits<double>::infinity());
          descriptor.floating_point_range.at(0).to_value = 20.0;
          auto parameter = to_parameter_value(updated_params.gt_fifteen_lt_eq_twenty_double);
          parameters_interface_->declare_parameter(prefix_ + "gt_fifteen_lt_eq_twenty_double", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "gt_fifteen_lt_twenty_double")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "should be a number greater than 15 and less than 20";
          descriptor.read_only = false;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = std::nextafter(15.0, std::numeric_limits<double>::infinity());
          descriptor.floating_point_range.at(0).to_value = std::nextafter(20.0, -std::numeric_limits<double>::infinity());
          auto parameter = to_parameter_value(updated_params.gt_fifteen_lt_twenty_double);
          parameters_interface_->declare_parameter(prefix_ + "gt_fifteen_lt_twenty_double", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "one_number")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = true;
          descriptor.integer_range.resize(1);
          descriptor.integer_range.at(0).from_value = 1024;
          descriptor.integer_range.at(0).to_value = 65535;
          auto parameter = to_parameter_value(updated_params.one_number);
          parameters_interface_->declare_parameter(prefix_ + "one_number", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "three_numbers")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.three_numbers);
          parameters_interface_->declare_parameter(prefix_ + "three_numbers", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "three_numbers_of_five")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.three_numbers_of_five);
          parameters_interface_->declare_parameter(prefix_ + "three_numbers_of_five", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "hover_override")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Override hover action:\n    0: Hover\n    1: Push\n    2: Pull\n    -1: Do not override";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.hover_override);
          parameters_interface_->declare_parameter(prefix_ + "hover_override", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angle_wraparound")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "For joints that wrap around (without end stop, ie. are continuous) where the shortest rotation to the target position is the desired motion. If true, the position error :math:e = normalize(s_d - s) is normalized between :math:-\\pi, \\pi. Otherwise  :math:e = s_d - s is used, with the desired position :math:s_d and the measured position :math:s from the state interface.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angle_wraparound);
          parameters_interface_->declare_parameter(prefix_ + "angle_wraparound", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "open_loop_control")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Use controller in open-loop control mode \n    \n     * The controller ignores the states provided by hardware interface but using last commands as states for starting the trajectory interpolation.\n     * It deactivates the feedback control, see the gains structure. \n    \n     This is useful if hardware states are not following commands, i.e., an offset between those (typical for hydraulic manipulators). \n    \n     If this flag is set, the controller tries to read the values from the command interfaces on activation. If they have real numeric values, those will be used instead of state interfaces. Therefore it is important set command interfaces to NaN (i.e., std::numeric_limits<double>::quiet_NaN()) or state values when the hardware is started.\n    ";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.open_loop_control);
          parameters_interface_->declare_parameter(prefix_ + "open_loop_control", parameter, descriptor);
      }
      // get parameters and fill struct fields
      rclcpp::Parameter param;
      param = parameters_interface_->get_parameter(prefix_ + "scientific_notation_num");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "scientific_notation_num") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.scientific_notation_num = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "interpolation_mode");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "interpolation_mode") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = one_of<std::string>(param, {"spline", "linear"});
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'interpolation_mode': {}", validation_result.error()));
      }
      if(auto validation_result = custom_validators::no_args_validator(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'interpolation_mode': {}", validation_result.error()));
      }
      updated_params.interpolation_mode = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "subset_selection");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "subset_selection") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = subset_of<std::string>(param, {"A", "B", "C"});
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'subset_selection': {}", validation_result.error()));
      }
      updated_params.subset_selection = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "joints");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "joints") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.joints = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "dof_names");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "dof_names") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.dof_names = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "pid.rate");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "pid.rate") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.pid.rate = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "nested_map.entries");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "nested_map.entries") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.nested_map.entries = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "nested_map_struct_entries");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "nested_map_struct_entries") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.nested_map_struct_entries = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "fixed_string");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "fixed_string") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = size_lt<std::string>(param, 26);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'fixed_string': {}", validation_result.error()));
      }
      updated_params.fixed_string = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "fixed_array");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "fixed_array") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = size_lt<double>(param, 11);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'fixed_array': {}", validation_result.error()));
      }
      updated_params.fixed_array = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "fixed_string_no_default");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "fixed_string_no_default") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = size_lt<std::string>(param, 26);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'fixed_string_no_default': {}", validation_result.error()));
      }
      updated_params.fixed_string_no_default = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "fixed_string_array");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "fixed_string_array") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = size_lt<std::string>(param, 6);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'fixed_string_array': {}", validation_result.error()));
      }
      updated_params.fixed_string_array = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "fixed_string_array_no_default");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "fixed_string_array_no_default") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = size_lt<std::string>(param, 6);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'fixed_string_array_no_default': {}", validation_result.error()));
      }
      updated_params.fixed_string_array_no_default = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "command_interfaces");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "command_interfaces") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.command_interfaces = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "state_interfaces");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "state_interfaces") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.state_interfaces = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "chainable_command_interfaces");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "chainable_command_interfaces") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.chainable_command_interfaces = param.as_string_array();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.plugin_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.plugin_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.plugin_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.plugin_package");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.plugin_package") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.plugin_package = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.base");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.base") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.base = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.tip");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.tip") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.tip = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.alpha");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.alpha") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.alpha = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.group_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.group_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.group_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "ft_sensor.name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "ft_sensor.name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.ft_sensor.name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "ft_sensor.frame.id");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "ft_sensor.frame.id") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.ft_sensor.frame.id = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "ft_sensor.frame.external");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "ft_sensor.frame.external") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.ft_sensor.frame.external = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "ft_sensor.filter_coefficient");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "ft_sensor.filter_coefficient") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.ft_sensor.filter_coefficient = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "control.frame.id");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "control.frame.id") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.control.frame.id = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "control.frame.external");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "control.frame.external") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.control.frame.external = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "fixed_world_frame.frame.id");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "fixed_world_frame.frame.id") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.fixed_world_frame.frame.id = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "fixed_world_frame.frame.external");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "fixed_world_frame.frame.external") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.fixed_world_frame.frame.external = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "gravity_compensation.frame.id");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gravity_compensation.frame.id") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.gravity_compensation.frame.id = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "gravity_compensation.frame.external");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gravity_compensation.frame.external") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.gravity_compensation.frame.external = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "gravity_compensation.CoG.pos");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gravity_compensation.CoG.pos") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = fixed_size<double>(param, 3);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gravity_compensation.CoG.pos': {}", validation_result.error()));
      }
      updated_params.gravity_compensation.CoG.pos = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "gravity_compensation.CoG.force");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gravity_compensation.CoG.force") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.gravity_compensation.CoG.force = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "admittance.selected_axes");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "admittance.selected_axes") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = fixed_size<bool>(param, 6);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'admittance.selected_axes': {}", validation_result.error()));
      }
      updated_params.admittance.selected_axes = param.as_bool_array();
      param = parameters_interface_->get_parameter(prefix_ + "admittance.mass");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "admittance.mass") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = fixed_size<double>(param, 6);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'admittance.mass': {}", validation_result.error()));
      }
      if(auto validation_result = element_bounds<double>(param, 0.0001, 1000000.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'admittance.mass': {}", validation_result.error()));
      }
      updated_params.admittance.mass = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "admittance.damping_ratio");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "admittance.damping_ratio") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = fixed_size<double>(param, 6);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'admittance.damping_ratio': {}", validation_result.error()));
      }
      if(auto validation_result = custom_validators::validate_double_array_custom_func(param, 20.3, 5.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'admittance.damping_ratio': {}", validation_result.error()));
      }
      if(auto validation_result = element_bounds<double>(param, 0.1, 10.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'admittance.damping_ratio': {}", validation_result.error()));
      }
      updated_params.admittance.damping_ratio = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "admittance.stiffness");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "admittance.stiffness") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = element_bounds(param, 0.0001, 100000.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'admittance.stiffness': {}", validation_result.error()));
      }
      updated_params.admittance.stiffness = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "acceleration_limits");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "acceleration_limits") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = fixed_size<double>(param, 3);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'acceleration_limits': {}", validation_result.error()));
      }
      if(auto validation_result = lower_element_bounds<double>(param, -10.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'acceleration_limits': {}", validation_result.error()));
      }
      if(auto validation_result = upper_element_bounds<double>(param, 10.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'acceleration_limits': {}", validation_result.error()));
      }
      updated_params.acceleration_limits = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "enable_parameter_update_without_reactivation");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "enable_parameter_update_without_reactivation") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.enable_parameter_update_without_reactivation = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "use_feedforward_commanded_input");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "use_feedforward_commanded_input") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.use_feedforward_commanded_input = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "lt_eq_fifteen");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "lt_eq_fifteen") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = lt_eq<int64_t>(param, 15);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'lt_eq_fifteen': {}", validation_result.error()));
      }
      updated_params.lt_eq_fifteen = param.as_int();
      param = parameters_interface_->get_parameter(prefix_ + "gt_fifteen");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gt_fifteen") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = gt<int64_t>(param, 15);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen': {}", validation_result.error()));
      }
      updated_params.gt_fifteen = param.as_int();
      param = parameters_interface_->get_parameter(prefix_ + "gt_fifteen_lt_eq_twenty");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gt_fifteen_lt_eq_twenty") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = gt<int64_t>(param, 15);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_eq_twenty': {}", validation_result.error()));
      }
      if(auto validation_result = lt_eq<int64_t>(param, 20);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_eq_twenty': {}", validation_result.error()));
      }
      updated_params.gt_fifteen_lt_eq_twenty = param.as_int();
      param = parameters_interface_->get_parameter(prefix_ + "gt_fifteen_lt_twenty");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gt_fifteen_lt_twenty") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = gt<int64_t>(param, 15);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_twenty': {}", validation_result.error()));
      }
      if(auto validation_result = lt<int64_t>(param, 20);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_twenty': {}", validation_result.error()));
      }
      updated_params.gt_fifteen_lt_twenty = param.as_int();
      param = parameters_interface_->get_parameter(prefix_ + "lt_eq_fifteen_double");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "lt_eq_fifteen_double") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = lt_eq<double>(param, 15.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'lt_eq_fifteen_double': {}", validation_result.error()));
      }
      updated_params.lt_eq_fifteen_double = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "gt_fifteen_double");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gt_fifteen_double") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = gt<double>(param, 15.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_double': {}", validation_result.error()));
      }
      updated_params.gt_fifteen_double = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "gt_fifteen_lt_eq_twenty_double");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gt_fifteen_lt_eq_twenty_double") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = gt<double>(param, 15.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_eq_twenty_double': {}", validation_result.error()));
      }
      if(auto validation_result = lt_eq<double>(param, 20.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_eq_twenty_double': {}", validation_result.error()));
      }
      updated_params.gt_fifteen_lt_eq_twenty_double = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "gt_fifteen_lt_twenty_double");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "gt_fifteen_lt_twenty_double") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = gt<double>(param, 15.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_twenty_double': {}", validation_result.error()));
      }
      if(auto validation_result = lt<double>(param, 20.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'gt_fifteen_lt_twenty_double': {}", validation_result.error()));
      }
      updated_params.gt_fifteen_lt_twenty_double = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "one_number");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "one_number") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = bounds<int64_t>(param, 1024, 65535);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'one_number': {}", validation_result.error()));
      }
      updated_params.one_number = param.as_int();
      param = parameters_interface_->get_parameter(prefix_ + "three_numbers");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "three_numbers") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.three_numbers = param.as_integer_array();
      param = parameters_interface_->get_parameter(prefix_ + "three_numbers_of_five");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "three_numbers_of_five") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = size_lt<int64_t>(param, 6);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'three_numbers_of_five': {}", validation_result.error()));
      }
      updated_params.three_numbers_of_five = param.as_integer_array();
      param = parameters_interface_->get_parameter(prefix_ + "hover_override");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "hover_override") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = one_of<int64_t>(param, {0, 1, 2, -1});
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'hover_override': {}", validation_result.error()));
      }
      updated_params.hover_override = param.as_int();
      param = parameters_interface_->get_parameter(prefix_ + "angle_wraparound");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angle_wraparound") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.angle_wraparound = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "open_loop_control");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "open_loop_control") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.open_loop_control = param.as_bool();


      // declare and set all dynamic parameters
      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {

          auto& entry = updated_params.joints_map[value_1].dof_names_map[value_2];
          std::string value = fmt::format("{}.{}", value_1, value_2);

          auto param_name = fmt::format("{}{}.{}", prefix_, value, "weight");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "map parameter without struct name";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = std::nextafter(0.0, std::numeric_limits<double>::infinity());
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.weight);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt<double>(param, 0.0);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.weight': {}", validation_result.error()));
          }
          entry.weight = param.as_double();}
      }

      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {

          auto& entry = updated_params.joints_map[value_1].dof_names_map[value_2];
          std::string value = fmt::format("{}.{}", value_1, value_2);

          auto param_name = fmt::format("{}{}.{}", prefix_, value, "limit");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "specifies limit for x, y and z axis";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = -10.0;
              descriptor.floating_point_range.at(0).to_value = 10.0;
              auto parameter = rclcpp::ParameterValue(entry.limit);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = fixed_size<double>(param, 3);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.limit': {}", validation_result.error()));
          }
          if(auto validation_result = lower_element_bounds<double>(param, -10.0);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.limit': {}", validation_result.error()));
          }
          if(auto validation_result = upper_element_bounds<double>(param, 10.0);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter '__map_joints.__map_dof_names.limit': {}", validation_result.error()));
          }
          entry.limit = param.as_double_array();}
      }

      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {

          auto& entry = updated_params.nested_dynamic.joints_map[value_1].dof_names_map[value_2];
          std::string value = fmt::format("{}.{}", value_1, value_2);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_dynamic", value, "nested");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "test nested map params";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = 0.0001;
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.nested);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt_eq<double>(param, 0.0001);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'nested_dynamic.__map_joints.__map_dof_names.nested': {}", validation_result.error()));
          }
          entry.nested = param.as_double();}
      }

      for (const auto & value_1 : updated_params.joints) {
      for (const auto & value_2 : updated_params.dof_names) {
      for (const auto & value_3 : updated_params.joints) {
      for (const auto & value_4 : updated_params.dof_names) {

          auto& entry = updated_params.nested_dynamic.joints_map[value_1].dof_names_map[value_2].joints_map[value_3].dof_names_map[value_4];
          std::string value = fmt::format("{}.{}.{}.{}", value_1, value_2, value_3, value_4);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_dynamic", value, "nested_deep");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "test deep nested map params";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = 0.0001;
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.nested_deep);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt_eq<double>(param, 0.0001);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'nested_dynamic.__map_joints.__map_dof_names.__map_joints.__map_dof_names.nested_deep': {}", validation_result.error()));
          }
          entry.nested_deep = param.as_double();}
      }
      }
      }

      for (const auto & value_1 : updated_params.joints) {

          auto& entry = updated_params.pid.joints_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "p");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "proportional gain term";
              descriptor.read_only = false;
              descriptor.floating_point_range.resize(1);
              descriptor.floating_point_range.at(0).from_value = 0.0001;
              descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
              auto parameter = rclcpp::ParameterValue(entry.p);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          if(auto validation_result = gt_eq<double>(param, 0.0001);
            !validation_result) {
              throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'pid.__map_joints.p': {}", validation_result.error()));
          }
          entry.p = param.as_double();}

      for (const auto & value_1 : updated_params.joints) {

          auto& entry = updated_params.pid.joints_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "i");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "integral gain term";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.i);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.i = param.as_double();}

      for (const auto & value_1 : updated_params.joints) {

          auto& entry = updated_params.pid.joints_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "pid", value, "d");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "derivative gain term";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.d);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.d = param.as_double();}

      for (const auto & value_1 : updated_params.dof_names) {

          auto& entry = updated_params.gains.dof_names_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "gains", value, "k");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "general gain";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.k);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.k = param.as_double();}

      for (const auto & value_1 : updated_params.nested_map.entries) {

          auto& entry = updated_params.nested_map.entries_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_map", value, "value");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "A value in the nested map with sibling keys";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.value);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.value = param.as_double();}

      for (const auto & value_1 : updated_params.nested_map_struct_entries) {

          auto& entry = updated_params.nested_map_struct.nested_map_struct_entries_map[value_1];
          std::string value = fmt::format("{}", value_1);

          auto param_name = fmt::format("{}{}.{}.{}", prefix_, "nested_map_struct", value, "nested_struct.nested_struct_field");
          if (!parameters_interface_->has_parameter(param_name)) {
              rcl_interfaces::msg::ParameterDescriptor descriptor;
              descriptor.description = "test fields in deeply nested structs";
              descriptor.read_only = false;
              auto parameter = rclcpp::ParameterValue(entry.nested_struct.nested_struct_field);
              parameters_interface_->declare_parameter(param_name, parameter, descriptor);
          }
          param = parameters_interface_->get_parameter(param_name);
          RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
          entry.nested_struct.nested_struct_field = param.as_string();}


      updated_params.__stamp = clock_.now();
      update_internal_params(updated_params);
    }

    using userParameterUpdateCB = std::function<void(const Params&)>;
    void setUserCallback(const userParameterUpdateCB& callback){
      user_callback_ = callback;
    }

    void clearUserCallback(){
      user_callback_ = {};
    }

    private:
      void update_internal_params(Params updated_params) {
        std::lock_guard<std::mutex> lock(mutex_);
        params_ = std::move(updated_params);
      }

      std::string prefix_;
      Params params_;
      rclcpp::Clock clock_;
      std::shared_ptr<rclcpp::node_interfaces::OnSetParametersCallbackHandle> handle_;
      std::shared_ptr<rclcpp::node_interfaces::NodeParametersInterface> parameters_interface_;
      userParameterUpdateCB user_callback_;

      rclcpp::Logger logger_;
      std::mutex mutable mutex_;
  };

} // namespace admittance_controller
