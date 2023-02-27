use core::ffi::c_void;

extern "C" {
    #[cfg(test)]
    pub fn malloc(len: usize) -> *mut u8;

    pub fn free(ptr: *mut c_void);
}
