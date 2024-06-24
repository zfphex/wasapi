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
    pub GetBufferSize:
        unsafe extern "system" fn(this: *mut IAudioClient, pNumBufferFrames: *mut u32) -> i32,
    pub GetStreamLatency:
        unsafe extern "system" fn(this: *mut IAudioClient, phnsLatency: *mut i64) -> i32,
    pub GetCurrentPadding:
        unsafe extern "system" fn(this: *mut IAudioClient, pNumPaddingFrames: *mut u32) -> i32,
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
    pub SetEventHandle:
        unsafe extern "system" fn(this: *mut IAudioClient, eventHandle: isize) -> i32,
    pub GetService: unsafe extern "system" fn(
        this: *mut IAudioClient,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> i32,
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
        guid: *const GUID,
    ) -> Result<(), i32> {
        let (this, vtable) = self.vtable();
        (vtable.Initialize)(
            this,
            share_mode,
            stream_flags,
            buffer_duration,
            periodicity,
            wave_format,
            guid,
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
    // #[inline]
    // pub unsafe fn Start(&self) -> HRESULT {
    //     ((*self.lpVtbl).Start)(self as *const _ as *mut _)
    // }
    // #[inline]
    // pub unsafe fn Stop(&self) -> HRESULT {
    //     ((*self.lpVtbl).Stop)(self as *const _ as *mut _)
    // }
    // #[inline]
    // pub unsafe fn Reset(&self) -> HRESULT {
    //     ((*self.lpVtbl).Reset)(self as *const _ as *mut _)
    // }
    // #[inline]
    // pub unsafe fn SetEventHandle(&self, eventHandle: isize) -> HRESULT {
    //     ((*self.lpVtbl).SetEventHandle)(self as *const _ as *mut _, eventHandle)
    // }
    // #[inline]
    // pub unsafe fn GetService(&self, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    //     ((*self.lpVtbl).GetService)(self as *const _ as *mut _, riid, ppv)
    // }
}

impl Interface for IAudioClient {
    #[inline]
    fn id() -> GUID {
        IAudioClient::INTERFACE_ID
    }
}
