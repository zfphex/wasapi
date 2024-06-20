#![allow(non_snake_case, unused)]
use crate::{WindowsResult, WindowsResultEmpty};
use makepad_windows::core::HRESULT;
use makepad_windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use makepad_windows::Win32::System::Variant::VT_LPWSTR;

pub mod com {
    #![allow(unused)]
    use crate::*;
    use core::{ffi::c_void, mem::transmute, ptr::null_mut};
    use makepad_windows::core::GUID;

    #[link(name = "ole32")]
    extern "system" {}

    ///https://learn.microsoft.com/en-us/windows/win32/api/objbase/ne-objbase-coinit
    #[repr(u32)]
    #[derive(Debug)]
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
    #[derive(Debug)]
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
        CoInitializeEx(null_mut(), transmute(model)).ok()
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
        CoCreateInstance(
            class_id,
            null_mut(),
            transmute(context),
            interface_id,
            &mut out,
        )
        .into_result(out)
    }
}

pub mod audio_endpoints {
    use core::{ffi::c_void, mem::transmute_copy};
    use makepad_windows::{
        core::{GUID, HRESULT},
        Win32::{
            Media::Audio::{IMMDeviceCollection, IMMNotificationClient},
            System::Com::StructuredStorage::PROPVARIANT,
            UI::Shell::PropertiesSystem::IPropertyStore,
        },
    };
    // use makepad_windows::core::Result

    pub const CLSID_MM_DEVICE_ENUMERATOR: GUID =
        GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
    pub const IID_IMM_DEVICE_ENUMERATOR: GUID =
        GUID::from_u128(0xa95664d2_9614_4f35_a746_de8db63617e6);

    #[repr(u32)]
    #[derive(Debug)]
    pub enum DataFlow {
        Render = 0,
        Capture = 1,
        All = 2,
    }

    #[repr(u32)]
    #[derive(Debug)]
    pub enum Role {
        Console = 0,
        Multimedia = 1,
        Communications = 2,
    }

    //TODO: These are flags but rust is stupid. So no ORing repr(u32) enums.
    #[repr(u32)]
    #[derive(Debug)]
    pub enum DeviceState {
        Active = 1,
        Disabled = 2,
        NotPresent = 4,
        Unplugged = 8,
        ///Warning: this will cause rust to panic.
        All = 15,
    }

    #[repr(u32)]
    #[derive(Debug)]
    pub enum StorageAccessMode {
        Read = 0,
        Write = 1,
        ReadWrite = 2,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct IUnknownVtbl {
        pub QueryInterface: unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            iid: &GUID,
            interface: *mut *const std::ffi::c_void,
        ) -> HRESULT,
        pub AddRef: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> u32,
        pub Release: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> u32,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct IMMDeviceEnumeratorVtbl {
        pub base: IUnknownVtbl,
        pub EnumAudioEndpoints: unsafe extern "system" fn(
            this: *mut c_void,
            data_flow: DataFlow,       //u32
            device_state: DeviceState, //u32
            devices: *mut *mut c_void,
        ) -> HRESULT,
        pub GetDefaultAudioEndpoint: unsafe extern "system" fn(
            this: *mut c_void,
            data_flow: DataFlow, //u32
            role: Role,          //u32
            endpoint: *mut *mut c_void,
        ) -> HRESULT,
        pub GetDevice: unsafe extern "system" fn(
            this: *mut c_void,
            str_id: *const u16,
            device: *mut *mut c_void,
        ) -> HRESULT,
        pub RegisterEndpointNotificationCallback:
            unsafe extern "system" fn(this: *mut c_void, client: *mut c_void) -> HRESULT,
        pub UnregisterEndpointNotificationCallback:
            unsafe extern "system" fn(this: *mut c_void, client: *mut c_void) -> HRESULT,
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct IMMDeviceEnumerator(*mut c_void);

    impl IMMDeviceEnumerator {
        #[inline]
        pub unsafe fn vtable(&self) -> (*mut c_void, &IMMDeviceEnumeratorVtbl) {
            let raw: *mut c_void = transmute_copy(self);
            (raw, (&**(raw as *mut *mut IMMDeviceEnumeratorVtbl)))
        }

        #[inline]
        pub unsafe fn EnumAudioEndpoints(
            &self,
            dataFlow: DataFlow,
            device_state: DeviceState,
        ) -> Result<IMMDeviceCollection, i32> {
            let mut devices = core::mem::zeroed();
            let (raw, vtable) = self.vtable();
            (vtable.EnumAudioEndpoints)(raw, dataFlow, device_state, &mut devices)
                .into_result(devices)
        }

        #[inline]
        pub unsafe fn GetDefaultAudioEndpoint(
            &self,
            dataFlow: DataFlow,
            role: Role,
        ) -> Result<IMMDevice, i32> {
            let mut device = core::mem::zeroed();
            let (raw, vtable) = self.vtable();
            (vtable.GetDefaultAudioEndpoint)(raw, dataFlow, role, &mut device).into_result(device)
        }

        #[inline]
        pub unsafe fn GetDevice(&self, str_id: *const u16) -> Result<IMMDevice, i32> {
            let mut device = core::mem::zeroed();
            let (raw, vtable) = self.vtable();
            (vtable.GetDevice)(raw, str_id, &mut device).into_result(device)
        }

        // #[inline]
        // pub unsafe fn RegisterEndpointNotificationCallback(
        //     &self,
        //     pClient: *mut IMMNotificationClient,
        // ) -> HRESULT {
        //     // ((*self.vtable).RegisterEndpointNotificationCallback)(
        //     //     self as *const _ as *mut _,
        //     //     pClient,
        //     // )
        //     todo!()
        // }

        // #[inline]
        // pub unsafe fn UnregisterEndpointNotificationCallback(
        //     &self,
        //     pClient: *mut IMMNotificationClient,
        // ) -> HRESULT {
        //     // ((*self.vtable).UnregisterEndpointNotificationCallback)(
        //     //     self as *const _ as *mut _,
        //     //     pClient,
        //     // )
        //     todo!()
        // }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct IMMDeviceVtbl {
        pub parent: IUnknownVtbl,
        pub Activate: unsafe extern "system" fn(
            this: *mut IMMDevice,
            iid: *const GUID,
            cls_ctx: u32,
            activation_params: *mut PROPVARIANT,
            interface: *mut *mut c_void,
        ) -> i32,
        pub OpenPropertyStore: unsafe extern "system" fn(
            this: *mut IMMDevice,
            stgm_access: StorageAccessMode,
            properties: *mut *mut IPropertyStore,
        ) -> i32,
        pub GetId: unsafe extern "system" fn(this: *mut IMMDevice, str_id: *mut *mut u16) -> i32,
        pub GetState: unsafe extern "system" fn(this: *mut IMMDevice, state: *mut u32) -> i32,
    }

    use super::WindowsResult;

    pub const STGM_READ: u32 = 0;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct IMMDevice(*mut c_void);

    impl IMMDevice {
        #[inline]
        pub unsafe fn vtable(&self) -> (*mut IMMDevice, &IMMDeviceVtbl) {
            let raw: *mut IMMDevice = transmute_copy(self);
            (raw, (&**(raw as *mut *mut IMMDeviceVtbl)))
        }

        // #[inline]
        // pub unsafe fn Activate(
        //     &self,
        //     iid: REFIID,
        //     dwClsCtx: DWORD,
        //     pActivationParams: *mut PROPVARIANT,
        //     ppInterface: *mut LPVOID,
        // ) -> HRESULT {
        //     ((*self.lpVtbl).Activate)(
        //         self as *const _ as *mut _,
        //         iid,
        //         dwClsCtx,
        //         pActivationParams,
        //         ppInterface,
        //     )
        // }

        #[inline]
        pub unsafe fn OpenPropertyStore(
            &self,
            access_mode: StorageAccessMode,
        ) -> Result<IPropertyStore, i32> {
            let mut properties = core::mem::zeroed();
            let (raw, vtable) = self.vtable();
            (vtable.OpenPropertyStore)(raw, access_mode, &mut properties).into_result(properties)
        }

        // #[inline]
        // pub unsafe fn GetId(&self, ppstrId: *mut LPWSTR) -> HRESULT {
        //     ((*self.lpVtbl).GetId)(self as *const _ as *mut _, ppstrId)
        // }
        // #[inline]
        // pub unsafe fn GetState(&self, pdwState: *mut DWORD) -> HRESULT {
        //     ((*self.lpVtbl).GetState)(self as *const _ as *mut _, pdwState)
        // }
    }
}

pub unsafe fn mkpad() {
    use audio_endpoints::*;
    use com::*;

    CoInitializeEx(ConcurrencyModel::MultiThreaded).unwrap();

    let enumerator: audio_endpoints::IMMDeviceEnumerator = com::CoCreateInstance(
        &CLSID_MM_DEVICE_ENUMERATOR,
        ExecutionContext::All,
        &IID_IMM_DEVICE_ENUMERATOR,
    )
    .unwrap();

    let default = enumerator
        .GetDefaultAudioEndpoint(DataFlow::Render, Role::Console)
        .unwrap();

    let collection = enumerator
        .EnumAudioEndpoints(DataFlow::Render, DeviceState::Active)
        .unwrap();

    // dbg!(default);

    pub unsafe fn device_name(device: &IMMDevice) -> String {
        let store = device.OpenPropertyStore(StorageAccessMode::Read).unwrap();
        let prop = store.GetValue(&PKEY_Device_FriendlyName).unwrap();
        assert!(prop.Anonymous.Anonymous.vt == VT_LPWSTR);
        let data = prop.Anonymous.Anonymous.Anonymous.pwszVal;
        data.to_string().unwrap()
    }

    // let devices: Vec<(String, IMMDevice)> = (0..collection.GetCount().unwrap())
    //     .map(|i| {
    //         let device = collection.Item(i).unwrap();
    //         let name = device_name(&device);
    //         (name, device)
    //     })
    //     .collect();

    let default = (device_name(&default), default);
    dbg!(default);

    // dbg!(devices, default);
}
