use crate::*;

#[repr(u32)]
pub enum ShareMode {
    Shared = 0,
    Exclusive = 1,
}

//https://learn.microsoft.com/en-us/windows/win32/coreaudio/audclnt-streamflags-xxx-constants
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 0x00010000;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 0x00020000;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 0x00040000;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 0x00080000;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 0x00100000;
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 0x80000000;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 0x08000000;
//https://learn.microsoft.com/en-us/windows/win32/coreaudio/audclnt-sessionflags-xxx-constants
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 0x10000000;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 0x20000000;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 0x40000000;

#[rustfmt::skip] 
#[repr(C)]
pub struct IAudioClientVtbl {
    pub parent: IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(
        this: *mut IAudioClient,
        share_mode: ShareMode,
        StreamFlags: u32,
        hnsBufferDuration: i64,
        hnsPeriodicity: i64,
        pFormat: *const WAVEFORMATEX,
        AudioSessionGuid: *const GUID,
    ) -> i32,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut IAudioClient, pNumBufferFrames: *mut u32) -> i32,
    pub GetStreamLatency: unsafe extern "system" fn(this: *mut IAudioClient, phnsLatency: *mut i64) -> i32,
    pub GetCurrentPadding: unsafe extern "system" fn(this: *mut IAudioClient, pNumPaddingFrames: *mut u32) -> i32,
    pub IsFormatSupported: unsafe extern "system" fn(
        this: *mut IAudioClient,
        ShareMode: i32,
        pFormat: *const WAVEFORMATEX,
        ppClosestMatch: *mut *mut WAVEFORMATEX,
    ) -> i32,
    pub GetMixFormat: unsafe extern "system" fn(
        this: *mut IAudioClient,
        device_format: *mut *mut WAVEFORMATEX,
        // device_format: *mut *mut c_void,
    ) -> i32,
    pub GetDevicePeriod: unsafe extern "system" fn(
        this: *mut IAudioClient,
        phnsDefaultDevicePeriod: *mut i64,
        phnsMinimumDevicePeriod: *mut i64,
    ) -> i32,
    pub Start: unsafe extern "system" fn(this: *mut IAudioClient) -> i32,
    pub Stop: unsafe extern "system" fn(this: *mut IAudioClient) -> i32,
    pub Reset: unsafe extern "system" fn(this: *mut IAudioClient) -> i32,
    pub SetEventHandle: unsafe extern "system" fn(this: *mut IAudioClient, eventHandle: isize) -> i32,
    pub GetService: unsafe extern "system" fn(
        this: *mut IAudioClient,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> i32,
}

#[repr(C)]
pub struct IAudioClient2Vtbl {
    pub parent: IAudioClientVtbl,
    // pub IsOffloadCapable: unsafe extern "system" fn(_: *mut c_void, _: AUDIO_STREAM_CATEGORY, _: *mut BOOL) -> HRESULT,
    // pub SetClientProperties: unsafe extern "system" fn(_: *mut c_void, _: *const AudioClientProperties) -> HRESULT,
    // pub GetBufferSizeLimits: unsafe extern "system" fn(_: *mut c_void, _: *const WAVEFORMATEX, _: BOOL, _: *mut i64, _: *mut i64) -> HRESULT,
}

#[repr(C)]
pub struct IAudioClient3Vtbl {
    pub parent: IAudioClient2Vtbl,
    // pub GetSharedModeEnginePeriod: unsafe extern "system" fn(_: *mut c_void, _: *const WAVEFORMATEX, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32) -> HRESULT,
    // pub GetCurrentSharedModeEnginePeriod: unsafe extern "system" fn(_: *mut c_void, _: *mut *mut WAVEFORMATEX, _: *mut u32) -> HRESULT,
    // pub InitializeSharedAudioStream: unsafe extern "system" fn(_: *mut c_void, _: u32, _: u32, _: *const WAVEFORMATEX, _: *const GUID) -> HRESULT,
}

// use makepad_windows::Win32::Media::Audio::IAudioClient;

#[repr(transparent)]
#[derive(Debug)]
pub struct IAudioClient(*mut c_void);

impl IAudioClient {
    pub const INTERFACE_ID: GUID = GUID::from_u128(0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);

    #[inline]
    pub unsafe fn vtable(&self) -> (*mut Self, &IAudioClientVtbl) {
        let this: *mut Self = transmute_copy(self);
        (this, (&**(this as *mut *mut IAudioClientVtbl)))
    }

    #[inline]
    pub unsafe fn Initialize(
        &self,
        share_mode: ShareMode,
        stream_flags: u32,
        buffer_duration: i64,
        periodicity: i64,
        wave_format: *const WAVEFORMATEX,
        guid: Option<*const GUID>,
    ) -> Result<(), i32> {
        let (this, vtable) = self.vtable();
        (vtable.Initialize)(
            this,
            share_mode,
            stream_flags,
            buffer_duration,
            periodicity,
            wave_format,
            guid.unwrap_or_else(|| core::mem::zeroed()),
        )
        .as_result_owned(())
    }

    #[inline]
    pub unsafe fn GetBufferSize(&self) -> Result<u32, i32> {
        let mut buffer_size = ::std::mem::zeroed();
        let (this, vtable) = self.vtable();
        (vtable.GetBufferSize)(this, &mut buffer_size).as_result_owned(buffer_size)
    }
    // #[inline]
    // pub unsafe fn GetStreamLatency(&self, phnsLatency: *mut i64) -> HRESULT {
    //     ((*self.lpVtbl).GetStreamLatency)(self as *const _ as *mut _, phnsLatency)
    // }
    // #[inline]
    // pub unsafe fn GetCurrentPadding(&self, pNumPaddingFrames: *mut u32) -> HRESULT {
    //     ((*self.lpVtbl).GetCurrentPadding)(self as *const _ as *mut _, pNumPaddingFrames)
    // }
    // #[inline]
    // pub unsafe fn IsFormatSupported(
    //     &self,
    //     ShareMode: i32,
    //     pFormat: *const WAVEFORMATEX,
    //     ppClosestMatch: *mut *mut WAVEFORMATEX,
    // ) -> HRESULT {
    //     ((*self.lpVtbl).IsFormatSupported)(
    //         self as *const _ as *mut _,
    //         ShareMode,
    //         pFormat,
    //         ppClosestMatch,
    //     )
    // }

    #[inline]
    pub unsafe fn GetMixFormat(&self) -> Result<*mut WAVEFORMATEX, i32> {
        let (this, vtable) = self.vtable();
        let mut device_format = std::mem::zeroed();
        (vtable.GetMixFormat)(this, &mut device_format).as_result(device_format)
    }

    #[inline]
    ///(Default, Minimum)
    pub unsafe fn GetDevicePeriod(&self) -> Result<(i64, i64), i32> {
        let (this, vtable) = self.vtable();
        let mut default_device_period = core::mem::zeroed();
        let mut min_device_period = core::mem::zeroed();
        (vtable.GetDevicePeriod)(this, &mut default_device_period, &mut min_device_period)
            .as_result_owned((default_device_period, min_device_period))
    }

    #[inline]
    pub unsafe fn Start(&self) -> Result<(), i32> {
        let (this, vtable) = self.vtable();
        (vtable.Start)(this).as_result_owned(())
    }

    #[inline]
    pub unsafe fn Stop(&self) ->  Result<(), i32> {
        let (this, vtable) = self.vtable();
        (vtable.Stop)(this).as_result_owned(())
    }

    #[inline]
    pub unsafe fn Reset(&self) ->  Result<(), i32> {
        let (this, vtable) = self.vtable();
        (vtable.Reset)(this).as_result_owned(())
    }

    #[inline]
    pub unsafe fn SetEventHandle(&self, event_handle: isize) -> Result<(), i32> {
        let (this, vtable) = self.vtable();
        (vtable.SetEventHandle)(this, event_handle).as_result_owned(())
    }

    #[inline]
    pub unsafe fn GetService(
        &self,
    ) -> Result<IAudioRenderClient, i32> {
        let (this, vtable) = self.vtable();
        let mut service = core::ptr::null_mut();
        (vtable.GetService)(this, &IAudioRenderClient::id() as *const GUID, &mut service).as_result(service)
    }
}

impl Interface for IAudioClient {
    #[inline]
    fn id() -> GUID {
        IAudioClient::INTERFACE_ID
    }
}

// #[repr(C)]
// pub struct IAudioCaptureClient_Vtbl {
//     pub base__: ::windows_core::IUnknown_Vtbl,
//     pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::HRESULT,
//     pub ReleaseBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows_core::HRESULT,
//     pub GetNextPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows_core::HRESULT,
// }

// pub trait IAudioCaptureClient_Impl: Sized {
//     fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::Result<()>;
//     fn ReleaseBuffer(&self, numframesread: u32) -> ::windows_core::Result<()>;
//     fn GetNextPacketSize(&self) -> ::windows_core::Result<u32>;
// }

// impl IAudioCaptureClient_Vtbl {
//     pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>() -> IAudioCaptureClient_Vtbl {
//         unsafe extern "system" fn GetBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::HRESULT {
//             let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
//             let this = (*this).get_impl();
//             this.GetBuffer(::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pnumframestoread), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pu64deviceposition), ::core::mem::transmute_copy(&pu64qpcposition)).into()
//         }
//         unsafe extern "system" fn ReleaseBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows_core::HRESULT {
//             let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
//             let this = (*this).get_impl();
//             this.ReleaseBuffer(::core::mem::transmute_copy(&numframesread)).into()
//         }
//         unsafe extern "system" fn GetNextPacketSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows_core::HRESULT {
//             let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
//             let this = (*this).get_impl();
//             match this.GetNextPacketSize() {
//                 ::core::result::Result::Ok(ok__) => {
//                     ::core::ptr::write(pnumframesinnextpacket, ::core::mem::transmute(ok__));
//                     ::windows_core::HRESULT(0)
//                 }
//                 ::core::result::Result::Err(err) => err.into(),
//             }
//         }
//         Self {
//             base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
//             GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
//             ReleaseBuffer: ReleaseBuffer::<Identity, Impl, OFFSET>,
//             GetNextPacketSize: GetNextPacketSize::<Identity, Impl, OFFSET>,
//         }
//     }
//     pub fn matches(iid: &::windows_core::GUID) -> bool {
//         iid == &<IAudioCaptureClient as ::windows_core::ComInterface>::IID
//     }
// }

// impl IAudioCaptureClient_Vtbl {
//     pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>() -> IAudioCaptureClient_Vtbl {
//         unsafe extern "system" fn GetBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::HRESULT {
//             let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
//             let this = (*this).get_impl();
//             this.GetBuffer(::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pnumframestoread), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pu64deviceposition), ::core::mem::transmute_copy(&pu64qpcposition)).into()
//         }
//         unsafe extern "system" fn ReleaseBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows_core::HRESULT {
//             let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
//             let this = (*this).get_impl();
//             this.ReleaseBuffer(::core::mem::transmute_copy(&numframesread)).into()
//         }
//         unsafe extern "system" fn GetNextPacketSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows_core::HRESULT {
//             let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
//             let this = (*this).get_impl();
//             match this.GetNextPacketSize() {
//                 ::core::result::Result::Ok(ok__) => {
//                     ::core::ptr::write(pnumframesinnextpacket, ::core::mem::transmute(ok__));
//                     ::windows_core::HRESULT(0)
//                 }
//                 ::core::result::Result::Err(err) => err.into(),
//             }
//         }
//         Self {
//             base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
//             GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
//             ReleaseBuffer: ReleaseBuffer::<Identity, Impl, OFFSET>,
//             GetNextPacketSize: GetNextPacketSize::<Identity, Impl, OFFSET>,
//         }
//     }
//     pub fn matches(iid: &::windows_core::GUID) -> bool {
//         iid == &<IAudioCaptureClient as ::windows_core::ComInterface>::IID
//     }
// }

#[rustfmt::skip] 
#[repr(C)]
pub struct IAudioRenderClientVtbl {
    pub parent: IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> i32,
    pub ReleaseBuffer: unsafe extern "system" fn(this: *mut c_void, numframeswritten: u32, dwflags: u32) -> i32,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct IAudioRenderClient(*mut c_void);

impl IAudioRenderClient {
    pub const INTERFACE_ID: GUID = GUID::from_u128(0xf294acfc_3146_4483_a7bf_addca7c260e2);
    // pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> ::windows_core::Result<*mut u8> {
    //     let mut result__ = ::std::mem::zeroed();
    //     (::windows_core::Interface::vtable(self).GetBuffer)(::windows_core::Interface::as_raw(self), numframesrequested, &mut result__).from_abi(result__)
    // }
    // pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> ::windows_core::Result<()> {
    //     (::windows_core::Interface::vtable(self).ReleaseBuffer)(::windows_core::Interface::as_raw(self), numframeswritten, dwflags).ok()
    // }
}

impl Interface for IAudioRenderClient {
    fn id() -> GUID {
        Self::INTERFACE_ID
    }
}
