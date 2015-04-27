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

pub enum TssPcrsStructType {
    Default, Info, InfoLong, InfoShort
}

pub struct TssContext {
    pub handle: u32
}
pub struct TssTPM<'context> {
    pub context: &'context TssContext,
    pub handle: u32
}
pub struct TssPCRComposite<'context> {
    pub context: &'context TssContext,
    pub handle: u32
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

    // TODO: Should selecting an index be tied into actually creating the PCRS object?
    pub fn create_pcr_composite(&self, pcrs_type: TssPcrsStructType) -> Result<TssPCRComposite, TssResult> {
        let mut handle = 0;
        // Do it as a <> instead? Would mean we can actually make functions taking *only* PCR
        // composite objects that are supported by the function!
        // e.g. Tspi_PcrComposite_SelectPcrIndex only supports TCPA_PCR_INFO structures
        // Maybe do 1.2 structures as an enum...
        let init_flags = match pcrs_type {
            TssPcrsStructType::Default => TSS_PCRS_STRUCT_DEFAULT,
            TssPcrsStructType::Info => TSS_PCRS_STRUCT_INFO,
            TssPcrsStructType::InfoLong => TSS_PCRS_STRUCT_INFO_LONG,
            TssPcrsStructType::InfoShort => TSS_PCRS_STRUCT_INFO_SHORT
        };
        let result = unsafe {
            Tspi_Context_CreateObject(self.handle, TSS_OBJECT_TYPE_PCRS, init_flags, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(TssPCRComposite { context: self, handle: handle })
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

    pub fn pcr_reset(&self, pcr_composite: TssPCRComposite) -> Result<(), TssResult> {
        let result = unsafe {
            Tspi_TPM_PcrReset(self.handle, pcr_composite.handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(())
    }
}

impl<'context> TssPCRComposite<'context> {
}
