#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn cxxrtl_get(
    handle: cxxrtl_handle,
    name: *const ::std::os::raw::c_char,
) -> *mut cxxrtl_object {
    let mut parts = 0;
    let object = cxxrtl_get_parts(handle, name, &mut parts);
    assert!(object.is_null() || parts == 1);
    if object.is_null() || parts == 1 {
        object
    } else {
        std::ptr::null_mut()
    }
}
