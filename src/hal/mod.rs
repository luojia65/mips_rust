pub mod digital;

use digital::*;

pub type MyGpioPin<MODE, const N: usize> = GpioPin<MODE, 0x1060_0000, N>;

pub struct MyGpioPeripheral {
    pub pin0: MyGpioPin<Input, 0>,
    pub pin1: MyGpioPin<Input, 1>,
    pub pin2: MyGpioPin<Input, 2>,
    pub pin3: MyGpioPin<Input, 3>,
    pub pin4: MyGpioPin<Input, 4>,
    pub pin5: MyGpioPin<Input, 5>,
    pub pin6: MyGpioPin<Input, 6>,
    pub pin7: MyGpioPin<Input, 7>,
    pub pin8: MyGpioPin<Input, 8>,
    pub pin9: MyGpioPin<Input, 9>,
    pub pin10: MyGpioPin<Input, 10>,
    pub pin11: MyGpioPin<Input, 11>,
    pub pin12: MyGpioPin<Input, 12>,
    pub pin13: MyGpioPin<Input, 13>,
    pub pin14: MyGpioPin<Input, 14>,
    pub pin15: MyGpioPin<Input, 15>,
}

impl MyGpioPeripheral {
    pub fn take() -> Option<MyGpioPeripheral> {
        // note(unsafe): 获得所有权
        Some(unsafe { core::mem::transmute(()) })
    }
}
