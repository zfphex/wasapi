#![allow(non_snake_case)]
use makepad_windows::core::ComInterface;
use makepad_windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use makepad_windows::Win32::Media::Audio::{eConsole, eRender, IMMDevice, DEVICE_STATE_ACTIVE};
use makepad_windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator};
use makepad_windows::Win32::System::Com::STGM_READ;
use makepad_windows::Win32::System::Variant::VT_LPWSTR;

#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
#[must_use]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub i32);

//TODO: Change the error type from i32 to something more useful.
pub trait WinResult {
    fn into_result(self) -> Result<(), i32>;
}

impl WinResult for i32 {
    fn into_result(self) -> Result<(), i32> {
        if self >= 0 {
            Ok(())
        } else {
            Err(self)
        }
    }
}

/// Convert a function that returns `i32(HRESULT)` into `Result<T, i32>`
/// `let out = null_mut(); let result = fn(&mut out);`
///  into
/// `Result<out, result>`
pub const fn check<T>(result: i32, t: *mut T) -> Result<T, i32> {
    if result >= 0 {
        unsafe { Ok(std::mem::transmute_copy(&t)) }
    } else {
        Err(result)
    }
}

pub mod com {
    #![allow(unused)]
    use core::{ffi::c_void, mem::transmute, ptr::null_mut};
    use makepad_windows::core::GUID;

    use crate::mkpad::{check, WinResult};

    #[link(name = "ole32")]
    extern "system" {}

    ///https://learn.microsoft.com/en-us/windows/win32/api/objbase/ne-objbase-coinit
    #[repr(u32)]
    pub enum ConcurrencyModel {
        ///Initializes the thread for apartment-threaded object concurrency (see Remarks).
        ApartmentThreaded = 0x2,
        ///Initializes the thread for multithreaded object concurrency (see Remarks).
        MultiThreaded = 0x0,
        ///Disables DDE for OLE1 support.
        DisableOLE1DDE = 0x4,
        ///Increase memory usage in an attempt to increase performance.
        SpeedOverMemory = 0x8,
    }

    ///https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx
    #[repr(u32)]
    pub enum ExecutionContext {
        InprocServer = 0x1,
        InprocHandler = 0x2,
        LocalServer = 0x4,
        InprocServer16 = 0x8,
        RemoteServer = 0x10,
        All = 0x17,
        InprocHandler16 = 0x20,
        Reserved1 = 0x40,
        Reserved2 = 0x80,
        Reserved3 = 0x100,
        Reserved4 = 0x200,
        NoCodeDownload = 0x400,
        Reserved5 = 0x800,
        NoCustomMarshal = 0x1000,
        EnableCodeDownload = 0x2000,
        NoFailureLog = 0x4000,
        DisableAAA = 0x8000,
        EnableAAA = 0x10000,
        FromDefaultContext = 0x20000,
        Activatex86Server = 0x40000,
        Activate32bitServer,
        Activate64bitServer = 0x80000,
        EnableCloaking = 0x100000,
        Appcontainer = 0x400000,
        ActivateAAAasIU = 0x800000,
        Reserved6 = 0x1000000,
        ActivateArm32Server = 0x2000000,
        // ALLOW_LOWER_TRUST_REGISTRATION = ?,
        PsDll = 0x80000000,
    }

    pub unsafe fn CoInitializeEx(model: ConcurrencyModel) -> Result<(), i32> {
        extern "system" {
            fn CoInitializeEx(pvReserved: *mut std::ffi::c_void, dwCoInit: u32) -> i32;
        }
        CoInitializeEx(null_mut(), transmute(model)).into_result()
    }

    pub unsafe fn CoCreateInstance<T>(
        class_id: *const GUID,
        context: ExecutionContext,
        interface_id: *const GUID,
    ) -> Result<T, i32> {
        extern "system" {
            pub fn CoCreateInstance(
                rclsid: *const GUID,
                pUnkOuter: *mut *mut c_void,
                dwClsContext: u32,
                riid: *const GUID,
                ppv: *mut *mut c_void,
            ) -> i32;
        }

        let mut out = std::ptr::null_mut();
        check(
            CoCreateInstance(
                class_id,
                null_mut(),
                transmute(context),
                interface_id,
                &mut out,
            ),
            out as *mut T,
        )
    }
}

pub unsafe fn mkpad() {
    use com::*;

    CoInitializeEx(ConcurrencyModel::MultiThreaded).unwrap();

    let enumerator: IMMDeviceEnumerator = com::CoCreateInstance(
        &MMDeviceEnumerator,
        ExecutionContext::All,
        &IMMDeviceEnumerator::IID,
    )
    .unwrap();

    pub unsafe fn device_name(device: &IMMDevice) -> String {
        let store = device.OpenPropertyStore(STGM_READ).unwrap();
        let prop = store.GetValue(&PKEY_Device_FriendlyName).unwrap();
        assert!(prop.Anonymous.Anonymous.vt == VT_LPWSTR);
        let data = prop.Anonymous.Anonymous.Anonymous.pwszVal;
        data.to_string().unwrap()
    }

    let collection = enumerator
        .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
        .unwrap();

    let devices: Vec<(String, IMMDevice)> = (0..collection.GetCount().unwrap())
        .map(|i| {
            let device = collection.Item(i).unwrap();
            let name = device_name(&device);
            (name, device)
        })
        .collect();

    let default = enumerator
        .GetDefaultAudioEndpoint(eRender, eConsole)
        .unwrap();

    let default = (device_name(&default), default);

    dbg!(devices, default);
}
