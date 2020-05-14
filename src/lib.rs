use embedded_hal::blocking::i2c::{Write, WriteRead};
use linux_embedded_hal::I2cdev;

#[derive(Debug)]

pub enum Error<E> {
    /// I²C bus error
    I2C(E),

    /// Invalid input data.
    WrongAddress,

    WriteToReadOnly,
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

impl<I2C, E> Lp5521<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        let lp5521 = Lp5521 { i2c, address };
        lp5521
    }
    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) -> Result<(), Error<E>> {
        const COLOR_0_ADDR: u8 = 0x02;
        self.i2c
            .write(self.address, &[COLOR_0_ADDR, red, green, blue])
            .map_err(Error::I2C)
    }
    pub fn set_enable(&mut self, log_en: bool, chip_en: bool) -> Result<(), Error<E>> {
        let mut value = 0;
        if log_en {
            value |= 1 << 7;
        }
        if chip_en {
            value |= 1 << 6;
        }
        self.write_reg(&[0x0, value])
    }

    pub fn set_direct(&mut self, direct: bool) -> Result<(), Error<E>> {
        let value = {
            if direct {
                0b11
            } else {
                0b00
            }
        };
        self.write_reg(&[1u8, value | value << 2 | value << 4])
    }

    pub fn set_config(
        &mut self,
        pwm_hf: bool,
        cp_mode: ChargepumpMode,
        r_to_batt: bool,
        clk: ClkMode,
    ) -> Result<(), Error<E>> {
        let mut value = 0;
        if pwm_hf {
            value |= 1 << 7;
        }
        value |= (cp_mode as u8) << 3;
        if r_to_batt {
            value |= 1 << 2;
        }
        value |= clk as u8;
        self.write_reg(&[0x8, value])
    }

    pub fn write_reg(&mut self, values: &[u8]) -> Result<(), Error<E>> {
        self.i2c.write(self.address, values).map_err(Error::I2C)
    }
}
