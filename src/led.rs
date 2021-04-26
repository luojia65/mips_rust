// LED lighting up 

use crate::hal::digital::{
	GPIO, Output, 
};

#[inline]
pub fn set_on(num: u32) {
	GPIO::get_OutputPin(num).unwrap()
			.set_high().unwrap();
}

#[inline]
pub fn set_off(num: u32) {
	GPIO::get_OutputPin(num).unwrap()
			.set_low().unwrap();
}
