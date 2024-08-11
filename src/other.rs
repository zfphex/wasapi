use std::ffi::c_void;

#[repr(C)]
pub struct SecurityAttributes {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut c_void,
    pub bInheritHandle: i32,
}

unsafe extern "system" {
    pub fn CreateEventA(
        lpEventAttributes: *mut SecurityAttributes,
        bManualReset: i32,
        bInitialState: i32,
        lpName: *const i8,
    ) -> *mut c_void;
}
