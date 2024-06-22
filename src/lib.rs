#![allow(non_snake_case, dead_code)]

//TODO: Will this be compiled out?
#[link(name = "ole32")]
#[link(name = "user32")]
extern "system" {}

pub mod com;
pub mod mmdevice;
pub mod propvariant;

pub use com::*;
pub use mmdevice::*;
pub use propvariant::*;

pub use core::ffi::c_void;
pub use core::mem::{transmute, transmute_copy};

//TODO: Change the error type from i32 to something more useful.
pub trait WindowsResult {
    fn as_result<T, P>(self, pointer: *mut P) -> Result<T, i32>;
    fn as_result_owned<T>(self, owned: T) -> Result<T, i32>;
}

impl WindowsResult for i32 {
    fn as_result<T, P>(self, pointer: *mut P) -> Result<T, i32> {
        if self >= 0 {
            unsafe { Ok(transmute_copy(&(pointer as *mut T))) }
        } else {
            Err(self)
        }
    }

    fn as_result_owned<T>(self, owned: T) -> Result<T, i32> {
        if self >= 0 {
            Ok(owned)
        } else {
            Err(self)
        }
    }
}

impl WindowsResult for makepad_windows::core::HRESULT {
    fn as_result<T, P>(self, pointer: *mut P) -> Result<T, i32> {
        self.0.as_result(pointer)
    }

    fn as_result_owned<T>(self, owned: T) -> Result<T, i32> {
        self.0.as_result_owned(owned)
    }
}
