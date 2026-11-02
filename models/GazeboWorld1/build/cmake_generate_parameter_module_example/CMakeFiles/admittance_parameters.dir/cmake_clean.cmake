file(REMOVE_RECURSE
  "generate_parameter_module_temp/admittance_parameters.py"
)

# Per-language clean rules from dependency scanning.
foreach(lang )
  include(CMakeFiles/admittance_parameters.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
