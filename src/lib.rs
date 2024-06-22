#![allow(non_snake_case)]

//TODO: Will this be compiled out?
#[link(name = "ole32")]
#[link(name = "user32")]
extern "system" {}

pub mod mkpad;
pub mod winapi;

//TODO: Change the error type from i32 to something more useful.
pub trait WindowsResultEmpty {
    fn ok(self) -> Result<(), i32>;
}

impl WindowsResultEmpty for i32 {
    fn ok(self) -> Result<(), i32> {
        if self >= 0 {
            Ok(())
        } else {
            Err(self)
        }
    }
}

pub trait WindowsResult {
    fn as_result<T, P>(self, pointer: *mut P) -> Result<T, i32>;
    fn as_result_owned<T>(self, owned: T) -> Result<T, i32>;
}

impl WindowsResult for i32 {
    fn as_result<T, P>(self, pointer: *mut P) -> Result<T, i32> {
        if self >= 0 {
            unsafe { Ok(std::mem::transmute_copy(&(pointer as *mut T))) }
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
