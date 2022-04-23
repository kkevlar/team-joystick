use std::error;
use std::vec;


mod joystick;
mod serial;

#[derive(Debug)]
struct UserError
{
reason : String,
}

impl error::Error for UserError {}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User Error: {}", self.reason)
        }
    }

fn main() -> Result<(), Box<dyn error::Error>> {
    let my_sdl2 = sdl2::init().expect("Failed to Initialize SDL2. Install libsdl2-dev?");
    let joystick_subsystem = my_sdl2.joystick().expect("Failed to get the SDL Joystick Subsystem.");
    let joystick_count = joystick_subsystem.num_joysticks().expect("Failed to get the number of joysticks...");
    println!("SDL2 Initialization Complete. Detected {0} joyticks...", joystick_count);

    match joystick_count
    {
        0 => Err(Box::new(UserError{ reason: "Cannot run this tool with 0 joysticks connected.".into(), })),
        _ => sdljoysticktime(joystick_subsystem),
    }
}

fn sdljoysticktime(joystick_subsystem : sdl2::JoystickSubsystem) -> Result<(), Box<dyn error::Error>> 
{
    let num = joystick_subsystem.num_joysticks().unwrap();
    for i in 0..num
    {
        let name = joystick_subsystem.name_for_index(i);
        let name = name.unwrap_or("<FAILED TO GET NAME INFORMATION>".into());
        println!("\t{0} --> Name: {1}", i, name);
    }

    let joy_vec : Result<Vec<sdl2::joystick::Joystick> , sdl2::IntegerOrSdlError>= (0..num).into_iter().map( |n| joystick_subsystem.open(n)).collect();
    let joy_vec = joy_vec?;

    loop{
        joystick_subsystem.update();
        for (js_index, js ) in joy_vec.iter().enumerate()
        {
            for button_index in  0..js.num_buttons()
            {
                if js.button(button_index)?
                {
                    println!("Pressed! Controller {}, button {}", js_index, button_index);
                }
            }
        }
    }

    // let js = joystick_subsystem.open(0)?;
    return Ok(());

    let joystick = joystick::Joystick::new()?;

    println!(
        "Created joystick with device path {}",
        joystick.device_path()?.to_string_lossy()
    );

    loop {
        joystick.button_press(joystick::Button::LeftNorth, true)?;
        joystick.button_press(joystick::Button::RightSouth, true)?;
        joystick.move_axis(joystick::Axis::Y, 100)?;

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