use std::ptr;

struct TssContext { handle: u32 }
struct TssTpm<'context> { context: &'context TssContext, handle: u32 }

type TssHContext = u32;
type TssHTPM = u32;
type TssResult = u32;
type TssUnicode = u16;

const TSS_SUCCESS: TssResult = 0;

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
    fn new() -> Result<TssContext, TssResult> {
        unsafe {
            let mut handle = 0;
            let result = Tspi_Context_Create(&mut handle);
            if result != TSS_SUCCESS {
                Err(result)
            } else {
                Ok(TssContext { handle: handle })
            }
        }
    }

    // TODO: support destination
    fn connect(&self) -> Result<(), TssResult> {
        unsafe {
            let result = Tspi_Context_Connect(self.handle, ptr::null());
            if result != TSS_SUCCESS {
                Err(result)
            } else {
                Ok(())
            }
        }
    }

    fn get_tpm_object(&self) -> Result<TssTpm, TssResult> {
        unsafe {
            let mut handle = 0;
            let result = Tspi_Context_GetTpmObject(self.handle, &mut handle);
            if result != TSS_SUCCESS {
                Err(result)
            } else {
                Ok(TssTpm { context: self, handle: handle })
            }
        }
    }
}

impl Drop for TssContext {
    fn drop(&mut self) {
        unsafe {
            Tspi_Context_Close(self.handle);
        }
    }
}

impl<'context> TssTpm<'context> {
    fn pcr_read(&self, pcr_index: u32) -> Result<Vec<u8>, TssResult> {
        unsafe {
            let mut ulPcrValueLength = -1;
            let mut pRgbPcrValue = 0 as *mut u8;
            let result = Tspi_TPM_PcrRead(self.handle, pcr_index, &mut ulPcrValueLength, &mut pRgbPcrValue);
            if result != TSS_SUCCESS {
                Err(result)
            } else {
                let mut vec = Vec::new();
                for i in 0..ulPcrValueLength {
                    vec.push(*pRgbPcrValue.offset(i as isize));
                }
                Tspi_Context_FreeMemory(self.context.handle, pRgbPcrValue);
                Ok(vec)
            }
        }
    }
}

fn main() {
    // TODO: Any cleaner way to write this?
    let contextresult = TssContext::new();
    if let Ok(context) = contextresult {
        if let Ok(_) = context.connect() {
            if let Ok(tpm) = context.get_tpm_object() {
                println!("I'M IN UR TPM READING UR PCRZ (From Rust!)");
                println!("TPM handle: {:?}", tpm.handle);
                for i in 0..24 {
                    if let Ok(vec) = tpm.pcr_read(i) {
                        print!("PCR {:02}", i);
                        for j in 0..19 {
                            print!(" {:02X}", vec[j]);
                        }
                        print!("\n");
                    }
                }
            } else {
                println!("Failed to get TPM handle :(")
            }
        } else {
            println!("Failed to connect :(");
        }
    } else {
        println!("Failed :(");
    }

    /*
    match blah {
        Ok(context) => println!("Context created! {:?}", context.handle),
        Err(e) => println!("Context failed with err {:?}", e),
    }
    println!("Hello world!");
    */
}
