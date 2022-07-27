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

// The address of the GPIO register sets
const P0: u32 = 0x50000000;
const P1: u32 = 0x50000300;

// Register offsets
const REG_DIR: u32 = 0x518;
const REG_SET: u32 = 0x508;
const REG_CLR: u32 = 0x50c;

// Bit masks
const P0_ROW_1: u32 = 0x1 << 21;
const P0_ROW_2: u32 = 0x1 << 22;
const P0_ROW_3: u32 = 0x1 << 15;
const P0_ROW_4: u32 = 0x1 << 24;
const P0_ROW_5: u32 = 0x1 << 19;

const P0_COL_1: u32 = 0x1 << 28;
const P0_COL_2: u32 = 0x1 << 11;
const P0_COL_3: u32 = 0x1 << 31;
const P1_COL_4: u32 = 0x1 << 5;
const P0_COL_5: u32 = 0x1 << 30;

const P0_ROW_MASK: u32 = P0_ROW_1 | P0_ROW_2 | P0_ROW_3 | P0_ROW_4 | P0_ROW_5;
const P0_COL_MASK: u32 = P0_COL_1 | P0_COL_2 | P0_COL_3 | P0_COL_5;
const P1_COL_MASK: u32 = P1_COL_4;

const P0_MASK: u32 = P0_ROW_MASK | P0_COL_MASK; //0xd1688800;
const P1_MASK: u32 = P1_COL_MASK; // 0x20

pub fn init_led_matrix() {
    // Set DIR register for outputs on the LED matrix
    unsafe {
        // Set all matrix row/col GPIO lines to output
        *((P0 + REG_DIR) as *mut u32) = P0_MASK;
        *((P1 + REG_DIR) as *mut u32) = P1_MASK;

        *((P0 + REG_CLR) as *mut u32) = P0_MASK;
        *((P1 + REG_CLR) as *mut u32) = P1_MASK;
    }
    // Leave PIN_CNF unchanged because the only bit we need to set
    // right now is the output direction, which is the same as DIR
}

pub fn flash() {
    let p0_set = (P0 + REG_SET) as *mut u32;
    let p0_clr = (P0 + REG_CLR) as *mut u32;
    unsafe {
        // Turn on each row in sequence
        *p0_set = P0_ROW_1;
        *p0_clr = P0_ROW_1;

        *p0_set = P0_ROW_2;
        *p0_clr = P0_ROW_2;

        *p0_set = P0_ROW_3;
        *p0_clr = P0_ROW_3;

        *p0_set = P0_ROW_4;
        *p0_clr = P0_ROW_4;

        *p0_set = P0_ROW_5;
        *p0_clr = P0_ROW_5;
    }

    // Now turn on each column in sequence
    let p1_set = (P1 + REG_SET) as *mut u32;
    let p1_clr = (P1 + REG_CLR) as *mut u32;
    unsafe {
        // Set all matrix lines high
        *p1_set = P1_MASK;
        *p0_set = P0_MASK;

        *p0_clr = P0_COL_1;
        *p0_set = P0_COL_1;

        *p0_clr = P0_COL_2;
        *p0_set = P0_COL_2;

        *p0_clr = P0_COL_3;
        *p0_set = P0_COL_3;

        *p1_clr = P1_COL_4;
        *p1_set = P1_COL_4;

        *p0_clr = P0_COL_5;
        *p0_set = P0_COL_5;
    }
}
