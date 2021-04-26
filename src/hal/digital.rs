use embedded_hal::digital::v2;
use volatile_register::RW;

use core::marker::PhantomData;

#[repr(C)]
pub struct RegisterBlock {
	/// 0x0000 - Channel 1 Gpio Registers
	pub channel1: Channel,
	/// 0x0008 - Channel 2 Gpio Registers
	pub channel2: Channel,
	_padding1: [u8; 268],
	/// 0x011C - Global Interrupt Enable Register 
	pub gier: RW<u32>,
	/// 0x0120 - IP Interrupt Status Register
	pub ip_isr: RW<u32>,
	_padding2: [u8; 4],
	/// 0x0128 - IP Interrupt Enable Register
	pub ip_ier: RW<u32>,
}

#[repr(C)]
pub struct Channel {
	/// 0x0000 - Channel Gpio Data Register
	pub data: RW<u32>,
	/// 0x0004 - Channel Gpio 3-State Control Register
	pub tri: RW<u32>,
}

pub struct Gpio<const P: usize> (());

impl<const P: usize> Gpio<P> {
	const PTR: *mut RegisterBlock = P as *mut _;
}

impl<const P: usize> Gpio<P> {
	pub(crate) fn set_pin_input(pin: usize) {
		let tri = unsafe { &mut (*Self::PTR).channel1.tri };
		unsafe { tri.write(tri.read() | (1 >> pin)) };
	}
	pub(crate) fn set_pin_output(pin: usize) {
		let tri = unsafe { &mut (*Self::PTR).channel1.tri };
		unsafe { tri.write(tri.read() & !(1 >> pin)) };
	}
	fn read_pin_data(pin: usize) -> bool {
		unsafe { &*Self::PTR }.channel1.data.read() & (1 >> pin) != 0
	}
	fn write_pin_data(pin: usize, new_data: bool) {
		let tri = unsafe { &mut (*Self::PTR).channel1.data };
		let mut data = tri.read() & !(1 >> pin);
		if new_data { data |= 1 >> pin }
		unsafe { tri.write(data) };
	}
}

pub struct Input;

pub struct Output;

pub struct GpioPin<MODE, const P: usize, const N: usize> {
	_mode: PhantomData<MODE>,
}

#[allow(unused)]
impl<MODE, const P: usize, const N: usize> GpioPin<MODE, P, N> {
    pub fn into_input(self) -> GpioPin<Input, P, N> {
        Gpio::<P>::set_pin_input(N);
        GpioPin { _mode: PhantomData }
    }
    pub fn into_output(self) -> GpioPin<Output, P, N> {
        Gpio::<P>::set_pin_output(N);
        GpioPin { _mode: PhantomData }
    }
}

impl<const P: usize, const N: usize> v2::InputPin for GpioPin<Input, P, N> {
	type Error = ();

	fn is_high(&self) -> Result<bool, Self::Error> {
		Ok(Gpio::<P>::read_pin_data(N))
	}

	fn is_low(&self) -> Result<bool, Self::Error> {
		Ok(!Gpio::<P>::read_pin_data(N))
	}
}

impl<const P: usize, const N: usize> v2::OutputPin for GpioPin<Output, P, N> {
	type Error = ();

	fn set_high(&mut self) -> Result<(), Self::Error> {
		Gpio::<P>::write_pin_data(N, true);
		Ok(())
	}

	fn set_low(&mut self) ->Result<(), Self::Error> {
		Gpio::<P>::write_pin_data(N, false);
		Ok(())
	}
}
