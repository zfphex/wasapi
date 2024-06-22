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
        .as_result(out)
    }
}

pub mod audio_endpoints {
    use crate::*;
    use core::{ffi::c_void, mem::transmute_copy};
    use makepad_windows::{
        core::{GUID, HRESULT},
        Win32::{
            Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
            Media::Audio::IMMNotificationClient,
            System::{Com::StructuredStorage::PROPVARIANT, Variant::VT_LPWSTR},
            UI::Shell::PropertiesSystem::IPropertyStore,
        },
    };
    use std::ptr::{addr_of_mut, null_mut};

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
    /// Windows has a feature called 'communication devices', this allows you to set a different input and output devices
    /// when using communication software.
    /// This feature is essentially useless and isn't used in programs like Discord.
    /// I have never seen any mention of 'multimedia devices' in windows.
    /// It's also important to note that the *new* Windows 10 settings doesn't have any options to set device roles.
    /// My guess it was implemented in Windows 7 and immediatly abandoned, and since Microsoft doesn't change it's API ever,
    /// we're stuck writing pointless code.
    ///
    /// https://learn.microsoft.com/en-us/windows/win32/api/mmdeviceapi/ne-mmdeviceapi-erole
    /// https://learn.microsoft.com/en-us/windows/win32/coreaudio/device-roles-in-windows-vista
    pub enum Role {
        /// Games, system notification sounds, and voice commands.
        Console = 0,
        /// Music, movies, narration, and live music recording.
        Multimedia = 1,
        /// Voice communications (talking to another person).
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
            let this: *mut c_void = transmute_copy(self);
            (this, (&**(this as *mut *mut IMMDeviceEnumeratorVtbl)))
        }

        #[inline]
        pub unsafe fn EnumAudioEndpoints(
            &self,
            dataFlow: DataFlow,
            device_state: DeviceState,
        ) -> Result<IMMDeviceCollection, i32> {
            let mut devices = core::mem::zeroed();
            let (this, vtable) = self.vtable();
            (vtable.EnumAudioEndpoints)(this, dataFlow, device_state, &mut devices)
                .as_result(devices)
        }

        #[inline]
        pub unsafe fn GetDefaultAudioEndpoint(
            &self,
            dataFlow: DataFlow,
            role: Role,
        ) -> Result<IMMDevice, i32> {
            let mut device = core::mem::zeroed();
            let (this, vtable) = self.vtable();
            (vtable.GetDefaultAudioEndpoint)(this, dataFlow, role, &mut device).as_result(device)
        }

        #[inline]
        pub unsafe fn GetDevice(&self, str_id: *const u16) -> Result<IMMDevice, i32> {
            let mut device = core::mem::zeroed();
            let (this, vtable) = self.vtable();
            (vtable.GetDevice)(this, str_id, &mut device).as_result(device)
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

    // #[repr(C)]
    // pub struct PROPERTYKEY {
    //     pub fmtid: GUID,
    //     pub pid: u32,
    // }
    // pub const PKEY_DEVICE_FRIENDLY_NAME: PROPERTYKEY = PROPERTYKEY {
    //     fmtid: GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    //     pid: 14,
    // };

    #[repr(transparent)]
    #[derive(Debug, Clone)]
    pub struct IMMDevice(*mut c_void);

    impl IMMDevice {
        pub fn name(&self) -> makepad_windows::core::Result<String> {
            unsafe {
                //TODO: Error propagation is not working here. Need to write result types :/.
                let store = self.OpenPropertyStore(StorageAccessMode::Read).unwrap();
                let prop = store.GetValue(&PKEY_Device_FriendlyName)?;
                assert!(prop.Anonymous.Anonymous.vt == VT_LPWSTR);
                let data = prop.Anonymous.Anonymous.Anonymous.pwszVal;
                Ok(data.to_string()?)
            }
        }

        #[inline]
        pub unsafe fn vtable(&self) -> (*mut Self, &IMMDeviceVtbl) {
            let this: *mut IMMDevice = transmute_copy(self);
            (this, (&**(this as *mut *mut IMMDeviceVtbl)))
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
            let (this, vtable) = self.vtable();
            (vtable.OpenPropertyStore)(this, access_mode, &mut properties).as_result(properties)
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

    #[repr(C)]
    pub struct IMMDeviceCollectionVtbl {
        pub parent: IUnknownVtbl,
        pub GetCount: unsafe extern "system" fn(
            this: *mut IMMDeviceCollection,
            device_count: *const u32,
        ) -> HRESULT,
        pub Item: unsafe extern "system" fn(
            this: *mut IMMDeviceCollection,
            device_index: u32,
            device: *mut *mut c_void,
        ) -> HRESULT,
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct IMMDeviceCollection(*mut c_void);

    impl IMMDeviceCollection {
        #[inline]
        pub unsafe fn vtable(&self) -> (*mut Self, &IMMDeviceCollectionVtbl) {
            let this: *mut IMMDeviceCollection = transmute_copy(self);
            (this, (&**(this as *mut *mut IMMDeviceCollectionVtbl)))
        }

        #[inline]
        pub unsafe fn GetCount(&self) -> Result<u32, i32> {
            let (this, vtable) = self.vtable();
            let mut device_count = core::mem::zeroed();
            (vtable.GetCount)(this, &mut device_count).as_result_owned(device_count)
        }

        #[inline]
        pub unsafe fn Item(&self, device_index: u32) -> Result<IMMDevice, i32> {
            let (this, vtable) = self.vtable();
            let mut device = core::mem::zeroed();
            (vtable.Item)(this, device_index, &mut device).as_result(device)
        }
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
        .EnumAudioEndpoints(DataFlow::All, DeviceState::Active)
        .unwrap();

    let devices: Vec<IMMDevice> = (0..collection.GetCount().unwrap())
        .map(|i| collection.Item(i).unwrap())
        .collect();

    println!("Default: {}", default.name().unwrap());
    for device in devices {
        println!("{}", device.name().unwrap());
    }
}
