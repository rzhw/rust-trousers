extern crate trousers_sys;

use std::error;
use std::ffi;
use std::fmt;
use std::slice;
use trousers_sys::trousers::*;
use trousers_sys::tspi::*;

pub type TssFlag = u32;
pub type TssHObject = u32;
pub type TssHContext = TssHObject;
pub type TssHTPM = TssHObject;
pub type TssHPCRS = TssHObject;
pub type TssResult = u32;
pub type TssUnicode = u16;

// TODO move this to a separate module
#[derive(Debug)]
pub struct TssError {
    pub result: TssResult
}

impl fmt::Display for TssError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt.write_str(error::Error::description(self)));
        Ok(())
    }
}

impl error::Error for TssError {
    fn description(&self) -> &str {
        let c_buf = unsafe { Trspi_Error_String(self.result) };
        let c_str = unsafe { ffi::CStr::from_ptr(c_buf) };
        let buf: &[u8] = c_str.to_bytes();
        let str_slice: &str = std::str::from_utf8(buf).unwrap();
        str_slice
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

// TODO macros for the funcitons below

// TODO move constants into a separate module?

pub const TSS_TSPATTRIB_KEY_INFO: TssFlag = 0x00000080;
pub const TSS_TSPATTRIB_KEYINFO_ALGORITHM: TssFlag = 0x00000280;
pub const TSS_TSPATTRIB_RSAKEY_INFO: TssFlag = 0x00000140;
pub const TSS_TSPATTRIB_KEYINFO_RSA_PRIMES: TssFlag = 0x00004000;
pub const TSS_ALG_RSA: TssFlag = 0x20;

pub const TSS_SUCCESS: TssResult = 0;

pub const TSS_UUID_SRK: TSS_UUID = TSS_UUID { ulTimeLow: 0, usTimeMid: 0, usTimeHigh: 0, bClockSeqHigh: 0, bClockSeqLow: 0, rgbNode: [0, 0, 0, 0, 0, 1] };

pub const TSS_PS_TYPE_USER: TssFlag = 1;
pub const TSS_PS_TYPE_SYSTEM: TssFlag = 2;

const TSS_OBJECT_TYPE_POLICY: TssFlag = 1;
const TSS_OBJECT_TYPE_RSAKEY: TssFlag = 2;
const TSS_OBJECT_TYPE_PCRS: TssFlag = 4;

const TSS_POLICY_USAGE: TssFlag = 1;
const TSS_POLICY_MIGRATION: TssFlag = 2;
const TSS_POLICY_OPERATOR: TssFlag = 3;
const TSS_PCRS_STRUCT_DEFAULT: TssFlag = 0;
const TSS_PCRS_STRUCT_INFO: TssFlag = 1;
const TSS_PCRS_STRUCT_INFO_LONG: TssFlag = 2;
const TSS_PCRS_STRUCT_INFO_SHORT: TssFlag = 3;

const TSS_KEY_NO_AUTHORIZATION: TssFlag = 0x00000000;
const TSS_KEY_AUTHORIZATION: TssFlag = 0x00000001;
const TSS_KEY_AUTHORIZATION_PRIV_USE_ONLY: TssFlag = 0x00000002;

const TSS_KEY_NON_VOLATILE: TssFlag = 0x00000000;
const TSS_KEY_VOLATILE: TssFlag = 0x00000004;

const TSS_KEY_NOT_MIGRATABLE: TssFlag = 0x00000000;
const TSS_KEY_MIGRATABLE: TssFlag = 0x00000008;

// TODO: certified

const TSS_KEY_TYPE_SIGNING: TssFlag = 0x00000010;
const TSS_KEY_TYPE_STORAGE: TssFlag = 0x00000020;
const TSS_KEY_TYPE_IDENTITY: TssFlag = 0x00000030;
const TSS_KEY_TYPE_AUTHCHANGE: TssFlag = 0x00000040;
const TSS_KEY_TYPE_BIND: TssFlag = 0x00000050;
const TSS_KEY_TYPE_LEGACY: TssFlag = 0x00000060;
const TSS_KEY_TYPE_MIGRATE: TssFlag = 0x00000070;

const TSS_KEY_SIZE_DEFAULT: TssFlag = 0x00000000;
const TSS_KEY_SIZE_512: TssFlag = 0x00000100;
const TSS_KEY_SIZE_1024: TssFlag = 0x00000200;
const TSS_KEY_SIZE_2048: TssFlag = 0x00000300;
const TSS_KEY_SIZE_4096: TssFlag = 0x00000400;
const TSS_KEY_SIZE_8192: TssFlag = 0x00000500;
const TSS_KEY_SIZE_16384: TssFlag = 0x00000600;

const TSS_KEY_STRUCT_DEFAULT: TssFlag = 0x00000000;
const TSS_KEY_STRUCT_KEY: TssFlag = 0x00004000;
const TSS_KEY_STRUCT_KEY12: TssFlag = 0x00008000;

pub enum TssPolicyInitFlag {
    Usage, Migration, Operator
}

pub enum TssKeySize {
    Default = TSS_KEY_SIZE_DEFAULT as isize,
    Size512 = TSS_KEY_SIZE_512 as isize,
    Size1024 = TSS_KEY_SIZE_1024 as isize,
    Size2048 = TSS_KEY_SIZE_2048 as isize,
    Size4096 = TSS_KEY_SIZE_4096 as isize,
    Size8192 = TSS_KEY_SIZE_8192 as isize,
    Size16384 = TSS_KEY_SIZE_16384 as isize
}
pub enum TssKeyType {
    Signing = TSS_KEY_TYPE_SIGNING as isize,
    Storage = TSS_KEY_TYPE_STORAGE as isize,
    Identity = TSS_KEY_TYPE_IDENTITY as isize,
    AuthChange = TSS_KEY_TYPE_AUTHCHANGE as isize,
    Bind = TSS_KEY_TYPE_BIND as isize,
    Legacy = TSS_KEY_TYPE_LEGACY as isize,
    Migrate = TSS_KEY_TYPE_MIGRATE as isize
}
pub enum TssKeyAuthorization {
    NoAuthorization = TSS_KEY_NO_AUTHORIZATION as isize,
    Authorization = TSS_KEY_AUTHORIZATION as isize,
    AuthorizationPrivUseOnly = TSS_KEY_AUTHORIZATION_PRIV_USE_ONLY as isize
}
pub enum TssKeyVolatility {
    NonVolatile = TSS_KEY_NON_VOLATILE as isize,
    Volatile = TSS_KEY_VOLATILE as isize
}
pub enum TssKeyMigratability {
    NotMigratable = TSS_KEY_NOT_MIGRATABLE as isize,
    Migratable = TSS_KEY_MIGRATABLE as isize
}
pub enum TssKeyStruct {
    Default = TSS_KEY_STRUCT_DEFAULT as isize,
    Key = TSS_KEY_STRUCT_KEY as isize,
    Key12 = TSS_KEY_STRUCT_KEY12 as isize
}

pub trait TssObject {
    fn get_handle(&self) -> TssHObject;
    fn set_attrib_uint32(&self, attrib_flag: TssFlag, sub_flag: TssFlag, attrib: u32) -> Result<(), TssError>;
    fn set_attrib_data(&self, attrib_flag: TssFlag, sub_flag: TssFlag, attrib_data: &[u8]) -> Result<(), TssError>;
}

pub struct TssContext {
    pub handle: u32
}
pub struct TssTPM<'context> {
    pub context: &'context TssContext,
    pub handle: u32
}

pub enum TssPcrsStructType {
    Default, Info, InfoLong, InfoShort
}

// TODO: need to have Drop?
pub struct TssPolicy<'context> {
    pub context: &'context TssContext,
    pub handle: TssHPCRS
}

// TODO: need to have Drop?
pub struct TssRsaKey<'context> {
    pub context: &'context TssContext,
    pub handle: TssHObject
}

pub struct TssValidation {
    pub version_info: TSS_VERSION,
    pub external_data: Vec<u8>,
    pub data: Vec<u8>,
    pub validation_data: Vec<u8>
}

pub struct TssPCRCompositeInfo<'context> {
    pub context: &'context TssContext,
    pub handle: TssHPCRS
}
pub struct TssPCRCompositeInfoLong<'context> {
    pub context: &'context TssContext,
    pub handle: TssHPCRS
}
pub struct TssPCRCompositeInfoShort<'context> {
    pub context: &'context TssContext,
    pub handle: TssHPCRS
}
pub trait TcpaPcrInfoAny {
    fn get_handle(&self) -> TssHPCRS;
}
#[allow(non_camel_case_types)]
pub trait TcpaPcrInfo1_1 : TcpaPcrInfoAny {
    fn get_handle(&self) -> TssHPCRS;
}
#[allow(non_camel_case_types)]
pub trait TcpaPcrInfo1_2 : TcpaPcrInfoAny {
    fn get_handle(&self) -> TssHPCRS;
    fn select_pcr_index_ex(&self, pcr_index: u32, direction: u32) -> Result<(), TssError>;
}
impl<'c> TcpaPcrInfoAny for TssPCRCompositeInfo<'c> {
    fn get_handle(&self) -> TssHPCRS { self.handle }
}
impl<'c> TcpaPcrInfo1_1 for TssPCRCompositeInfo<'c> {
    fn get_handle(&self) -> TssHPCRS { self.handle }
}
impl<'c> TcpaPcrInfoAny for TssPCRCompositeInfoLong<'c> {
    fn get_handle(&self) -> TssHPCRS { self.handle }
}
impl<'c> TcpaPcrInfoAny for TssPCRCompositeInfoShort<'c> {
    fn get_handle(&self) -> TssHPCRS { self.handle }
}

fn copy_raw_ptr_to_vec(ptr: *const u8, length: usize) -> Vec<u8> {
    let ptr_slice = unsafe {
        slice::from_raw_parts(ptr, length)
    };
    let mut vec = Vec::new();
    for byte in ptr_slice {
        vec.push(*byte)
    }
    vec
}

fn set_attrib_uint32_impl(object: &TssObject, attrib_flag: TssFlag, sub_flag: TssFlag, attrib: u32) -> Result<(), TssError> {
    let result = unsafe {
        Tspi_SetAttribUint32(object.get_handle(), attrib_flag, sub_flag, attrib)
    };
    if result != TSS_SUCCESS {
        return Err(TssError { result: result });
    }
    Ok(())
}
fn set_attrib_data_impl(object: &TssObject, attrib_flag: TssFlag, sub_flag: TssFlag, attrib_data: &[u8]) -> Result<(), TssError> {
    let result = unsafe {
        // TODO is usize to u32 cast safe?
        Tspi_SetAttribData(object.get_handle(), attrib_flag, sub_flag, attrib_data.len() as u32, attrib_data.as_ptr() as *mut u8)
    };
    if result != TSS_SUCCESS {
        return Err(TssError { result: result });
    }
    Ok(())
}

impl TssContext {
    pub fn new() -> Result<TssContext, TssError> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_Create(&mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssContext { handle: handle })
    }

    // TODO: support destination
    pub fn connect(&self) -> Result<(), TssError> {
        let result = unsafe {
            Tspi_Context_Connect(self.handle, 0 as *mut u16)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(())
    }

    pub fn get_tpm_object(&self) -> Result<TssTPM, TssError> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_GetTpmObject(self.handle, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssTPM { context: self, handle: handle })
    }

    pub fn load_key_by_uuid(&self, persistent_storage_type: TssFlag, uuid_data: TSS_UUID) -> Result<TssRsaKey, TssError> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_LoadKeyByUUID(self.handle, persistent_storage_type, uuid_data, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssRsaKey { context: self, handle: handle })
    }

    // TODO: DRY creating objects, probably use try!

    // TODO: make this signature shorter? give default values?
    pub fn create_rsakey(&self, key_size: TssKeySize, key_type: TssKeyType,
        auth: TssKeyAuthorization, volatile: TssKeyVolatility, migratable: TssKeyMigratability,
        struct_version: TssKeyStruct) -> Result<TssRsaKey, TssError> {
        let init_flags = key_size as u32 | key_type as u32 | auth as u32 | volatile as u32 | migratable as u32 | struct_version as u32;
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_RSAKEY, init_flags, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssRsaKey { context: self, handle: handle })
    }

    pub fn create_policy(&self, init_flag: TssPolicyInitFlag) -> Result<TssPolicy, TssError> {
        let init_flags = match init_flag {
            TssPolicyInitFlag::Usage => TSS_POLICY_USAGE,
            TssPolicyInitFlag::Migration => TSS_POLICY_MIGRATION,
            TssPolicyInitFlag::Operator => TSS_POLICY_OPERATOR
        };
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_POLICY, init_flags, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssPolicy { context: self, handle: handle })
    }

    pub fn create_pcr_composite_info(&self) -> Result<TssPCRCompositeInfo, TssError> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_PCRS, TSS_PCRS_STRUCT_INFO, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssPCRCompositeInfo { context: self, handle: handle })
    }
    pub fn create_pcr_composite_info_long(&self) -> Result<TssPCRCompositeInfoLong, TssError> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_PCRS, TSS_PCRS_STRUCT_INFO_LONG, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssPCRCompositeInfoLong { context: self, handle: handle })
    }
    pub fn create_pcr_composite_info_short(&self) -> Result<TssPCRCompositeInfoShort, TssError> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_PCRS, TSS_PCRS_STRUCT_INFO_SHORT, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(TssPCRCompositeInfoShort { context: self, handle: handle })
    }
}

impl Drop for TssContext {
    fn drop(&mut self) {
        unsafe {
            Tspi_Context_FreeMemory(self.handle, 0 as *mut u8);
            Tspi_Context_Close(self.handle);
        }
    }
}

impl<'context> TssTPM<'context> {
    // TODO: UNTESTED
    pub fn quote(&self, ident_key: &TssRsaKey, pcr_composite: &TssPCRCompositeInfo, external_data: &[u8; 20]) -> Result<TssValidation, TssError> {
        let mut validation_data = TSS_VALIDATION { versionInfo: TSS_VERSION { bMajor: 0, bMinor: 0, bRevMajor: 0, bRevMinor: 0 }, ulExternalDataLength: 20, rgbExternalData: external_data.as_ptr() as *mut u8, ulDataLength: 0, rgbData: 0 as *mut u8, ulValidationDataLength: 0, rgbValidationData: 0 as *mut u8 };
        let result = unsafe {
            Tspi_TPM_Quote(self.handle, ident_key.handle, pcr_composite.handle, &mut validation_data)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        let validation_result = TssValidation {
            version_info: validation_data.versionInfo.clone(),
            external_data: copy_raw_ptr_to_vec(external_data as *const u8, external_data.len()),
            data: copy_raw_ptr_to_vec(validation_data.rgbData, validation_data.ulDataLength as usize),
            validation_data: copy_raw_ptr_to_vec(validation_data.rgbValidationData, validation_data.ulValidationDataLength as usize)
        };
        unsafe {
            Tspi_Context_FreeMemory(self.context.handle, validation_data.rgbData);
            Tspi_Context_FreeMemory(self.context.handle, validation_data.rgbValidationData);
        }
        Ok(validation_result)
    }

    pub fn pcr_read(&self, pcr_index: u32) -> Result<Vec<u8>, TssError> {
        let mut pcr_value_length = 0;
        let mut pcr_value_ptr = 0 as *mut u8;
        let result = unsafe {
            Tspi_TPM_PcrRead(self.handle, pcr_index, &mut pcr_value_length, &mut pcr_value_ptr)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        let mut vec = Vec::new();
        unsafe {
            for i in 0..pcr_value_length {
                // TODO: Is this isize cast safe?
                vec.push(*pcr_value_ptr.offset(i as isize));
            }
            Tspi_Context_FreeMemory(self.context.handle, pcr_value_ptr);
        }
        Ok(vec)
    }

    // TODO: events
    pub fn pcr_extend(&self, pcr_index: u32, data: &[u8]) -> Result<Vec<u8>, TssError> {
        let mut pcr_value_length = 0;
        let mut pcr_value_ptr = 0 as *mut u8;
        let result = unsafe {
            // TODO: Is this u32 cast safe?
            Tspi_TPM_PcrExtend(self.handle, pcr_index, data.len() as u32, data.as_ptr() as *mut u8, 0 as *mut Struct_tdTSS_PCR_EVENT, &mut pcr_value_length, &mut pcr_value_ptr)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        // TODO: DRY with above
        let mut vec = Vec::new();
        unsafe {
            for i in 0..pcr_value_length {
                // TODO: Is this isize cast safe?
                vec.push(*pcr_value_ptr.offset(i as isize));
            }
            Tspi_Context_FreeMemory(self.context.handle, pcr_value_ptr);
        }
        Ok(vec)
    }

    pub fn pcr_reset(&self, pcr_composite: &TcpaPcrInfoAny) -> Result<(), TssError> {
        let result = unsafe {
            Tspi_TPM_PcrReset(self.handle, pcr_composite.get_handle())
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(())
    }
}

impl<'context> TssPolicy<'context> {
    //pub fn set_secret(mode: TssSecretMode, secret_length: u32, secret: &[u8]) -> Result<(), TssError> {
        // TODO
    //}
}

impl<'c> TssObject for TssRsaKey<'c> {
    fn get_handle(&self) -> TssHObject { self.handle }
    fn set_attrib_uint32(&self, attrib_flag: TssFlag, sub_flag: TssFlag, attrib: u32) -> Result<(), TssError> {
        set_attrib_uint32_impl(self, attrib_flag, sub_flag, attrib)
    }
    fn set_attrib_data(&self, attrib_flag: TssFlag, sub_flag: TssFlag, attrib_data: &[u8]) -> Result<(), TssError> {
        set_attrib_data_impl(self, attrib_flag, sub_flag, attrib_data)
    }
}

fn pcr_composite_select_pcr_index_ex(handle: TssHPCRS, pcr_index: u32, direction: u32) -> Result<(), TssError> {
    let result = unsafe {
        Tspi_PcrComposite_SelectPcrIndexEx(handle, pcr_index, direction)
    };
    if result != TSS_SUCCESS {
        return Err(TssError { result: result });
    }
    Ok(())
}

impl<'c> TssPCRCompositeInfo<'c> {
    fn select_pcr_index(&self, pcr_index: u32) -> Result<(), TssError> {
        let result = unsafe {
            Tspi_PcrComposite_SelectPcrIndex(self.handle, pcr_index)
        };
        if result != TSS_SUCCESS {
            return Err(TssError { result: result });
        }
        Ok(())
    }
}
impl<'c> TcpaPcrInfo1_2 for TssPCRCompositeInfoLong<'c> {
    fn get_handle(&self) -> u32 { self.handle }
    fn select_pcr_index_ex(&self, pcr_index: u32, direction: u32) -> Result<(), TssError> {
        pcr_composite_select_pcr_index_ex(self.handle, pcr_index, direction)
    }
}
impl<'c> TcpaPcrInfo1_2 for TssPCRCompositeInfoShort<'c> {
    fn get_handle(&self) -> u32 { self.handle }
    fn select_pcr_index_ex(&self, pcr_index: u32, direction: u32) -> Result<(), TssError> {
        pcr_composite_select_pcr_index_ex(self.handle, pcr_index, direction)
    }
}
// Need to implement Drop?
