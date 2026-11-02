#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "force_torque_sensor_broadcaster::dummy_filter" for configuration "Release"
set_property(TARGET force_torque_sensor_broadcaster::dummy_filter APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(force_torque_sensor_broadcaster::dummy_filter PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libdummy_filter.so"
  IMPORTED_SONAME_RELEASE "libdummy_filter.so"
  )

list(APPEND _cmake_import_check_targets force_torque_sensor_broadcaster::dummy_filter )
list(APPEND _cmake_import_check_files_for_force_torque_sensor_broadcaster::dummy_filter "${_IMPORT_PREFIX}/lib/libdummy_filter.so" )

# Import target "force_torque_sensor_broadcaster::force_torque_sensor_broadcaster" for configuration "Release"
set_property(TARGET force_torque_sensor_broadcaster::force_torque_sensor_broadcaster APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(force_torque_sensor_broadcaster::force_torque_sensor_broadcaster PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libforce_torque_sensor_broadcaster.so"
  IMPORTED_SONAME_RELEASE "libforce_torque_sensor_broadcaster.so"
  )

list(APPEND _cmake_import_check_targets force_torque_sensor_broadcaster::force_torque_sensor_broadcaster )
list(APPEND _cmake_import_check_files_for_force_torque_sensor_broadcaster::force_torque_sensor_broadcaster "${_IMPORT_PREFIX}/lib/libforce_torque_sensor_broadcaster.so" )

# Import target "force_torque_sensor_broadcaster::wrench_transformer" for configuration "Release"
set_property(TARGET force_torque_sensor_broadcaster::wrench_transformer APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(force_torque_sensor_broadcaster::wrench_transformer PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libwrench_transformer.so"
  IMPORTED_SONAME_RELEASE "libwrench_transformer.so"
  )

list(APPEND _cmake_import_check_targets force_torque_sensor_broadcaster::wrench_transformer )
list(APPEND _cmake_import_check_files_for_force_torque_sensor_broadcaster::wrench_transformer "${_IMPORT_PREFIX}/lib/libwrench_transformer.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
