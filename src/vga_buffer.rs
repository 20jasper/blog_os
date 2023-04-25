#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// store each variant as a u8
// https://doc.rust-lang.org/nomicon/other-reprs.html#repru-repri
// the effect is similar to the effect of repr(C) in that there is a defined
// layout of the type. This makes it possible to pass the enum to C code, or
// access the type's raw representation and directly manipulate its tag and
// fields
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
