#![feature(asm)]
#![no_std]
#![no_main]

// the hardware abstraction layer 
mod hal;
mod mem;

use embedded_hal::digital::v2::OutputPin;
use hal::MyGpioPeripheral;
use core::panic::PanicInfo;

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[export_name = "rust_main"]
pub extern "C" fn rust_main() -> ! {
	let p = MyGpioPeripheral::take().unwrap();
	
	p.pin0.into_output().set_high().ok();
	p.pin1.into_output().set_high().ok();
	p.pin2.into_output().set_high().ok();
	p.pin3.into_output().set_high().ok();
	p.pin4.into_output().set_high().ok();
	p.pin5.into_output().set_high().ok();
	p.pin6.into_output().set_high().ok();
	p.pin7.into_output().set_high().ok();
	p.pin8.into_output().set_high().ok();
	p.pin9.into_output().set_high().ok();
	p.pin10.into_output().set_high().ok();
	p.pin11.into_output().set_high().ok();
	p.pin12.into_output().set_high().ok();
	p.pin13.into_output().set_high().ok();
	p.pin14.into_output().set_high().ok();
	p.pin15.into_output().set_high().ok();

	loop {}
}

const BOOT_STACK_SIZE: usize = 1024 * 2;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];

#[link_section = ".text.entry"]
#[export_name = "entry"]
pub unsafe fn entry() -> ! {
	asm!(
		"la     $sp, {boot_stack}",
		"addi   $sp, $sp, {boot_stack_size}",
		".cprestore 4",
		"jal    {rust_main}",
		boot_stack = sym BOOT_STACK,
		boot_stack_size = const BOOT_STACK_SIZE,
		rust_main = sym rust_main,
		options(noreturn)
	)
}
