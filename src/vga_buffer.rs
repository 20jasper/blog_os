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
// similar to the repr trait above—defines the layout of the struct as a u8 in
// this case
// https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
// https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation
// is it the same as using the repr(u8) trait though??
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

#[repr(transparent)]
struct Buffer {
	chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
