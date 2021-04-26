use embedded_hal::digital::v2;

use core::ptr::{
	write_volatile, 
	read_volatile, 
};

pub use v2::InputPin as Input;
pub use v2::OutputPin as Output;

#[allow(dead_code)]
const GPIO_BASE: usize = 0x1060_0000;

#[allow(dead_code)]
mod offset {
	pub const GPIO_DATA: usize 	= 0x0000;
	pub const GPIO_TRI: usize 	= 0x0004;
	pub const GPIO2_DATA: usize = 0x0008;
	pub const GPIO2_TRI: usize 	= 0x000c;

	pub const GIER: usize 		= 0x011c;
	pub const IP_IER: usize 	= 0x0128;
	pub const IP_ISR: usize 	= 0x0120;
}

static mut GPIO_DATA: u32 = 0;

pub struct GPIO;

impl GPIO {
	pub fn init() ->GPIO {
		let channel1 = GPIO_BASE + offset::GPIO_TRI;
		let channel2 = GPIO_BASE + offset::GPIO2_TRI;

		unsafe {
			// set channel1 as output 
			write_volatile(channel1 as *mut u32, 0x0000);

			// reset GPIO_DATA as 0x0000 
			GPIO_DATA = 0x0000;
			write_volatile((GPIO_BASE + offset::GPIO_DATA) as *mut u32, GPIO_DATA);

			// set channel2 as input 
			write_volatile(channel2 as *mut u32, 0xffff);
		}

		GPIO {}
	}

	#[allow(non_snake_case)]
	pub fn get_InputPin(pin: u32) ->Option<InputPin> {
		let tri: u32;
		unsafe {
			tri = read_volatile((GPIO_BASE + offset::GPIO2_TRI) as *const u32);
		}

		if 0 == (tri >> pin) & 0x01 {
			Some(InputPin(pin)) 
		}
		else {None}
	}

	#[allow(non_snake_case)]
	pub fn get_OutputPin(pin: u32) ->Option<OutputPin> {
		let tri: u32;
		unsafe {
			tri = read_volatile((GPIO_BASE + offset::GPIO_TRI) as *const u32) as u32;
		}

		Some(OutputPin(pin))
	}
}

pub struct InputPin(u32);
impl v2::InputPin for InputPin {
	type Error = ();

	fn is_high(&self) ->Result<bool, Self::Error> {
		let data: u32;
		unsafe {
			data = read_volatile((GPIO_BASE + offset::GPIO2_DATA) as *const u32);
		}

		Ok(0x01 == (data >> self.0) & 0x01) 
	}

	fn is_low(&self) ->Result<bool, Self::Error> {
		let data: u32;
		unsafe {
			data = read_volatile((GPIO_BASE + offset::GPIO2_DATA) as *const u32);
		}

		Ok(0x00 == (data >> self.0) & 0x01) 
	}
}

pub struct OutputPin(u32);
impl v2::OutputPin for OutputPin {
	type Error = ();

	fn set_high(&mut self) ->Result<(), Self::Error> {
		let mask = 0x01u32 << self.0;

		unsafe {
			GPIO_DATA = GPIO_DATA | mask;
			write_volatile((GPIO_BASE + offset::GPIO_DATA) as *mut u32, GPIO_DATA);
		}
		Ok(())
	}

	fn set_low(&mut self) ->Result<(), Self::Error> {
		let mask = !(0x01u32 << self.0);

		unsafe {
			GPIO_DATA = GPIO_DATA & mask;
			write_volatile((GPIO_BASE + offset::GPIO_DATA) as *mut u32, GPIO_DATA);
		}
		Ok(())
	}
}
