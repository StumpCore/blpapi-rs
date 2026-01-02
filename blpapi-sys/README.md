# Builder for the bindings
Depending on the OS a specific bindings_<OsType>.rs is generated when building the blpapi-sys.rs.
During development it was identified that the compliler changes some of the type values depending on which OS is used during the building procedure.
A simple case is that some functions return c_uint on linux and c_int on windows.
This was not an error until know, since newly created rust-enums require a specific From<> value. 

# Mandatory
Set environmental Variable *BLPAPI_LIB*
