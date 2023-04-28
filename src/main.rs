#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// the no_mangle attribute ensures the rust compiler does not change the name of
// the _start function
#[no_mangle]
// this function is marked with "extern C" since it uses the C convention for
// calling a function. This creates an FFI, or Foreign Function Interface
// Reference: https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
//
// the "!" return type means it has a divergent return type, or it never
// divergent return type, or it never only invoked directly "since there’s
// nothing left to do if a freestanding binary returns"
pub extern "C" fn _start() -> ! {
	use core::fmt::Write;
	vga_buffer::WRITER
		// block other threads from accessing this data
		.lock()
		.write_str("Hello again")
		.unwrap();

	write!(
		vga_buffer::WRITER.lock(),
		", some numbers: {} {}",
		42,
		1.337
	)
	.unwrap();

	#[allow(clippy::empty_loop)]
	loop {}
}

/// this is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
