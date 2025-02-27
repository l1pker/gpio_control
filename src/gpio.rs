#[cfg(target_os = "linux")]
use rppal::gpio::{Gpio, OutputPin};

pub struct GpioControl {
    #[cfg(target_os = "linux")]
    pin: OutputPin,
}

impl GpioControl {
    pub fn new(pin_num: u8) -> Self {
        #[cfg(target_os = "linux")]
        let pin = Gpio::new().unwrap().get(pin_num).unwrap().into_output();
        
        GpioControl {
            #[cfg(target_os = "linux")]
            pin,
        }
    }

    pub fn turn_on(&mut self) {
        #[cfg(target_os = "linux")]
        self.pin.set_high();
    }

    pub fn turn_off(&mut self) {
        #[cfg(target_os = "linux")]
        self.pin.set_low();
    }
}