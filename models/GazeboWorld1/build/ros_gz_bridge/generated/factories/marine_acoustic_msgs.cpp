// generated from ros_gz_bridge/resource/pkg_factories.cpp.em


#include "factories/marine_acoustic_msgs.hpp"

#include <memory>
#include <string>

#include "factory.hpp"
#include "ros_gz_bridge/convert/marine_acoustic_msgs.hpp"

namespace ros_gz_bridge
{

std::shared_ptr<FactoryInterface>
get_factory__marine_acoustic_msgs(
  const std::string & ros_type_name,
  const std::string & gz_type_name)
{
  if ((ros_type_name == "marine_acoustic_msgs/msg/Dvl" || ros_type_name.empty()) &&
      (gz_type_name == "gz.msgs.DVLVelocityTracking" || gz_type_name == "ignition.msgs.DVLVelocityTracking"))
  {
    return std::make_shared<
      Factory<
        marine_acoustic_msgs::msg::Dvl,
        gz::msgs::DVLVelocityTracking
      >
    >("marine_acoustic_msgs/msg/Dvl", "gz.msgs.DVLVelocityTracking");
  }
  return nullptr;
}

template<>
void
Factory<
  marine_acoustic_msgs::msg::Dvl,
  gz::msgs::DVLVelocityTracking
>::convert_ros_to_gz(
  const marine_acoustic_msgs::msg::Dvl & ros_msg,
  gz::msgs::DVLVelocityTracking & gz_msg)
{
  ros_gz_bridge::convert_ros_to_gz(ros_msg, gz_msg);
}

template<>
void
Factory<
  marine_acoustic_msgs::msg::Dvl,
  gz::msgs::DVLVelocityTracking
>::convert_gz_to_ros(
  const gz::msgs::DVLVelocityTracking & gz_msg,
  marine_acoustic_msgs::msg::Dvl & ros_msg)
{
  ros_gz_bridge::convert_gz_to_ros(gz_msg, ros_msg);
}

}  // namespace ros_gz_bridge
