#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

// the no_mangle attribute ensures the rust compiler does not change the name of
// the _start function
#[no_mangle]
// this function is marked with "extern C" since it uses the C convention for
// calling a function
//
// the "!" return type means it has a divergent return type, or it never
// returns, since it will never be called, only invoked directly
// "since there’s nothing left to do if a freestanding binary returns"
pub extern "C" fn _start() -> ! {
	panic!()
}
