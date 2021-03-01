use linux_embedded_hal::I2cdev;
use crate::Lp5521;
use crate::StaticSettings;
use crate::lp5521_program::Command;
use crate::{Channel, BrightnessMode, ControllerMode};
use crate::Error;

pub struct Lp5521RGB {
//    lp5521: Lp5521::Lp5521<I2cdev, Lp5521::Error<linux_embedded_hal::i2cdev::linux::LinuxI2CError>>,
    lp5521: Lp5521<I2cdev>,
}

impl Lp5521RGB {
    pub fn new(i2c: I2cdev, addr: u8, settings: StaticSettings) -> Self {
        let mut the_lp5221: Lp5521<I2cdev> = Lp5521::new(i2c, addr);
        the_lp5221.init(settings);
        Self { lp5521: the_lp5221 }
    }
    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) -> Result<(), Error<linux_embedded_hal::i2cdev::linux::LinuxI2CError>>{
        self.lp5521.set_direct()?;
        self.lp5521.set_color_value(red, green, blue)?;
        Ok(())
    }
    pub fn blink_color(&mut self, millis: u16, red: u8, green: u8, blue: u8) -> Result<(), Error<linux_embedded_hal::i2cdev::linux::LinuxI2CError>>{
        self.lp5521.set_enable(BrightnessMode::Logarithmic, true, false)?;
        self.lp5521.set_control_mode(ControllerMode::LoadProgram)?;
        let red = (red/4) as i8;
        let green = (green/2) as i8;
        let blue = (blue/2) as i8;
        let program = [
            Command::ramp_ms(millis/4, red),
            Command::ramp_ms(millis/4, red),
            Command::ramp_ms(millis/4, -red),
            Command::ramp_ms(millis/4, -red),
            Command::GoToStart,
        ];
        self.lp5521.upload_program(Channel::R, &program)?;
        let program = [
            Command::ramp_ms(millis/4, green),
            Command::ramp_ms(millis/4, green),
            Command::ramp_ms(millis/4, -green),
            Command::ramp_ms(millis/4, -green),
            Command::GoToStart,
        ];
        self.lp5521.upload_program(Channel::G, &program);
        let program = [
            Command::ramp_ms(millis/4, blue),
            Command::ramp_ms(millis/4, blue),
            Command::ramp_ms(millis/4, -blue),
            Command::ramp_ms(millis/4, -blue),
            Command::GoToStart,
        ];
        self.lp5521.upload_program(Channel::B, &program);
        self.lp5521.set_control_mode(ControllerMode::RunProgram);
        self.lp5521.set_enable(BrightnessMode::Logarithmic, true, true)?;
        Ok(())
    }

    pub fn rainbow(&mut self, cycle_time: u16) -> Result<(), Error<linux_embedded_hal::i2cdev::linux::LinuxI2CError>>{
        self.lp5521.set_enable(BrightnessMode::Logarithmic, true, false)?;
        self.lp5521.set_control_mode(ControllerMode::LoadProgram)?;
        let program = [
            Command::ramp_ms(cycle_time/3, 50),
            Command::ramp_ms(cycle_time/3, -50),
            Command::ramp_ms(cycle_time/3, -50),
            Command::GoToStart,
        ];
        self.lp5521.upload_program(Channel::R, &program)?;
        let program = [
            Command::ramp_ms(cycle_time/3, -50),
            Command::ramp_ms(cycle_time/3, 50),
            Command::ramp_ms(cycle_time/3, -50),
            Command::GoToStart,
        ];
        self.lp5521.upload_program(Channel::G, &program);
        let program = [
            Command::ramp_ms(cycle_time/3, -50),
            Command::ramp_ms(cycle_time/3, -50),
            Command::ramp_ms(cycle_time/3, 50),
            Command::GoToStart,
        ];
        self.lp5521.upload_program(Channel::B, &program);
        self.lp5521.set_control_mode(ControllerMode::RunProgram);
        self.lp5521.set_enable(BrightnessMode::Logarithmic, true, true)?;
        Ok(())
    }

}
