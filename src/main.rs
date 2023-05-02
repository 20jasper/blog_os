#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

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
	println!("hello");
	println!("{}", 4444);

	#[cfg(test)]
	test_main();

	#[allow(clippy::empty_loop)]
	loop {}
}

/// this is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{info}");
	loop {}
}

/// runs all tests with the `#[test]` attribute
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
}

#[test_case]
fn trivial_assertion() {
	print!("trivial assertion... ");
	assert_eq!(1, 1);
	println!("[ok]");
}
