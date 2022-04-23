use std::{env, error};

mod joystick;
mod serial;

fn main() -> Result<(), Box<dyn error::Error>> {
    // let args: Vec<_> = env::args().collect();
    // let port = if args.len() > 1 {
    //     args[1].clone()
    // } else {
    //     "/dev/ttyUSB1".to_owned()
    // };

    // println!("Connecting to serial port at {}", port);
    // let mut serial = serial::SerialConnection::new(&port.into(), 115200)?;

    let joystick = joystick::Joystick::new()?;

    println!(
        "Created joystick with device path {}",
        joystick.device_path()?.to_string_lossy()
    );

    loop {
        joystick.button_press(joystick::Button::LeftNorth, true)?;
        joystick.button_press(joystick::Button::RightSouth, true)?;

        joystick.synchronise()?;
    }
}

fn button_map(i: usize) -> joystick::Button {
    use joystick::Button::*;
    match i {
        0 => LeftNorth,
        1 => LeftWest,
        2 => LeftEast,
        3 => LeftSouth,
        4 => LeftSpecial,
        5 => RightSouth,
        6 => RightSpecial,
        7 => RightEast,
        8 => RightWest,
        9 => RightNorth,
        10 => R2,
        11 => R1,
        12 => L2,
        13 => L1,
        _ => unreachable!(),
    }
}

fn axis_map(i: usize) -> joystick::Axis {
    use joystick::Axis::*;
    match i {
        0 => X,
        1 => Y,
        2 => RX,
        3 => RY,
        _ => unreachable!(),
    }
}
