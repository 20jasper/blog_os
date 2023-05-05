#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
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

/// called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{info}");
	loop {}
}

/// called on panic in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	serial_println!("[failed]\n");
	serial_println!("Error: {}\n", info);
	exit_qemu(QemuExitCode::Failed);
	loop {}
}

/// Codes passed to QEMU based on test failure or success
/// These values are arbitrary
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}

/// runs all tests with the `#[test]` attribute
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
	serial_println!("Running {} tests", tests.len());

	for test in tests {
		test.run();
	}

	exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}

/// trait adding the `run` function
pub trait Testable {
	fn run(&self);
}

/// prints the name of the function, then runs it and logs if it passes. An
/// error is thrown otherwise
// implements the testable trait for all types `T` that have the `Fn()` trait
impl<T> Testable for T
// where clauses are a much nicer way to write generic constraints in typescript
// `Type extends string`
where
	T: Fn(),
{
	fn run(&self) {
		// print description of a type--in this case, the function name
		serial_print!("{}...\t", core::any::type_name::<T>());
		// call self, which is the function
		self();

		serial_println!("[ok]");
	}
}
