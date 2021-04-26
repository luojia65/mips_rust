#![no_std]
#![no_main]

#![feature(global_asm)]

global_asm!(include_str!("entry.asm"));

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) ->! {
	loop {}
}

// the hardware abstraction layer 
mod hal;

mod led;

mod mem;

use hal::digital::GPIO;

#[no_mangle]
pub extern "C" fn main() ->! {
	GPIO::init();

	led::set_on(0);
	led::set_on(1);
	led::set_on(2);
	led::set_on(3);
	led::set_on(4);
	led::set_on(5);
	led::set_on(6);
	led::set_on(7);
	led::set_on(8);
	led::set_on(9);
	led::set_on(10);
	led::set_on(11);
	led::set_on(12);
	led::set_on(13);
	led::set_on(14);
	led::set_on(15);
	led::set_on(16);

	loop {}
}
