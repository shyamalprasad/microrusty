//
// The init module initializes the Microbit
//

// The function run on board reset
// It will eventually call a main() method which should be a Rust
// function identified using the entry!() macro defined below
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

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
    }
}
