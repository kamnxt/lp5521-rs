use linux_embedded_hal::I2cdev;
use lp5521_rs::{ChargepumpMode, ClkMode, Lp5521};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let red: u8 = args.next().unwrap().parse().unwrap();
    let green: u8 = args.next().unwrap().parse().unwrap();
    let blue: u8 = args.next().unwrap().parse().unwrap();
    println!("Hello, world!");
    let i2c = I2cdev::new("/dev/i2c-0").unwrap();
    let mut lp5221 = Lp5521::new(i2c, 0x32);
    lp5221.set_enable(true, true).unwrap();
    lp5221.set_direct(true).unwrap();
    lp5221.set_config(true, ChargepumpMode::Auto, false, ClkMode::Auto).unwrap();
    lp5221.set_color(red, green, blue).unwrap();
}
