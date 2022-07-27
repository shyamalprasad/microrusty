// P0 Base: 0x50000000

// P1 Base: 0x50000300

// OUT, OUTSET, OUTCLR at offset  0x504, 0x508, 0x50c
// DIR is at offset 0x514, 1 is output, 0 is input
// DIRSET, DIRCLR are at 0x518 and 0x51C

// PIN_CNF[n] is at offset 0x700 + (n * 0x4), LSB to 1 of output

// Col 1 P0.28 AIN4
// Col 2 P0.11 TRACEDATA2
// Col 3 P0.31 AIN7
// Col 4 P1.05 -
// Col 5 P0.30 AIN4

// Row 1 P0.21
// Row 2 P0.22
// Row 3 P0.15
// Row 4 P0.24
// Row 5 P0.19


pub fn init_led_matrix() {
    // Allows rows and columns are on P0, except col 4 is on P1.05
    const P0: u32 = 0x50000000;
    const P1: u32 = 0x50000300;

    // Set DIR register for outputs on the LED matrix
    unsafe {
	let p0_dir_set  = (P0 + 0x518) as *mut u32;
	let p1_dir_set = (P1 + 0x518) as *mut u32;
	// P0 leading bits  (31, 30, 28), (24), (22, 21), (19), (15), (11)
	*p0_dir_set = 0xd1688800;
	// P1 bits: (5)
	*p1_dir_set = 0x00000020;
    }
    // Leave PIN_CNF unchanged because the only bit we need to set
    // right now is the output direction, which is the same as DIR
}
