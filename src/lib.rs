use embedded_hal::blocking::i2c::{Write, WriteRead};
pub mod lp5521_program;
pub mod lp5521rgb;

#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum Channel {
    NoChannel = 0,
    R = 1,
    G,
    RG,
    B,
    RB,
    GB,
    RGB
}

impl std::ops::BitAnd<Channel> for Channel {
    type Output = Channel;
    fn bitand(self, rhs: Channel) -> Channel {
        Channel::from(self as u8 & rhs as u8)
    }
}
impl From<u8> for Channel {
    fn from(v: u8) -> Channel {
        match v {
            1 => Channel::R,
            2 => Channel::G,
            3 => Channel::RG,
            4 => Channel::B,
            5 => Channel::RB,
            6 => Channel::RG,
            7 => Channel::RGB,
            _ => Channel::NoChannel,
        }
    }

}

pub struct Lp5521<I2C> {
    i2c: I2C,
    address: u8,
}

#[repr(u8)]
pub enum ChargepumpMode {
    Off,
    ForceBypass,
    Force1point5,
    Auto,
}

#[repr(u8)]
pub enum ClkMode {
    External,
    Internal,
    Auto,
}

#[repr(u8)]
pub enum PwmMode {
    /// Internal oscillator
    HighFrequency,

    /// External oscillator
    LowFrequency, // external osc
}

#[repr(u8)]
pub enum ROutputConnection {
    /// R power supply connected to internal charge pump
    ChargePump,

    /// R power supply connected directly to battery
    Battery
}

#[repr(u8)]
pub enum BrightnessMode {
    /// Output is linear
    Linear,

    ///Output is logarithmic
    Logarithmic
}

#[repr(u8)]
pub enum ControllerMode {
    ///Disabled, reset PC
    Disabled,

    ///Load Program to SRAM, reset PC
    LoadProgram,

    ///Run program
    RunProgram,

    ///Direct control, reset PC
    DirectControl
}

pub struct StaticSettings {
    pub cp_mode: ChargepumpMode,
    pub clk_mode: ClkMode,
    pub pwm_mode: PwmMode,
    pub r_output_mode: ROutputConnection,
    pub brightness_mode: BrightnessMode,
}

impl<I2C, E> Lp5521<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        let lp5521 = Lp5521 { i2c, address };
        lp5521
    }
    pub fn init(&mut self, settings: StaticSettings) -> Result<(), Error<E>> {
        self.set_config(settings.pwm_mode, settings.cp_mode, settings.r_output_mode, settings.clk_mode)?;
        Ok(())
    }
    pub fn set_color_value(&mut self, red: u8, green: u8, blue: u8) -> Result<(), Error<E>> {
        const COLOR_0_ADDR: u8 = 0x02;
        self.i2c
            .write(self.address, &[COLOR_0_ADDR, red, green, blue])
            .map_err(Error::I2C)
    }
    pub fn set_enable(&mut self, brightness_mode: BrightnessMode, chip_en: bool, run_mode: bool) -> Result<(), Error<E>> {
        let mut value = 0;
        value |= (brightness_mode as u8) << 7;
        if chip_en {
            value |= 1 << 6;
        }
        if run_mode {
            value |= 0b101010;
        }
        self.write_reg(&[0x00, value])
    }

    pub fn set_control_mode(&mut self, mode: ControllerMode) -> Result<(), Error<E>> {
        let mode = mode as u8;
        // TODO make it possible to choose which leds to apply mode to
        self.write_reg(&[0x01, mode | mode << 2 | mode << 4])
    }

    pub fn set_direct(&mut self) -> Result<(), Error<E>> {
        self.set_control_mode(ControllerMode::DirectControl)
    }

    pub fn upload_program(&mut self, led_id: Channel, program: &[lp5521_program::Command]) -> Result<(), Error<E>> {
        let mut bytes = Vec::with_capacity(program.len() * 2 + 1);
        bytes.push(0x00);
        for cmd in program {
            let code = cmd.to_code();
            bytes.push((code >> 8) as u8);
            bytes.push((code & 0xff) as u8);
        }
        if led_id & Channel::R != Channel::NoChannel {
            // address
            bytes[0] = 0x10;
            self.write_reg(&bytes)?;
        }
        if led_id & Channel::G != Channel::NoChannel {
            // address
            bytes[0] = 0x30;
            self.write_reg(&bytes)?;
        }
        if led_id & Channel::B != Channel::NoChannel {
            // address
            bytes[0] = 0x50;
            self.write_reg(&bytes)?;
        }
        Ok(())
    }


    pub fn set_config(
        &mut self,
        pwm_mode: PwmMode,
        cp_mode: ChargepumpMode,
        r_output: ROutputConnection,
        clk: ClkMode,
    ) -> Result<(), Error<E>> {
        let mut value = 0;
        value |= clk as u8; // 2bit
        value |= (r_output as u8) << 2; // 1bit
        value |= (cp_mode as u8) << 3; // 2bit
        // power save skipped
        value |= (pwm_mode as u8) << 6; //1 bit
        self.write_reg(&[0x08, value])
    }

    pub fn write_reg(&mut self, values: &[u8]) -> Result<(), Error<E>> {
        self.i2c.write(self.address, values).map_err(Error::I2C)
    }
}
