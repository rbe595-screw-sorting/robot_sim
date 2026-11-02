import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/abigaildiehl/screw_sorting/models/GazeboWorld1/install/generate_parameter_library_py'
