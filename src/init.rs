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
