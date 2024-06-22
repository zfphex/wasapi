use makepad_windows::{
    core::{GUID, HRESULT},
    Win32::{
        Foundation::DECIMAL,
        System::{Com::StructuredStorage::PROPVARIANT_0_0_0, Variant::VARENUM},
    },
};

use crate::*;

#[repr(C)]
pub struct PROPERTYKEY {
    pub fmtid: GUID,
    pub pid: u32,
}

pub const PKEY_DEVICE_FRIENDLY_NAME: PROPERTYKEY = PROPERTYKEY {
    fmtid: GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
    pid: 14,
};

#[repr(C)]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT0,
}

#[repr(C)]
pub union PROPVARIANT0 {
    pub Anonymous: std::mem::ManuallyDrop<PROPVARIANT0_0>,
    pub decVal: DECIMAL,
}

#[repr(C)]
pub struct PROPVARIANT0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}

#[repr(C)]
pub struct IPropertyStoreVtbl {
    pub parent: IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut IPropertyStore, cProps: *mut u32) -> HRESULT,
    pub GetAt: unsafe extern "system" fn(
        this: *mut IPropertyStore,
        iProp: u32,
        propkey: *mut PROPERTYKEY,
    ) -> HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut IPropertyStore,
        key: *const PROPERTYKEY,
        propvar: *mut PROPVARIANT,
    ) -> HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut IPropertyStore,
        key: *const PROPERTYKEY,
        propvar: *const PROPVARIANT,
    ) -> HRESULT,
    pub Commit: unsafe extern "system" fn(This: *mut IPropertyStore) -> HRESULT,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct IPropertyStore(*mut c_void);

impl IPropertyStore {
    #[inline]
    pub unsafe fn vtable(&self) -> (*mut Self, &IPropertyStoreVtbl) {
        let this: *mut Self = transmute_copy(self);
        (this, (&**(this as *mut *mut IPropertyStoreVtbl)))
    }

    #[inline]
    pub unsafe fn GetCount(&self, cProps: *mut u32) -> HRESULT {
        let (this, vtable) = self.vtable();
        (vtable.GetCount)(this, cProps)
    }

    #[inline]
    pub unsafe fn GetAt(&self, iProp: u32) -> Result<PROPERTYKEY, i32> {
        let (this, vtable) = self.vtable();
        let propkey = zeroed();
        (vtable.GetAt)(this, iProp, propkey).as_result(propkey)
    }

    #[inline]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT, i32> {
        let (this, vtable) = self.vtable();
        let mut propvar = zeroed();
        (vtable.GetValue)(this, key, &mut propvar).as_result_owned(propvar)
    }

    #[inline]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const PROPVARIANT) -> HRESULT {
        let (this, vtable) = self.vtable();
        (vtable.SetValue)(this, key, propvar)
    }

    #[inline]
    pub unsafe fn Commit(&self) -> HRESULT {
        let (this, vtable) = self.vtable();
        (vtable.Commit)(this)
    }
}
