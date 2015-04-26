use std::ptr;

pub type TssHContext = u32;
pub type TssHTPM = u32;
pub type TssResult = u32;
pub type TssUnicode = u16;

pub const TSS_SUCCESS: TssResult = 0;

pub struct TssContext {
    pub handle: u32
}
pub struct TssTPM<'context> {
    pub context: &'context TssContext,
    pub handle: u32
}

#[link(name = "tspi")]
extern {
    fn Tspi_Context_Create(phContext: *mut TssHContext) -> TssResult;
    fn Tspi_Context_Connect(phContext: TssHContext, wszDestination: *const TssUnicode) -> TssResult;
    fn Tspi_Context_GetTpmObject(phContext: TssHContext, phTPM: *mut TssHTPM) -> TssResult;
    fn Tspi_Context_FreeMemory(phContext: TssHContext, rgbMemory: *mut u8) -> TssResult;
    fn Tspi_Context_Close(phContext: TssHContext) -> TssResult;
    fn Tspi_TPM_PcrRead(hTPM: TssHTPM, ulPcrIndex: u32, pulPcrValueLength: *mut u32, prgbPcrValue: *mut *mut u8) -> TssResult;
    fn Tspi_TPM_PcrExtend(hTPM: TssHTPM, ulPcrIndex: u32, ulPcrDataLength: u32, pbPcrData: *const u8, pPcrEvent: *mut u8, pulPcrValueLength: *mut u32, prgbPcrValue: *mut *mut u8) -> TssResult;
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
}
