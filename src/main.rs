use linux_embedded_hal::I2cdev;
use lp5521_rs::{ChargepumpMode, ClkMode, Lp5521, ROutputConnection, PwmMode, BrightnessMode, ControllerMode, Channel, StaticSettings};
use lp5521_rs::lp5521_program::Command;
use lp5521_rs::lp5521rgb::Lp5521RGB;
use std::process;
use std::thread::sleep;
use std::time::Duration;

fn raw_control_demo(i2c: I2cdev, red: u8, green: u8, blue:u8) -> Result<(), lp5521_rs::Error<linux_embedded_hal::i2cdev::linux::LinuxI2CError>>{
    let mut lp5221 = Lp5521::new(i2c, 0x32);
    lp5221.set_config(PwmMode::HighFrequency, ChargepumpMode::Auto, ROutputConnection::ChargePump, ClkMode::Auto);
    lp5221.set_enable(BrightnessMode::Logarithmic, true, false)?;
    lp5221.set_control_mode(ControllerMode::LoadProgram)?;
    let program = [
        Command::ramp_ms(200, 50),
        Command::ramp_ms(200, -50),
        Command::ramp_ms(200, -50),
        Command::GoToStart,
    ];
    lp5221.upload_program(Channel::R, &program)?;
    let program = [
        Command::ramp_ms(200, -50),
        Command::ramp_ms(200, 50),
        Command::ramp_ms(200, -50),
        Command::GoToStart,
    ];
    lp5221.upload_program(Channel::G, &program);
    let program = [
        Command::ramp_ms(200, -50),
        Command::ramp_ms(200, -50),
        Command::ramp_ms(200, 50),
        Command::GoToStart,
    ];
    lp5221.upload_program(Channel::B, &program);
    sleep(Duration::from_millis(100));
    lp5221.set_control_mode(ControllerMode::RunProgram);
    lp5221.set_enable(BrightnessMode::Logarithmic, true, true)?;
    sleep(Duration::from_millis(10000));
    lp5221.set_enable(BrightnessMode::Logarithmic, true, false)?;
    let program = [
        Command::ramp_ms(100, 10),
        Command::ramp_ms(100, -10),
        Command::ramp_ms(100, 10),
        Command::ramp_ms(100, -10),
        Command::ramp_ms(100, 10),
        Command::ramp_ms(100, -10),
        Command::ramp_ms(100, 10),
        Command::ramp_ms(100, -10),
        Command::End{reset: false, int: false},
    ];
    println!("load program");
    lp5221.set_control_mode(ControllerMode::LoadProgram);
    println!("upload program");
    lp5221.upload_program(Channel::RGB, &program);
    println!("run program");
    lp5221.set_control_mode(ControllerMode::RunProgram);
    lp5221.set_enable(BrightnessMode::Logarithmic, true, true)?;
    sleep(Duration::from_millis(8000));
    lp5221.set_enable(BrightnessMode::Logarithmic, true, false)?;
    lp5221.set_direct()?;
    lp5221.set_color_value(red, green, blue)?;
    Ok(())
}

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    if args.len() < 3 {
        eprintln!("Usage: lp5521-rs R G B");
        eprintln!("where 0 <=R, G, B <= 255");
        process::exit(1);
    }
    let red: u8 = args.next().unwrap().parse().unwrap();
    let green: u8 = args.next().unwrap().parse().unwrap();
    let blue: u8 = args.next().unwrap().parse().unwrap();
    println!("Hello, world!");
    let i2c = I2cdev::new("/dev/i2c-0");
    if let Err(e) = i2c {
        eprintln!("Failed to open I2C device: {:?}", e);
        process::exit(1);
    };
    println!("i2c opened");
    let i2c = i2c.unwrap();
//    if let Err(err) = raw_control_demo(i2c, red, green, blue) {
//        eprintln!("oof {:?}", err);
//        process::exit(1);
//    }
//
    let config = StaticSettings {
        cp_mode: ChargepumpMode::Auto,
        clk_mode: ClkMode::Auto,
        pwm_mode: PwmMode::HighFrequency,
        r_output_mode: ROutputConnection::ChargePump,
        brightness_mode: BrightnessMode::Logarithmic
    };
    let mut rgb = Lp5521RGB::new(i2c, 0x32, config);
    rgb.blink_color(200, red, green, blue);
    sleep(Duration::from_millis(1000));
    rgb.blink_color(100, red, green, blue);
    sleep(Duration::from_millis(1000));
    rgb.rainbow(500);
    sleep(Duration::from_millis(1000));
    rgb.set_color(red, green, blue);
}
