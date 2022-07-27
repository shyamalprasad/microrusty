// The microbit's 25 element LED display is wired to the MCU using
// only 10 pins. This is done by organizing the display in a matrix of
// 5 rows and 5 columns (the older v1 board is different!).

// See the Microbit v2 schematic for details.

// The key thing is that the LED anodes are wired to 5 row lines, each
// row being tied to one GPIO pin. Each LED cathode is wired to a
// column line, and each column line is wired to a GPIO pin through a
// 105 ohm resistor.

// To light up an LED it's row must be set high, and the colum set
// low.  To display an arbitrary pattern on this grid, software must
// drive the row and column lines to sequence so that on only those
// LEDs that should be visible are turned on, and this must be at a
// frequency high enough that the human eye does not notice flicker.



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

// Register offsets for each GPIO register set
const REG_DIR: u32 = 0x518;
const REG_SET: u32 = 0x508;
const REG_CLR: u32 = 0x50c;
// PIN_CNF[n] is at offset 0x700 + (n * 0x4), LSB to 1 of output

// Bit masks for the GPIO pins to each row (all on P0)
const P0_ROW_1: u32 = 0x1 << 21;
const P0_ROW_2: u32 = 0x1 << 22;
const P0_ROW_3: u32 = 0x1 << 15;
const P0_ROW_4: u32 = 0x1 << 24;
const P0_ROW_5: u32 = 0x1 << 19;

// Bit mask for the GPIO pins to each column (all on P0, except column 4)
const P0_COL_1: u32 = 0x1 << 28; // Also wired to AIN4
const P0_COL_2: u32 = 0x1 << 11; // Also wired to TRACEDATA2
const P0_COL_3: u32 = 0x1 << 31; // Also wired to AIN7
const P1_COL_4: u32 = 0x1 << 5;
const P0_COL_5: u32 = 0x1 << 30; // Also wired to AIN4

// Convenient sets of GPIO pins
const P0_ROW_PINS: u32 = P0_ROW_1 | P0_ROW_2 | P0_ROW_3 | P0_ROW_4 | P0_ROW_5;
const P0_COL_PINS: u32 = P0_COL_1 | P0_COL_2 | P0_COL_3 | P0_COL_5;
const P1_COL_PINS: u32 = P1_COL_4;
const P0_PINS: u32 = P0_ROW_PINS | P0_COL_PINS; //0xd1688800;
const P1_PINS: u32 = P1_COL_PINS; // 0x20

// Sets the GPIO pins to the matrix to be output lines
// and leaves all pins low (0).
pub fn init_led_matrix() {
    // Set DIR register for outputs on the LED matrix
    unsafe {
	// Set all matrix row/col GPIO lines to output
	*((P0 + REG_DIR) as *mut u32) = P0_PINS;
	*((P1 + REG_DIR) as *mut u32) = P1_PINS;

	*((P0 + REG_CLR) as *mut u32) = P0_PINS;
	*((P1 + REG_CLR) as *mut u32) = P1_PINS;
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
	*p1_set = P1_PINS;
	*p0_set = P0_PINS;

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
