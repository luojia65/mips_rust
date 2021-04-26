pub mod digital;

use digital::*;

pub type MyGpioPeripheral = GpioPeripheral<0x1060_0000>;

pub struct GpioPeripheral<const P: usize> {
    pub pin0: GpioPin<Input, P, 0>,
    pub pin1: GpioPin<Input, P, 1>,
    pub pin2: GpioPin<Input, P, 2>,
    pub pin3: GpioPin<Input, P, 3>,
    pub pin4: GpioPin<Input, P, 4>,
    pub pin5: GpioPin<Input, P, 5>,
    pub pin6: GpioPin<Input, P, 6>,
    pub pin7: GpioPin<Input, P, 7>,
    pub pin8: GpioPin<Input, P, 8>,
    pub pin9: GpioPin<Input, P, 9>,
    pub pin10: GpioPin<Input, P, 10>,
    pub pin11: GpioPin<Input, P, 11>,
    pub pin12: GpioPin<Input, P, 12>,
    pub pin13: GpioPin<Input, P, 13>,
    pub pin14: GpioPin<Input, P, 14>,
    pub pin15: GpioPin<Input, P, 15>,
}

impl<const P: usize> GpioPeripheral<P> {
    pub fn take() -> Option<GpioPeripheral<P>> {
        // note(unsafe): 获得所有权
        Some(unsafe { core::mem::transmute(()) })
    }
}
