//
// The init module initializes the Microbit
//

use core::ptr;

// The function run on board reset
// It will eventually call a main() method which should be a Rust
// function identified using the entry!() macro defined below
#[no_mangle]
unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        // BSS and .data are work aligned - see link.x

        static mut _sbss: u8; // BSS start
        static mut _ebss: u8; // BSS end

        // Static data must be copied from ROM to ROM
        static mut _erodata: u8; // LMA of .data in ROM
        static mut _sdata: u8; // VMA of .data in RAM
        static mut _edata: u8; // VMA of the end of .data in RAM
    }

    // A prior attempt to write explicit loops for these function calls
    // to produce minimal machine instructions for the clear/copy below
    // did not seem to work. In fact, the compiler optimized them to
    // vectorized code. Which is cute, but in this case I do want two
    // really simple loops, right? Not a few hundered bytes of library
    // code.....so, yeah, I don't know "how to write C code" in Rust as
    // yet!
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_erodata as *const u8, &mut _sdata as *mut u8, count);

    extern "Rust" {
        fn main() -> !;
    }
    main()
}

// See link.x - this sets the reset vector in flash memory (address 0x4)
// to the Reset() function above
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

// A Macro to make sure that any main function we define never returns
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

// The default exception handler set up on initialization
// It should not be used anywhere else!
#[no_mangle]
extern "C" fn UnhandledException() {
    loop {}
}

// The 8 standard Cortex-M exception handlers
// If these functions are not defined, the linker script will set
// them to UnhandledException
extern "C" {
    fn NMI();
    fn HardFault();
    fn MMFault();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

// A union type to set a reference to an exception handler
// or to set a 32 bit word to zero (or, really, any u32 value).
pub union ExceptionHandler {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

// Cortex-M defines 14 standard exceptions to follow the Reset Vector
// Note that the Reset vector must never return, while exception
// handlers normally do so, which is why the Reset vector is note
// initialized as an ExceptionHandler
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [ExceptionHandler; 14] = [
    ExceptionHandler { handler: NMI },
    ExceptionHandler { handler: HardFault },
    ExceptionHandler { handler: MMFault },
    ExceptionHandler { handler: BusFault },
    ExceptionHandler {
        handler: UsageFault,
    },
    ExceptionHandler { reserved: 0 },
    ExceptionHandler { reserved: 0 },
    ExceptionHandler { reserved: 0 },
    ExceptionHandler { reserved: 0 },
    ExceptionHandler { handler: SVCall },
    ExceptionHandler { reserved: 0 },
    ExceptionHandler { reserved: 0 },
    ExceptionHandler { handler: PendSV },
    ExceptionHandler { handler: SysTick },
];
