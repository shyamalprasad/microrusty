#![no_main]
#![no_std]

mod init;
mod led;
use core::panic::PanicInfo;

static HELLO: &str = "hello, world"; /* goes into .rodata */
static mut MZERO: u8 = 0; /* goes into .bss */
static mut MONE: u16 = 4; /* goes into .data */
static mut MTWO: u8 = 1;
static mut MTHREE: u16 = 8;

entry!(main);
pub fn main() -> ! {
    let _x = unsafe { &MZERO };
    let _y = &HELLO;
    let _z1 = unsafe { &MONE };
    let _z2 = unsafe { &MTWO };
    let _z3 = unsafe { &MTHREE };
    led::init_led_matrix();
    loop {
        led::flash();
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    led::all_on();
    loop {}
}
