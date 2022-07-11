//
// The init module initializes the Microbit
//

// The function run on board reset
// It will eventually call a main() method which should be a Rust
// function identified using the entry!() macro defined below
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        // BSS and .data are work aligned - see link.x

        static mut _sbss: u32; // BSS start
        static mut _ebss: u32; // BSS end

        // Static data must be copied from ROM to ROM
        static mut _erodata: u32; // LMA of .data in ROM
        static mut _sdata: u32; // VMA of .data in RAM
        static mut _edata: u32; // VMA of the end of .data in RAM
    }

    // *You* should use core::ptr::write_bytes, I'm just learning Rust!
    let mut bss_ptr = &mut _sbss as *mut u32;
    while bss_ptr < &mut _ebss as *mut u32 {
        *bss_ptr = 0;
        bss_ptr = bss_ptr.add(1);
    }

    // *You* should use core::ptr::copy_nonoverlapping, I'm just learning Rust
    let mut data_src_ptr = &_erodata as *const u32;
    let mut data_dst_ptr = &mut _sdata as *mut u32;
    while data_dst_ptr < &mut _edata as *mut u32 {
        *data_dst_ptr = *data_src_ptr;
        data_dst_ptr = data_dst_ptr.add(1);
        data_src_ptr = data_src_ptr.add(1);
    }

    // The above loops don't compile down to what a C programmer expects:
    // - The pointers are kept on the stack, not in registers
    // - In --release mode we seem to get vectorized, and we pull in several
    //   hundred bytes of intrinsics anyway, so I guess we should just use those
    //   in the first place!
    // - It still sucks that I don't know how to write code like this to be really
    //   tight just yet....

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
