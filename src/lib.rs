use std::ptr;

pub type TssFlag = u32;
pub type TssHObject = u32;
pub type TssHContext = TssHObject;
pub type TssHTPM = TssHObject;
pub type TssHPCRS = TssHObject;
pub type TssResult = u32;
pub type TssUnicode = u16;

pub const TSS_SUCCESS: TssResult = 0;

const TSS_OBJECT_TYPE_PCRS: TssFlag = 4;

const TSS_PCRS_STRUCT_DEFAULT: TssFlag = 0;
const TSS_PCRS_STRUCT_INFO: TssFlag = 1;
const TSS_PCRS_STRUCT_INFO_LONG: TssFlag = 2;
const TSS_PCRS_STRUCT_INFO_SHORT: TssFlag = 3;

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

pub struct TssPCRCompositeInfo<'context> {
    pub context: &'context TssContext,
    pub handle: u32
}
pub struct TssPCRCompositeInfoLong<'context> {
    pub context: &'context TssContext,
    pub handle: u32
}
pub struct TssPCRCompositeInfoShort<'context> {
    pub context: &'context TssContext,
    pub handle: u32
}
pub trait TcpaPcrInfoAny {
    fn get_handle(&self) -> u32;
}
pub trait TcpaPcrInfo1_1 : TcpaPcrInfoAny {
    fn get_handle(&self) -> u32;
}
pub trait TcpaPcrInfo1_2 : TcpaPcrInfoAny {
    fn get_handle(&self) -> u32;
}
impl<'c> TcpaPcrInfoAny for TssPCRCompositeInfo<'c> {
    fn get_handle(&self) -> u32 { self.handle }
}
impl<'c> TcpaPcrInfo1_1 for TssPCRCompositeInfo<'c> {
    fn get_handle(&self) -> u32 { self.handle }
}
impl<'c> TcpaPcrInfoAny for TssPCRCompositeInfoLong<'c> {
    fn get_handle(&self) -> u32 { self.handle }
}
impl<'c> TcpaPcrInfo1_2 for TssPCRCompositeInfoLong<'c> {
    fn get_handle(&self) -> u32 { self.handle }
}
impl<'c> TcpaPcrInfoAny for TssPCRCompositeInfoShort<'c> {
    fn get_handle(&self) -> u32 { self.handle }
}
impl<'c> TcpaPcrInfo1_2 for TssPCRCompositeInfoShort<'c> {
    fn get_handle(&self) -> u32 { self.handle }
}

#[link(name = "tspi")]
extern {
    fn Tspi_Context_Create(phContext: *mut TssHContext) -> TssResult;
    fn Tspi_Context_Close(hContext: TssHContext) -> TssResult;
    fn Tspi_Context_Connect(hContext: TssHContext, wszDestination: *const TssUnicode) -> TssResult;
    fn Tspi_Context_FreeMemory(hContext: TssHContext, rgbMemory: *mut u8) -> TssResult;
    fn Tspi_Context_CreateObject(hContext: TssHContext, objectType: TssFlag, initFlags: TssFlag, phObject: *mut TssHObject) -> TssResult;
    fn Tspi_Context_GetTpmObject(hContext: TssHContext, phTPM: *mut TssHTPM) -> TssResult;
    fn Tspi_TPM_PcrRead(hTPM: TssHTPM, ulPcrIndex: u32, pulPcrValueLength: *mut u32, prgbPcrValue: *mut *mut u8) -> TssResult;
    fn Tspi_TPM_PcrExtend(hTPM: TssHTPM, ulPcrIndex: u32, ulPcrDataLength: u32, pbPcrData: *const u8, pPcrEvent: *mut u8, pulPcrValueLength: *mut u32, prgbPcrValue: *mut *mut u8) -> TssResult;
    fn Tspi_TPM_PcrReset(hTPM: TssHTPM, hPcrComposite: TssHPCRS) -> TssResult;
}

impl TssContext {
    pub fn new() -> Result<TssContext, TssResult> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_Create(&mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(TssContext { handle: handle })
    }

    // TODO: support destination
    pub fn connect(&self) -> Result<(), TssResult> {
        let result = unsafe {
            Tspi_Context_Connect(self.handle, ptr::null())
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(())
    }

    pub fn get_tpm_object(&self) -> Result<TssTPM, TssResult> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_GetTpmObject(self.handle, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(TssTPM { context: self, handle: handle })
    }

    pub fn create_pcr_composite_info(&self) -> Result<TssPCRCompositeInfo, TssResult> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_PCRS, TSS_PCRS_STRUCT_INFO, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(TssPCRCompositeInfo { context: self, handle: handle })
    }
    pub fn create_pcr_composite_info_long(&self) -> Result<TssPCRCompositeInfoLong, TssResult> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_PCRS, TSS_PCRS_STRUCT_INFO_LONG, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(TssPCRCompositeInfoLong { context: self, handle: handle })
    }
    pub fn create_pcr_composite_info_short(&self) -> Result<TssPCRCompositeInfoShort, TssResult> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_PCRS, TSS_PCRS_STRUCT_INFO_SHORT, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
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
    pub fn pcr_read(&self, pcr_index: u32) -> Result<Vec<u8>, TssResult> {
        let mut pcr_value_length = 0;
        let mut pcr_value_ptr = 0 as *mut u8;
        let result = unsafe {
            Tspi_TPM_PcrRead(self.handle, pcr_index, &mut pcr_value_length, &mut pcr_value_ptr)
        };
        if result != TSS_SUCCESS {
            return Err(result);
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
    pub fn pcr_extend(&self, pcr_index: u32, data: &[u8]) -> Result<Vec<u8>, TssResult> {
        let mut pcr_value_length = 0;
        let mut pcr_value_ptr = 0 as *mut u8;
        let result = unsafe {
            // TODO: Is this u32 cast safe?
            Tspi_TPM_PcrExtend(self.handle, pcr_index, data.len() as u32, data.as_ptr(), 0 as *mut u8, &mut pcr_value_length, &mut pcr_value_ptr)
        };
        if result != TSS_SUCCESS {
            return Err(result);
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

    pub fn pcr_reset(&self, pcr_composite: &TcpaPcrInfoAny) -> Result<(), TssResult> {
        let result = unsafe {
            Tspi_TPM_PcrReset(self.handle, pcr_composite.get_handle())
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(())
    }
}
