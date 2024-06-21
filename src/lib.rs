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
    // fn into_result(self, p: P) -> Result<T, i32>;
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

//TODO: What in gods name is this nightmare?

// mod winresult {
//     #[repr(transparent)]
//     #[derive(Copy, Clone, Default, Eq, PartialEq)]
//     #[must_use]
//     #[allow(non_camel_case_types)]
//     pub struct HRESULT(pub i32);

//     impl HRESULT {
//         pub unsafe fn from_abi<T: Type<T>>(self, abi: T::Abi) -> Result<T> {
//             if self.is_ok() {
//                 T::from_abi(abi)
//             } else {
//                 Err(Error::from(self))
//             }
//         }
//     }

//     #[doc(hidden)]
//     pub trait TypeKind {
//         type TypeKind;
//     }

//     #[doc(hidden)]
//     pub struct ReferenceType;

//     #[doc(hidden)]
//     pub struct ValueType;

//     #[doc(hidden)]
//     pub struct CopyType;

//     #[doc(hidden)]
//     pub trait Type<T: TypeKind, C = <T as TypeKind>::TypeKind>: TypeKind + Sized {
//         type Abi;
//         type Default;

//         fn abi(&self) -> Self::Abi {
//             unsafe { std::mem::transmute_copy(self) }
//         }

//         /// # Safety
//         unsafe fn from_abi(abi: Self::Abi) -> Result<Self>;
//         fn from_default(default: &Self::Default) -> Result<Self>;
//     }

//     impl<T> Type<T, ReferenceType> for T
//     where
//         T: TypeKind<TypeKind = ReferenceType> + Clone,
//     {
//         type Abi = *mut std::ffi::c_void;
//         type Default = Option<Self>;

//         unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
//             if !abi.is_null() {
//                 Ok(std::mem::transmute_copy(&abi))
//             } else {
//                 Err(())
//                 // Err(Error::OK)
//             }
//         }

//         fn from_default(default: &Self::Default) -> Result<Self> {
//             // default.as_ref().cloned().ok_or(Error::OK)
//             default.as_ref().cloned().ok_or(())
//         }
//     }

//     impl<T> Type<T, ValueType> for T
//     where
//         T: TypeKind<TypeKind = ValueType> + Clone,
//     {
//         type Abi = std::mem::MaybeUninit<Self>;
//         type Default = Self;

//         unsafe fn from_abi(abi: std::mem::MaybeUninit<Self>) -> Result<Self> {
//             Ok(abi.assume_init())
//         }

//         fn from_default(default: &Self::Default) -> Result<Self> {
//             Ok(default.clone())
//         }
//     }

//     impl<T> Type<T, CopyType> for T
//     where
//         T: TypeKind<TypeKind = CopyType> + Clone,
//     {
//         type Abi = Self;
//         type Default = Self;

//         unsafe fn from_abi(abi: Self) -> Result<Self> {
//             Ok(abi)
//         }

//         fn from_default(default: &Self) -> Result<Self> {
//             Ok(default.clone())
//         }
//     }

//     impl<T> TypeKind for *mut T {
//         type TypeKind = CopyType;
//     }

//     macro_rules! primitives {
//     ($($t:ty),+) => {
//         $(
//             impl TypeKind for $t {
//                 type TypeKind = CopyType;
//             }
//         )*
//     };
// }

//     primitives!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

//     #[doc(hidden)]
//     pub type AbiType<T> = <T as Type<T>>::Abi;

//     #[doc(hidden)]
//     pub unsafe fn from_abi<T: Type<T>>(abi: T::Abi) -> Result<T> {
//         T::from_abi(abi)
//     }
// }
