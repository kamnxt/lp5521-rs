pub enum Command {
    RampWait {
        prescale: bool,
        step_time: u8,
        increment: i8,
    },
    SetPwm {
        value: u8,
    },
    GoToStart,
    Branch {
        loop_times: u8,
        go_to_step: u8,
    },
    End {
        int: bool,
        reset: bool,
    },
    Trigger,
}

impl Command {
    pub fn wait_ms(millis: f32) -> Command {
        let prescale = millis > 30.87;
        let cycles = if prescale {
            f32::round(millis / 15.6) as u8
        } else {
            f32::round(millis / 0.49) as u8
        };
        Command::RampWait{prescale, step_time: cycles, increment:0}
    }
    pub fn ramp_ms(millis: u16, increment: i8) -> Command {
        let ms_per_increment = (millis as f32) / (i8::abs(increment)) as f32;
        // prescale can either be 16 or 512, and is used for scaling the 32768hz master clock.
        // those values correspond to 0.49ms and 15.6ms time per step, respectively.
        // maximum ms_per_increment is 63 as it's a 6bit value.
        // therefore if we have to wait more than 0.49*63=39.87ms we need to use the lower
        // accuracy prescale.
        let prescale = ms_per_increment > 30.87;
        let mut cycles = if prescale {
            f32::round(ms_per_increment / 15.6) as u8
        } else {
            f32::round(ms_per_increment / 0.49) as u8
        };
        if cycles == 0 {
            cycles = 1;
        }
        println!("ms: {:?}, pres: {:?}, cyc = {:?}, incr = {:?}", ms_per_increment, prescale, cycles, increment);

        Command::RampWait {
            prescale,
            step_time: cycles,
            increment,
        }
    }
    pub fn to_code(&self) -> u16 {
        match self {
            Command::RampWait {
                prescale,
                step_time,
                increment,
            } => {
                ((*prescale as u16) << 14)
                    | ((*step_time & 63) as u16) << 8
                    | ((*increment < 0) as u16) << 7
                    | (i8::abs(*increment) as u16)
            }
            Command::SetPwm {
                value
            } => {
                0b01000000_00000000 | (*value as u16)
            }
            Command::GoToStart => 0x0000,
            Command::End { int, reset } => {
                0b11000000_00000000 | (*int as u16) << 12 | (*reset as u16) << 11
            }
            _ => 0x0000,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ramp_ms() {
        // 500ms ramp up 127
        // = 8 cycles * 0.49ms, 127 times (actually comes out to 497.84ms)
        //                 p stime  s  incr
        let expected = 0b0_0_001000_0_1111111;
        assert_eq!(Command::ramp_ms(500, 127).to_code(), expected);

        // 500ms ramp down 127
        // = 8 cycles * 0.49ms, 127 times (actually comes out to 497.84ms)
        //                 p stime  s  incr
        let expected = 0b0_0_000100_1_1111111;
        assert_eq!(Command::ramp_ms(250, -127).to_code(), expected);

        // 2000ms ramp up 64
        // = 2 cycles * 15.6ms, 64 times (actually comes out to 1996.8ms)
        //                 p stime  s  incr
        let expected = 0b0_1_000010_0_1000000;
        assert_eq!(Command::ramp_ms(2000, 64).to_code(), expected);
    }
}
