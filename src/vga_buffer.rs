use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
	/// A global instance of `writer` to write to the VGA Buffer
	/// The mutex allows for synchronous access to the data within to prevent data
	/// races
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Yellow, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// store each variant as a u8
// Does this mean the enum can only have 255 discriminants or the fields' values
// can only go up to 255???
//
// https://doc.rust-lang.org/nomicon/other-reprs.html#repru-repri
// the effect is similar to the effect of repr(C) in that there is a defined
// layout of the type. This makes it possible to pass the enum to C code, or
// access the type's raw representation and directly manipulate its tag and
// fields
// https://doc.rust-lang.org/reference/type-layout.html#primitive-representation-of-enums-with-fields
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// similar to the `repr` trait above—defines the layout of the struct as a `u8` in
// this case
// https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
// https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation
// is it the same as using the `repr(u8)` trait though??
#[repr(transparent)]
// tuple struct
struct ColorCode(u8);

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// orders the struct exactly like in C
// https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
/// A buffer of chars to write to
///
/// `Volatile` is used guaruntee that the compiler reads and writes as we'd expect
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	// the static lifetime annotation means the reference is valid for the whole
	// program's runtime
	buffer: &'static mut Buffer,
}

impl Writer {
	/// writes ascii bytes to the buffer and moves to the next line if current is
	/// full
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;

				let color_code = self.color_code;
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					color_code,
				});
				self.column_position += 1;
			}
		}
	}

	/// writes everything in the text buffer to the screen
	/// skips the first row then hard wraps each line at `BUFFER_WIDTH` and clears
	/// the `BUFFER_HEIGHT`-1th line
	fn new_line(&mut self) {
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	/// clears rows by filling them with spaces
	fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code: self.color_code,
		};
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}

	/// writes an entire string of ascii characters to th buffer
	/// wraps when lines become full
	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// not part of printable ASCII range
				_ => self.write_byte(0xfe),
			}
		}
	}
}

impl fmt::Write for Writer {
	/// `write_string` does not return anything meaningful, so `Ok(())` is returned
	/// on success, `Ok(())` is returned, and on failure, Err is ruteurned
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

// this trait adds teh macro to the root of the crate
/// print text to the vga text buffer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

/// prints text postfixed with a newline character to the vga text buffer
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// lock `WRITER` and write to it
// doc hidden hides from generated docs
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}
