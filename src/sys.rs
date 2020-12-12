use core::ffi::c_void;

extern "C" {
    pub fn free(ptr: *mut c_void);
}
