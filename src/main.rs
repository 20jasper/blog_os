#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";
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
	// cast vga_buffer as a raw pointer, which aren't automatically cleaned up,
	// can be null, and aren't guarunteed to point to valid memory
	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			// the dereference operator (*) is needed to get the value from the memory
			// address so we can change it to the bytes in our message

			*vga_buffer.offset(i as isize * 2) = byte;
			// write color byte
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}

	#[allow(clippy::empty_loop)]
	loop {}
}

/// this is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
