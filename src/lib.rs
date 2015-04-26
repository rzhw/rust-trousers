use std::ptr;

pub type TssHContext = u32;
pub type TssHTPM = u32;
pub type TssResult = u32;
pub type TssUnicode = u16;

pub const TSS_SUCCESS: TssResult = 0;

pub struct TssContext {
    pub handle: u32
}
pub struct TssTpm<'context> {
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

    pub fn get_tpm_object(&self) -> Result<TssTpm, TssResult> {
        let mut handle = 0;
        let result = unsafe {
            Tspi_Context_GetTpmObject(self.handle, &mut handle)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        Ok(TssTpm { context: self, handle: handle })
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

impl<'context> TssTpm<'context> {
    pub fn pcr_read(&self, pcr_index: u32) -> Result<Vec<u8>, TssResult> {
        let mut ulPcrValueLength = -1;
        let mut pRgbPcrValue = 0 as *mut u8;
        let result = unsafe {
            Tspi_TPM_PcrRead(self.handle, pcr_index, &mut ulPcrValueLength, &mut pRgbPcrValue)
        };
        if result != TSS_SUCCESS {
            return Err(result);
        }
        let mut vec = Vec::new();
        unsafe {
            for i in 0..ulPcrValueLength {
                vec.push(*pRgbPcrValue.offset(i as isize));
            }
            Tspi_Context_FreeMemory(self.context.handle, pRgbPcrValue);
        }
        Ok(vec)
    }
}
