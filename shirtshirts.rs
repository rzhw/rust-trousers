use std::ptr;

struct TssContext { handle: u32 }
struct TssTpm { handle: u32 }

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
    fn Tspi_Context_Close(phContext: TssHContext) -> TssResult;
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
                Ok(TssTpm { handle: handle })
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

fn main() {
    // TODO: Any cleaner way to write this?
    let contextresult = TssContext::new();
    if let Ok(context) = contextresult {
        if let Ok(_) = context.connect() {
            if let Ok(tpm) = context.get_tpm_object() {
                println!("Success! TPM handle: {:?}", tpm.handle);
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
