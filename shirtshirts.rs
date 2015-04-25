struct TssContext { handle: u32 }
struct TssTpm { handle: u32 }

type TssHContext = u32;
type TssResult = u32;

const TSS_SUCCESS: TssResult = 0;

#[link(name = "tspi")]
extern {
    fn Tspi_Context_Create(phContext: *mut TssHContext) -> TssResult;
    fn Tspi_Context_Close(phContext: TssHContext) -> TssResult;
}

impl TssContext {
    fn new() -> Result<TssContext, TssResult> {
        unsafe {
            let mut handle = 0;
            let result = Tspi_Context_Create(&mut handle);
            if result == TSS_SUCCESS {
                Ok(TssContext { handle: handle })
            } else {
                Err(result)
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
    let blah = TssContext::new();
    match blah {
        Ok(context) => println!("Context created! {:?}", context.handle),
        Err(e) => println!("Context failed with err {:?}", e),
    }
    println!("Hello world!");
}
