use std::error;

use sdl2::sys::SDL_INIT_JOYSTICK;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod joystick;
mod serial;

#[derive(Debug)]
struct UserError {
    reason: String,
}

impl error::Error for UserError {}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User Error: {}", self.reason)
    }
}

#[derive(Debug, EnumIter)]
enum NamedButton {
    A,
    B,
    X,
    Y,
    L,
    R,
    Select,
    Start,
}

#[derive(Debug, EnumIter)]
enum NamedAxis {
    Xright,
    Yup,
}

fn snes_namedaxis_to_id_and_scalar(a: &NamedAxis) -> (u32, f32) {
    use NamedAxis::*;
    match a {
        Xright => (0, 32767.0),
        Yright => (1, -32767.0),
    }
}

#[derive(Debug)]
struct UnmappedError;

fn snes_namedbutton_to_id(b: &NamedButton) -> u32 {
    use NamedButton::*;
    match b {
        X => 0,
        A => 1,
        B => 2,
        Y => 3,
        Start => 9,
        Select => 8,
        L => 4,
        R => 5,
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let my_sdl2 = sdl2::init().expect("Failed to Initialize SDL2. Install libsdl2-dev?");
    let joystick_subsystem = my_sdl2
        .joystick()
        .expect("Failed to get the SDL Joystick Subsystem.");
    let joystick_count = joystick_subsystem
        .num_joysticks()
        .expect("Failed to get the number of joysticks...");
    println!(
        "SDL2 Initialization Complete. Detected {0} joyticks...",
        joystick_count
    );

    match joystick_count {
        0 => Err(Box::new(UserError {
            reason: "Cannot run this tool with 0 joysticks connected.".into(),
        })),
        _ => sdljoysticktime(joystick_subsystem),
    }
}

fn sdljoysticktime(
    joystick_subsystem: sdl2::JoystickSubsystem,
) -> Result<(), Box<dyn error::Error>> {
    let num = joystick_subsystem.num_joysticks().unwrap();
    for i in 0..num {
        let name = joystick_subsystem.name_for_index(i);
        let name = name.unwrap_or("<FAILED TO GET NAME INFORMATION>".into());
        println!("\t{0} --> Name: {1}", i, name);
    }

    let joy_vecs = {
        let mut joy_vecs = vec![Vec::new(), Vec::new()];
        let mut player_index = 0;
        for i in 0..num {
            let joy = joystick_subsystem.open(i)?;
            joy_vecs[player_index].push(joy);
            player_index += 1;
            player_index %= joy_vecs.len();
        }
        joy_vecs
    };

    let out_p1 = joystick::Joystick::new("BustersDirtySecret".into())?;
    let out_p2 = joystick::Joystick::new("BustersDirtySecretSqueakwel".into())?;
    let out_joysticks = [out_p1, out_p2];

    loop {
        joystick_subsystem.update();
        for player_index in 0..out_joysticks.len() {
            let out_joystick = &out_joysticks[player_index];
            let input_joystick_vector = &joy_vecs[player_index];

            for named_axis in NamedAxis::iter() {
                let (id, scalar_map) = snes_namedaxis_to_id_and_scalar(&named_axis);

                out_joystick.move_axis(
                    {
                        use joystick::Axis::*;
                        use NamedAxis::*;
                        match named_axis {
                            Xright => X,
                            Yup => Y,
                        }
                    },
                    {
                        let sum: f32 = input_joystick_vector
                            .iter()
                            .map(|ijoy| ((ijoy.axis(id).unwrap() as f32) / scalar_map))
                            .sum();
                        let avg: f32 = sum / (input_joystick_vector.len() as f32);
                        let value = avg * 512.0;
                        value.trunc() as i32
                    },
                )?;
            }
            for named_button in NamedButton::iter() {
                let id = snes_namedbutton_to_id(&named_button);
                let is_pressed = input_joystick_vector
                    .iter()
                    .all(|ijoy| ijoy.button(id).unwrap());
                out_joystick.button_press(
                    match named_button {
                        NamedButton::X => joystick::Button::RightNorth,
                        NamedButton::A => joystick::Button::RightEast,
                        NamedButton::B => joystick::Button::RightSouth,
                        NamedButton::Y => joystick::Button::RightWest,
                        NamedButton::L => joystick::Button::L1,
                        NamedButton::R => joystick::Button::R1,
                        NamedButton::Start => joystick::Button::RightSpecial,
                        NamedButton::Select => joystick::Button::LeftSpecial,
                    },
                    is_pressed,
                )?;
            }
            out_joystick.synchronise()?;
        }

    }

    // loop {
    //     joystick_subsystem.update();
    //     for (js_index, js) in joy_vec.iter().enumerate() {
    //         for button_index in 0..js.num_buttons() {
    //             if js.button(button_index)? {
    //                 println!("Pressed! Controller {}, button {}", js_index, button_index);
    //             }
    //         }
    //     }
    // }

    // // let js = joystick_subsystem.open(0)?;
    // return Ok(());

    // let joystick = joystick::Joystick::new("BustersDirtySecret".into())?;

    // println!(
    //     "Created joystick with device path {}",
    //     joystick.device_path()?.to_string_lossy()
    // );

    // loop {
    //     joystick.button_press(joystick::Button::LeftNorth, true)?;
    //     joystick.button_press(joystick::Button::RightSouth, true)?;
    //     joystick.move_axis(joystick::Axis::Y, 100)?;

    //     joystick.synchronise()?;
    // }
}

// fn button_map(i: usize) -> joystick::Button {
//     use joystick::Button::*;
//     match i {
//         0 => LeftNorth,
//         1 => LeftWest,
//         2 => LeftEast,
//         3 => LeftSouth,
//         4 => LeftSpecial,
//         5 => RightSouth,
//         6 => RightSpecial,
//         7 => RightEast,
//         8 => RightWest,
//         9 => RightNorth,
//         10 => R2,
//         11 => R1,
//         12 => L2,
//         13 => L1,
//         _ => unreachable!(),
//     }
// }

// fn axis_map(i: usize) -> joystick::Axis {
//     use joystick::Axis::*;
//     match i {
//         0 => X,
//         1 => Y,
//         2 => RX,
//         3 => RY,
//         _ => unreachable!(),
//     }
// }
