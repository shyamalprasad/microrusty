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

// Because all rows and 4 columns are controlled via the P0 GPIO
// lines, but only Column 4 is controlled via a P1 GPIO line, we will
// multiplex the display by column. On each refresh cycle, we will
// illuminate only the enabled LEDS in a single column, and turn off
// all the other columns.

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

        all_off();
    }
    // Leave PIN_CNF unchanged because the only bit we need to set
    // right now is the output direction, which is the same as DIR
}

// All rows are the P0 GPIO register set
fn get_row_gpio_bit(row: usize) -> u32 {
    match row {
        0 => P0_ROW_1,
        1 => P0_ROW_2,
        2 => P0_ROW_3,
        3 => P0_ROW_4,
        4 => P0_ROW_5,
        _ => 0,
    }
}

// Columns 1,2,3, and 5 are on the P0 GPIO register set, Column 4 is on P1
// Fortunately (?) the bits for each are completely exclusive.....
fn get_col_gpio_bit(col: usize) -> u32 {
    match col {
        0 => P0_COL_1,
        1 => P0_COL_2,
        2 => P0_COL_3,
        3 => P1_COL_4, // Caller must know this applies to P1!
        4 => P0_COL_5,
        _ => 0,
    }
}

// Turn off every LED in the matrix
pub fn all_off() {
    unsafe {
        // Set all pins high to disable the entire LED matrix
        *((P1 + REG_CLR) as *mut u32) = P1_PINS;
        *((P0 + REG_CLR) as *mut u32) = P0_PINS;
    }
}

// Turn on every LED in the matrix
pub fn all_on() {
    unsafe {
        // Set row pins high, column pins low
        *((P0 + REG_SET) as *mut u32) = P0_ROW_PINS;
        *((P0 + REG_CLR) as *mut u32) = P0_COL_PINS;
        *((P1 + REG_CLR) as *mut u32) = P1_PINS;
    }
}

// We store bit maps for the LED values in each column. A bit
// corresponding to the LED row in the P0 GPIO register is set to 1 if
// the LED is to be lit, and is set to 0 if it suppposed to
// extinguished.
pub struct Columns {
    cols: [u32; 5], // Set P0_ROW_{N} bit to turn on LED in column
}

impl Columns {
    pub fn clear_all(&mut self) {
        for i in 0..5 {
            self.clear_col(i);
        }
    }
    pub fn clear_col(&mut self, col: usize) {
        if col < 5 {
            self.cols[col] = 0;
        }
    }
    pub fn clear(&mut self, row: usize, col: usize) {
        if col < 5 && row < 5 {
            self.cols[col] = self.cols[col] & !get_row_gpio_bit(row);
        }
    }
    pub fn set_all(&mut self) {
        for i in 0..5 {
            self.set_col(i);
        }
    }
    pub fn set_col(&mut self, col: usize) {
        if col < 5 {
            self.cols[col] = P0_ROW_PINS;
        }
    }
    pub fn set(&mut self, row: usize, col: usize) {
        if col < 5 && row < 5 {
            self.cols[col] = self.cols[col] | get_row_gpio_bit(row);
        }
    }
    fn get_column(&self, col: usize) -> u32 {
        if col < 5 {
            self.cols[col]
        } else {
            0
        }
    }
    pub fn get(&self, col: usize, row: usize) -> bool {
        if col > 4 || row > 4 {
            return false;
        }
        self.cols[col] & get_row_gpio_bit(row) != 0
    }
}

fn show_column(columns: &Columns, cnum: usize) {
    let row_mask = columns.get_column(cnum);
    let col_mask = get_col_gpio_bit(cnum);

    unsafe {
        // Set enabled LED lines high
        *((P0 + REG_SET) as *mut u32) = row_mask;
        *((P0 + REG_CLR) as *mut u32) = P0_ROW_PINS & !row_mask;

        // Set only the enabled column low, others high
        if col_mask == P1_COL_4 {
            // Col 4 low, everything else high
            *((P1 + REG_CLR) as *mut u32) = col_mask;
            *((P0 + REG_SET) as *mut u32) = P0_COL_PINS;
        } else {
            // Enabled column low, everything else high
            *((P0 + REG_SET) as *mut u32) = P0_COL_PINS;
            *((P1 + REG_SET) as *mut u32) = P1_COL_PINS;
            *((P0 + REG_CLR) as *mut u32) = col_mask;
        }
    }
}

pub fn flash() {
    // Set alternate LED on
    let col_array: [u32; 5] = [0, 0, 0, 0, 0];
    let mut columns = Columns { cols: col_array };
    columns.clear_all();
    for i in 0..25 {
        if i % 2 == 0 {
            columns.set(i / 5, i % 5);
        }
    }
    for i in 0..5 {
        show_column(&columns, i);
    }
    all_off(); // turn off matrix to avoid last column being overly bright!
}
