#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "generate_parameter_library_example_external::minimal_publisher_external" for configuration "Release"
set_property(TARGET generate_parameter_library_example_external::minimal_publisher_external APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(generate_parameter_library_example_external::minimal_publisher_external PROPERTIES
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libminimal_publisher_external.so"
  IMPORTED_SONAME_RELEASE "libminimal_publisher_external.so"
  )

list(APPEND _cmake_import_check_targets generate_parameter_library_example_external::minimal_publisher_external )
list(APPEND _cmake_import_check_files_for_generate_parameter_library_example_external::minimal_publisher_external "${_IMPORT_PREFIX}/lib/libminimal_publisher_external.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
