use std::ptr::null_mut;

unsafe fn winapi() {
    use winapi::{
        shared::{
            guiddef::GUID,
            minwindef::LPVOID,
            winerror::{CLASS_E_NOAGGREGATION, E_NOINTERFACE, E_POINTER, REGDB_E_CLASSNOTREG},
        },
        um::{
            combaseapi::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINITBASE_MULTITHREADED},
            mmdeviceapi::CLSID_MMDeviceEnumerator,
        },
        Class, Interface,
    };

    pub const fn from_u128(uuid: u128) -> GUID {
        GUID {
            Data1: (uuid >> 96) as u32,
            Data2: (uuid >> 80 & 0xffff) as u16,
            Data3: (uuid >> 64 & 0xffff) as u16,
            Data4: (uuid as u64).to_be_bytes(),
        }
    }

    pub const MMDeviceEnumerator: GUID = from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);

    const IID: GUID = from_u128(0xa95664d2_9614_4f35_a746_de8db63617e6);
    let result = CoInitializeEx(null_mut(), COINITBASE_MULTITHREADED);
    assert_eq!(result, 0);

    let ppv: *mut LPVOID = null_mut();
    let result = CoCreateInstance(
        &MMDeviceEnumerator,
        std::mem::zeroed(),
        CLSCTX_ALL,
        &IID, // &IMMDeviceEnumerator::uuidof(),
        ppv,
    );
    match result {
        0 => {}
        REGDB_E_CLASSNOTREG => panic!("A specified class is not registered in the registration database. Also can indicate that the type of server you requested in the CLSCTX enumeration is not registered or the values for the server types in the registry are corrupt."),
        CLASS_E_NOAGGREGATION => panic!("This class cannot be created as part of an aggregate."),
        E_NOINTERFACE => panic!("The specified class does not implement the requested interface, or the controlling IUnknown does not expose the requested interface."),
        E_POINTER => panic!("The ppv parameter is NULL."),
        _ => panic!("result: {:#02x}", result),
    }
}
