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

#include "custom_validators.hpp"


namespace mecanum_drive_controller {

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
        double reference_timeout = 0.0;
        std::string front_left_wheel_command_joint_name = "";
        std::string front_right_wheel_command_joint_name = "";
        std::string rear_right_wheel_command_joint_name = "";
        std::string rear_left_wheel_command_joint_name = "";
        std::string front_left_wheel_state_joint_name = "";
        std::string front_right_wheel_state_joint_name = "";
        std::string rear_right_wheel_state_joint_name = "";
        std::string rear_left_wheel_state_joint_name = "";
        bool tf_frame_prefix_enable = true;
        std::string tf_frame_prefix = "";
        std::string base_frame_id = "base_link";
        std::string odom_frame_id = "odom";
        bool enable_odom_tf = true;
        std::vector<double> twist_covariance_diagonal = {0.1, 0.1, 0.1, 0.1, 0.1, 0.1};
        std::vector<double> pose_covariance_diagonal = {0.1, 0.1, 0.1, 0.1, 0.1, 0.1};
        struct Kinematics {
            double wheels_radius = 0.0;
            double sum_of_robot_center_projection_on_X_Y_axis = 0.0;
            struct BaseFrameOffset {
                double x = 0.0;
                double y = 0.0;
                double theta = 0.0;
            } base_frame_offset;
        } kinematics;
        struct Linear {
            struct X {
                double max_velocity = std::numeric_limits<double>::quiet_NaN();
                double min_velocity = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_jerk = std::numeric_limits<double>::quiet_NaN();
                double min_jerk = std::numeric_limits<double>::quiet_NaN();
            } x;
            struct Y {
                double max_velocity = std::numeric_limits<double>::quiet_NaN();
                double min_velocity = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_jerk = std::numeric_limits<double>::quiet_NaN();
                double min_jerk = std::numeric_limits<double>::quiet_NaN();
            } y;
        } linear;
        struct Angular {
            struct Z {
                double max_velocity = std::numeric_limits<double>::quiet_NaN();
                double min_velocity = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_jerk = std::numeric_limits<double>::quiet_NaN();
                double min_jerk = std::numeric_limits<double>::quiet_NaN();
            } z;
        } angular;
        // for detecting if the parameter struct has been updated
        rclcpp::Time __stamp;
    };
    struct StackParams {
        double reference_timeout = 0.0;
        bool tf_frame_prefix_enable = true;
        bool enable_odom_tf = true;
        struct Kinematics {
            double wheels_radius = 0.0;
            double sum_of_robot_center_projection_on_X_Y_axis = 0.0;
            struct BaseFrameOffset {
                double x = 0.0;
                double y = 0.0;
                double theta = 0.0;
            } base_frame_offset;
        } kinematics;
        struct Linear {
            struct X {
                double max_velocity = std::numeric_limits<double>::quiet_NaN();
                double min_velocity = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_jerk = std::numeric_limits<double>::quiet_NaN();
                double min_jerk = std::numeric_limits<double>::quiet_NaN();
            } x;
            struct Y {
                double max_velocity = std::numeric_limits<double>::quiet_NaN();
                double min_velocity = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_jerk = std::numeric_limits<double>::quiet_NaN();
                double min_jerk = std::numeric_limits<double>::quiet_NaN();
            } y;
        } linear;
        struct Angular {
            struct Z {
                double max_velocity = std::numeric_limits<double>::quiet_NaN();
                double min_velocity = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration = std::numeric_limits<double>::quiet_NaN();
                double max_acceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_deceleration_reverse = std::numeric_limits<double>::quiet_NaN();
                double max_jerk = std::numeric_limits<double>::quiet_NaN();
                double min_jerk = std::numeric_limits<double>::quiet_NaN();
            } z;
        } angular;
    };

  class ParamListener{
  public:
    // throws rclcpp::exceptions::InvalidParameterValueException on initialization if invalid parameter are loaded
    template <typename NodeT>
    ParamListener(NodeT node, std::string const& prefix = "")
    : ParamListener(node->get_node_parameters_interface(), node->get_logger(), prefix) {}

    ParamListener(const std::shared_ptr<rclcpp::node_interfaces::NodeParametersInterface>& parameters_interface,
                  std::string const& prefix = "")
    : ParamListener(parameters_interface, rclcpp::get_logger("mecanum_drive_controller"), prefix) {
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
      output.reference_timeout = params.reference_timeout;
      output.kinematics.base_frame_offset.x = params.kinematics.base_frame_offset.x;
      output.kinematics.base_frame_offset.y = params.kinematics.base_frame_offset.y;
      output.kinematics.base_frame_offset.theta = params.kinematics.base_frame_offset.theta;
      output.kinematics.wheels_radius = params.kinematics.wheels_radius;
      output.kinematics.sum_of_robot_center_projection_on_X_Y_axis = params.kinematics.sum_of_robot_center_projection_on_X_Y_axis;
      output.tf_frame_prefix_enable = params.tf_frame_prefix_enable;
      output.enable_odom_tf = params.enable_odom_tf;
      output.linear.x.max_velocity = params.linear.x.max_velocity;
      output.linear.x.min_velocity = params.linear.x.min_velocity;
      output.linear.x.max_acceleration = params.linear.x.max_acceleration;
      output.linear.x.max_deceleration = params.linear.x.max_deceleration;
      output.linear.x.max_acceleration_reverse = params.linear.x.max_acceleration_reverse;
      output.linear.x.max_deceleration_reverse = params.linear.x.max_deceleration_reverse;
      output.linear.x.max_jerk = params.linear.x.max_jerk;
      output.linear.x.min_jerk = params.linear.x.min_jerk;
      output.linear.y.max_velocity = params.linear.y.max_velocity;
      output.linear.y.min_velocity = params.linear.y.min_velocity;
      output.linear.y.max_acceleration = params.linear.y.max_acceleration;
      output.linear.y.max_deceleration = params.linear.y.max_deceleration;
      output.linear.y.max_acceleration_reverse = params.linear.y.max_acceleration_reverse;
      output.linear.y.max_deceleration_reverse = params.linear.y.max_deceleration_reverse;
      output.linear.y.max_jerk = params.linear.y.max_jerk;
      output.linear.y.min_jerk = params.linear.y.min_jerk;
      output.angular.z.max_velocity = params.angular.z.max_velocity;
      output.angular.z.min_velocity = params.angular.z.min_velocity;
      output.angular.z.max_acceleration = params.angular.z.max_acceleration;
      output.angular.z.max_deceleration = params.angular.z.max_deceleration;
      output.angular.z.max_acceleration_reverse = params.angular.z.max_acceleration_reverse;
      output.angular.z.max_deceleration_reverse = params.angular.z.max_deceleration_reverse;
      output.angular.z.max_jerk = params.angular.z.max_jerk;
      output.angular.z.min_jerk = params.angular.z.min_jerk;

      return output;
    }

    void refresh_dynamic_parameters() {
      auto updated_params = get_params();
      // TODO remove any destroyed dynamic parameters

      // declare any new dynamic parameters
      rclcpp::Parameter param;

    }

    rcl_interfaces::msg::SetParametersResult update(const std::vector<rclcpp::Parameter> &parameters) {
      auto updated_params = get_params();
      bool any_param_updated = false;

      for (const auto &param: parameters) {
        if (param.get_name() == (prefix_ + "reference_timeout")) {
            updated_params.reference_timeout = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "front_left_wheel_command_joint_name")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.front_left_wheel_command_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "front_right_wheel_command_joint_name")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.front_right_wheel_command_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "rear_right_wheel_command_joint_name")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.rear_right_wheel_command_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "rear_left_wheel_command_joint_name")) {
            if(auto validation_result = not_empty<std::string>(param);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.rear_left_wheel_command_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "front_left_wheel_state_joint_name")) {
            updated_params.front_left_wheel_state_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "front_right_wheel_state_joint_name")) {
            updated_params.front_right_wheel_state_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "rear_right_wheel_state_joint_name")) {
            updated_params.rear_right_wheel_state_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "rear_left_wheel_state_joint_name")) {
            updated_params.rear_left_wheel_state_joint_name = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.base_frame_offset.x")) {
            updated_params.kinematics.base_frame_offset.x = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.base_frame_offset.y")) {
            updated_params.kinematics.base_frame_offset.y = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.base_frame_offset.theta")) {
            updated_params.kinematics.base_frame_offset.theta = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.wheels_radius")) {
            if(auto validation_result = gt<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.kinematics.wheels_radius = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "kinematics.sum_of_robot_center_projection_on_X_Y_axis")) {
            updated_params.kinematics.sum_of_robot_center_projection_on_X_Y_axis = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "tf_frame_prefix_enable")) {
            updated_params.tf_frame_prefix_enable = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "tf_frame_prefix")) {
            updated_params.tf_frame_prefix = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "base_frame_id")) {
            updated_params.base_frame_id = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "odom_frame_id")) {
            updated_params.odom_frame_id = param.as_string();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "enable_odom_tf")) {
            updated_params.enable_odom_tf = param.as_bool();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "twist_covariance_diagonal")) {
            updated_params.twist_covariance_diagonal = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "pose_covariance_diagonal")) {
            updated_params.pose_covariance_diagonal = param.as_double_array();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.max_velocity")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.max_velocity = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.min_velocity")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.min_velocity = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.max_acceleration")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.max_acceleration = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.max_deceleration")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.max_deceleration = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.max_acceleration_reverse")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.max_acceleration_reverse = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.max_deceleration_reverse")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.max_deceleration_reverse = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.max_jerk")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.max_jerk = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.x.min_jerk")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.x.min_jerk = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.max_velocity")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.max_velocity = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.min_velocity")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.min_velocity = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.max_acceleration")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.max_acceleration = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.max_deceleration")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.max_deceleration = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.max_acceleration_reverse")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.max_acceleration_reverse = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.max_deceleration_reverse")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.max_deceleration_reverse = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.max_jerk")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.max_jerk = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "linear.y.min_jerk")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.linear.y.min_jerk = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.max_velocity")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.max_velocity = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.min_velocity")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.min_velocity = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.max_acceleration")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.max_acceleration = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.max_deceleration")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.max_deceleration = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.max_acceleration_reverse")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.max_acceleration_reverse = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.max_deceleration_reverse")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.max_deceleration_reverse = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.max_jerk")) {
            if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.max_jerk = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
        }
        if (param.get_name() == (prefix_ + "angular.z.min_jerk")) {
            if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
              !validation_result) {
                return rsl::to_parameter_result_msg(validation_result);
            }
            updated_params.angular.z.min_jerk = param.as_double();
            any_param_updated = true;
            RCLCPP_DEBUG_STREAM(logger_, param.get_name() << ": " << param.get_type_name() << " = " << param.value_to_string());
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
      if (!parameters_interface_->has_parameter(prefix_ + "reference_timeout")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Timeout for controller references after which they will be reset. This is especially useful for controllers that can cause unwanted and dangerous behavior if reference is not reset, e.g., velocity controllers. If value is 0 the reference is reset after each run.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.reference_timeout);
          parameters_interface_->declare_parameter(prefix_ + "reference_timeout", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "front_left_wheel_command_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Name of the joints for commanding front left wheel.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.front_left_wheel_command_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "front_left_wheel_command_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "front_right_wheel_command_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Name of the joints for commanding front right wheel.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.front_right_wheel_command_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "front_right_wheel_command_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "rear_right_wheel_command_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Name of the joints for commanding rear right wheel.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.rear_right_wheel_command_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "rear_right_wheel_command_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "rear_left_wheel_command_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Name of the joints for commanding rear left wheel.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.rear_left_wheel_command_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "rear_left_wheel_command_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "front_left_wheel_state_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "(optional) Specifies a joint name for reading states of front left wheel. This parameter is only relevant when state joints are different then command joint, i.e., when a following controller is used.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.front_left_wheel_state_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "front_left_wheel_state_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "front_right_wheel_state_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "(optional) Specifies a joint name for reading states of front right wheel. This parameter is only relevant when state joints are different then command joint, i.e., when a following controller is used.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.front_right_wheel_state_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "front_right_wheel_state_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "rear_right_wheel_state_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "(optional) Specifies a joint name for reading states of rear right wheel. This parameter is only relevant when state joints are different then command joint, i.e., when a following controller is used.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.rear_right_wheel_state_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "rear_right_wheel_state_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "rear_left_wheel_state_joint_name")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "(optional) Specifies a joint name for reading states of rear left wheel. This parameter is only relevant when state joints are different then command joint, i.e., when a following controller is used.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.rear_left_wheel_state_joint_name);
          parameters_interface_->declare_parameter(prefix_ + "rear_left_wheel_state_joint_name", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.base_frame_offset.x")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Base frame offset along X axis of base_frame (base_link frame).";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.kinematics.base_frame_offset.x);
          parameters_interface_->declare_parameter(prefix_ + "kinematics.base_frame_offset.x", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.base_frame_offset.y")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Base frame offset along Y axis of base_frame (base_link frame).";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.kinematics.base_frame_offset.y);
          parameters_interface_->declare_parameter(prefix_ + "kinematics.base_frame_offset.y", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.base_frame_offset.theta")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Base frame offset along Theta axis of base_frame (base_link frame).";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.kinematics.base_frame_offset.theta);
          parameters_interface_->declare_parameter(prefix_ + "kinematics.base_frame_offset.theta", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.wheels_radius")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Wheel's radius.";
          descriptor.read_only = true;
          descriptor.floating_point_range.resize(1);
          descriptor.floating_point_range.at(0).from_value = std::nextafter(0.0, std::numeric_limits<double>::infinity());
          descriptor.floating_point_range.at(0).to_value = std::numeric_limits<double>::max();
          auto parameter = to_parameter_value(updated_params.kinematics.wheels_radius);
          parameters_interface_->declare_parameter(prefix_ + "kinematics.wheels_radius", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "kinematics.sum_of_robot_center_projection_on_X_Y_axis")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Wheels geometric param used in mecanum wheels' IK. lx and ly represent the distance from the robot's center to the wheels projected on the x and y axis with origin at robots center respectively, sum_of_robot_center_projection_on_X_Y_axis = lx+ly";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.kinematics.sum_of_robot_center_projection_on_X_Y_axis);
          parameters_interface_->declare_parameter(prefix_ + "kinematics.sum_of_robot_center_projection_on_X_Y_axis", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "tf_frame_prefix_enable")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Enables or disables appending tf_frame_prefix to tf frame id's. See its parameter description for more information.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.tf_frame_prefix_enable);
          parameters_interface_->declare_parameter(prefix_ + "tf_frame_prefix_enable", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "tf_frame_prefix")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "(optional) Prefix to be appended to the tf frames, will be added to odom_frame_id and base_frame_id before publishing. If the parameter is empty, controller's namespace will be used.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.tf_frame_prefix);
          parameters_interface_->declare_parameter(prefix_ + "tf_frame_prefix", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "base_frame_id")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Base frame_id set to value of base_frame_id.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.base_frame_id);
          parameters_interface_->declare_parameter(prefix_ + "base_frame_id", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "odom_frame_id")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Odometry frame_id set to value of odom_frame_id.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.odom_frame_id);
          parameters_interface_->declare_parameter(prefix_ + "odom_frame_id", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "enable_odom_tf")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Publishing to tf is enabled or disabled?";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.enable_odom_tf);
          parameters_interface_->declare_parameter(prefix_ + "enable_odom_tf", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "twist_covariance_diagonal")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Diagonal values of twist covariance matrix.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.twist_covariance_diagonal);
          parameters_interface_->declare_parameter(prefix_ + "twist_covariance_diagonal", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "pose_covariance_diagonal")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Diagonal values of pose covariance matrix.";
          descriptor.read_only = true;
          auto parameter = to_parameter_value(updated_params.pose_covariance_diagonal);
          parameters_interface_->declare_parameter(prefix_ + "pose_covariance_diagonal", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.max_velocity")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.max_velocity);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.max_velocity", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.min_velocity")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.min_velocity);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.min_velocity", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.max_acceleration")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum acceleration in forward direction.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.max_acceleration);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.max_acceleration", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.max_deceleration")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum deceleration in forward direction.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.max_deceleration);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.max_deceleration", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.max_acceleration_reverse")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum acceleration in reverse direction. If not set, -max_acceleration will be used.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.max_acceleration_reverse);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.max_acceleration_reverse", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.max_deceleration_reverse")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum deceleration in reverse direction. If not set, -max_deceleration will be used.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.max_deceleration_reverse);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.max_deceleration_reverse", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.max_jerk")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.max_jerk);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.max_jerk", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.x.min_jerk")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.x.min_jerk);
          parameters_interface_->declare_parameter(prefix_ + "linear.x.min_jerk", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.max_velocity")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.max_velocity);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.max_velocity", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.min_velocity")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.min_velocity);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.min_velocity", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.max_acceleration")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum acceleration in forward direction.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.max_acceleration);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.max_acceleration", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.max_deceleration")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum deceleration in forward direction.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.max_deceleration);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.max_deceleration", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.max_acceleration_reverse")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum acceleration in reverse direction. If not set, -max_acceleration will be used.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.max_acceleration_reverse);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.max_acceleration_reverse", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.max_deceleration_reverse")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum deceleration in reverse direction. If not set, -max_deceleration will be used.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.max_deceleration_reverse);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.max_deceleration_reverse", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.max_jerk")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.max_jerk);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.max_jerk", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "linear.y.min_jerk")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.linear.y.min_jerk);
          parameters_interface_->declare_parameter(prefix_ + "linear.y.min_jerk", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.max_velocity")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.max_velocity);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.max_velocity", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.min_velocity")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.min_velocity);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.min_velocity", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.max_acceleration")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum acceleration in positive direction.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.max_acceleration);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.max_acceleration", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.max_deceleration")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum deceleration in positive direction.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.max_deceleration);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.max_deceleration", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.max_acceleration_reverse")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum acceleration in negative direction. If not set, -max_acceleration will be used.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.max_acceleration_reverse);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.max_acceleration_reverse", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.max_deceleration_reverse")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "Maximum deceleration in negative direction. If not set, -max_deceleration will be used.";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.max_deceleration_reverse);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.max_deceleration_reverse", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.max_jerk")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.max_jerk);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.max_jerk", parameter, descriptor);
      }
      if (!parameters_interface_->has_parameter(prefix_ + "angular.z.min_jerk")) {
          rcl_interfaces::msg::ParameterDescriptor descriptor;
          descriptor.description = "";
          descriptor.read_only = false;
          auto parameter = to_parameter_value(updated_params.angular.z.min_jerk);
          parameters_interface_->declare_parameter(prefix_ + "angular.z.min_jerk", parameter, descriptor);
      }
      // get parameters and fill struct fields
      rclcpp::Parameter param;
      param = parameters_interface_->get_parameter(prefix_ + "reference_timeout");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "reference_timeout") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.reference_timeout = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "front_left_wheel_command_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "front_left_wheel_command_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'front_left_wheel_command_joint_name': {}", validation_result.error()));
      }
      updated_params.front_left_wheel_command_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "front_right_wheel_command_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "front_right_wheel_command_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'front_right_wheel_command_joint_name': {}", validation_result.error()));
      }
      updated_params.front_right_wheel_command_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "rear_right_wheel_command_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "rear_right_wheel_command_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'rear_right_wheel_command_joint_name': {}", validation_result.error()));
      }
      updated_params.rear_right_wheel_command_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "rear_left_wheel_command_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "rear_left_wheel_command_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = not_empty<std::string>(param);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'rear_left_wheel_command_joint_name': {}", validation_result.error()));
      }
      updated_params.rear_left_wheel_command_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "front_left_wheel_state_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "front_left_wheel_state_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.front_left_wheel_state_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "front_right_wheel_state_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "front_right_wheel_state_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.front_right_wheel_state_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "rear_right_wheel_state_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "rear_right_wheel_state_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.rear_right_wheel_state_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "rear_left_wheel_state_joint_name");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "rear_left_wheel_state_joint_name") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.rear_left_wheel_state_joint_name = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.base_frame_offset.x");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.base_frame_offset.x") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.base_frame_offset.x = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.base_frame_offset.y");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.base_frame_offset.y") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.base_frame_offset.y = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.base_frame_offset.theta");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.base_frame_offset.theta") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.base_frame_offset.theta = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.wheels_radius");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.wheels_radius") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = gt<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'kinematics.wheels_radius': {}", validation_result.error()));
      }
      updated_params.kinematics.wheels_radius = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "kinematics.sum_of_robot_center_projection_on_X_Y_axis");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "kinematics.sum_of_robot_center_projection_on_X_Y_axis") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.kinematics.sum_of_robot_center_projection_on_X_Y_axis = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "tf_frame_prefix_enable");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "tf_frame_prefix_enable") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.tf_frame_prefix_enable = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "tf_frame_prefix");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "tf_frame_prefix") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.tf_frame_prefix = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "base_frame_id");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "base_frame_id") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.base_frame_id = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "odom_frame_id");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "odom_frame_id") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.odom_frame_id = param.as_string();
      param = parameters_interface_->get_parameter(prefix_ + "enable_odom_tf");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "enable_odom_tf") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.enable_odom_tf = param.as_bool();
      param = parameters_interface_->get_parameter(prefix_ + "twist_covariance_diagonal");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "twist_covariance_diagonal") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.twist_covariance_diagonal = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "pose_covariance_diagonal");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "pose_covariance_diagonal") << ": " << param.get_type_name() << " = " << param.value_to_string());
      updated_params.pose_covariance_diagonal = param.as_double_array();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.max_velocity");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.max_velocity") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.max_velocity': {}", validation_result.error()));
      }
      updated_params.linear.x.max_velocity = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.min_velocity");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.min_velocity") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.min_velocity': {}", validation_result.error()));
      }
      updated_params.linear.x.min_velocity = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.max_acceleration");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.max_acceleration") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.max_acceleration': {}", validation_result.error()));
      }
      updated_params.linear.x.max_acceleration = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.max_deceleration");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.max_deceleration") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.max_deceleration': {}", validation_result.error()));
      }
      updated_params.linear.x.max_deceleration = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.max_acceleration_reverse");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.max_acceleration_reverse") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.max_acceleration_reverse': {}", validation_result.error()));
      }
      updated_params.linear.x.max_acceleration_reverse = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.max_deceleration_reverse");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.max_deceleration_reverse") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.max_deceleration_reverse': {}", validation_result.error()));
      }
      updated_params.linear.x.max_deceleration_reverse = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.max_jerk");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.max_jerk") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.max_jerk': {}", validation_result.error()));
      }
      updated_params.linear.x.max_jerk = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.x.min_jerk");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.x.min_jerk") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.x.min_jerk': {}", validation_result.error()));
      }
      updated_params.linear.x.min_jerk = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.max_velocity");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.max_velocity") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.max_velocity': {}", validation_result.error()));
      }
      updated_params.linear.y.max_velocity = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.min_velocity");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.min_velocity") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.min_velocity': {}", validation_result.error()));
      }
      updated_params.linear.y.min_velocity = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.max_acceleration");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.max_acceleration") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.max_acceleration': {}", validation_result.error()));
      }
      updated_params.linear.y.max_acceleration = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.max_deceleration");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.max_deceleration") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.max_deceleration': {}", validation_result.error()));
      }
      updated_params.linear.y.max_deceleration = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.max_acceleration_reverse");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.max_acceleration_reverse") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.max_acceleration_reverse': {}", validation_result.error()));
      }
      updated_params.linear.y.max_acceleration_reverse = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.max_deceleration_reverse");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.max_deceleration_reverse") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.max_deceleration_reverse': {}", validation_result.error()));
      }
      updated_params.linear.y.max_deceleration_reverse = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.max_jerk");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.max_jerk") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.max_jerk': {}", validation_result.error()));
      }
      updated_params.linear.y.max_jerk = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "linear.y.min_jerk");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "linear.y.min_jerk") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'linear.y.min_jerk': {}", validation_result.error()));
      }
      updated_params.linear.y.min_jerk = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.max_velocity");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.max_velocity") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.max_velocity': {}", validation_result.error()));
      }
      updated_params.angular.z.max_velocity = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.min_velocity");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.min_velocity") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.min_velocity': {}", validation_result.error()));
      }
      updated_params.angular.z.min_velocity = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.max_acceleration");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.max_acceleration") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.max_acceleration': {}", validation_result.error()));
      }
      updated_params.angular.z.max_acceleration = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.max_deceleration");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.max_deceleration") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.max_deceleration': {}", validation_result.error()));
      }
      updated_params.angular.z.max_deceleration = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.max_acceleration_reverse");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.max_acceleration_reverse") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.max_acceleration_reverse': {}", validation_result.error()));
      }
      updated_params.angular.z.max_acceleration_reverse = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.max_deceleration_reverse");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.max_deceleration_reverse") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.max_deceleration_reverse': {}", validation_result.error()));
      }
      updated_params.angular.z.max_deceleration_reverse = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.max_jerk");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.max_jerk") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::gt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.max_jerk': {}", validation_result.error()));
      }
      updated_params.angular.z.max_jerk = param.as_double();
      param = parameters_interface_->get_parameter(prefix_ + "angular.z.min_jerk");
      RCLCPP_DEBUG_STREAM(logger_, (prefix_ + "angular.z.min_jerk") << ": " << param.get_type_name() << " = " << param.value_to_string());
      if(auto validation_result = control_filters::lt_eq_or_nan<double>(param, 0.0);
        !validation_result) {
          throw rclcpp::exceptions::InvalidParameterValueException(fmt::format("Invalid value set during initialization for parameter 'angular.z.min_jerk': {}", validation_result.error()));
      }
      updated_params.angular.z.min_jerk = param.as_double();


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

} // namespace mecanum_drive_controller
