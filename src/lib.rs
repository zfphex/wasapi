#![allow(non_snake_case)]

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

/// Convert a function that takes in *mut pointer and returns `HRESULT` into `Result<T, i32>`
/// ```
/// let input = null_mut();
/// let result = example_fn(&mut input);
/// let out = if result >= 0 { Ok(input) } else { Err(result) };
/// ```
/// ```
/// let input = null_mut()
/// let out: Result<T, i32> = example_fn(&mut input).into_result(input);
/// ```
pub trait WindowsResult<T, P> {
    fn into_result(self, p: *mut P) -> Result<T, i32>;
}

impl<T, P> WindowsResult<T, P> for i32 {
    fn into_result(self, p: *mut P) -> Result<T, i32> {
        if self >= 0 {
            unsafe { Ok(std::mem::transmute_copy(&(p as *mut T))) }
        } else {
            Err(self)
        }
    }
}

impl<T, P> WindowsResult<T, P> for makepad_windows::core::HRESULT {
    fn into_result(self, p: *mut P) -> Result<T, i32> {
        if self.0 >= 0 {
            unsafe { Ok(std::mem::transmute_copy(&(p as *mut T))) }
        } else {
            Err(self.0)
        }
    }
}
