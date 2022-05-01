use std::error;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
        Yup => (1, -32767.0),
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

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    output_controllers: u16,
    #[clap(short, long)]
    mayflash_controllers: u16,
    #[clap(short, long)]
    summed_axis_exponent: f32,
    #[clap(short, long)]
    p1_controller1_buttons_always_down: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Cli::parse();

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
        _ => sdljoysticktime(args, joystick_subsystem),
    }
}

fn sdljoysticktime(
    args: Cli,
    sdl2_input_joystick_subsystem: sdl2::JoystickSubsystem,
) -> Result<(), Box<dyn error::Error>> {
    let num_inputs_detected = sdl2_input_joystick_subsystem.num_joysticks().unwrap();
    for i in 0..num_inputs_detected {
        let name = sdl2_input_joystick_subsystem.name_for_index(i);
        let name = name.unwrap_or("<FAILED TO GET NAME INFORMATION>".into());
        println!("\t{0} --> Name: {1}", i, name);
    }

    let number_of_output_controllers = args.output_controllers;

    let input_joysticks_vector_of_vectors = {
        let mut input_vec_of_vecs = vec![];
        for _ in 0..number_of_output_controllers {
            input_vec_of_vecs.push(Vec::new())
        }
        let mut player_index = 0;
        let mut num_assigned_to_current_output_index = 0;
        let mut mayflash_count = 0;
        for i in 0..num_inputs_detected {
            if sdl2_input_joystick_subsystem
                .name_for_index(i)?
                .contains("MAYFLASH")
            {
                if mayflash_count >= args.mayflash_controllers {
                    continue;
                } else {
                    mayflash_count += 1;
                }
            }
            let current_in_joy = sdl2_input_joystick_subsystem.open(i)?;
            input_vec_of_vecs[player_index].push(current_in_joy);
            num_assigned_to_current_output_index += 1;
            if num_assigned_to_current_output_index >= {
                let controllers_per_team =
                    (num_inputs_detected as f32) / (number_of_output_controllers as f32);
                controllers_per_team.ceil() as i32
            } {
                player_index += 1;
                num_assigned_to_current_output_index = 0;
            }
        }
        input_vec_of_vecs
    };

    let out_joysticks = {
        let mut out_joysticks = Vec::new();
        for i in 0..number_of_output_controllers {
            let name = format!("Buster{}", i);
            let outjoy = software_joystick::Joystick::new(name)?;
            out_joysticks.push(outjoy);
        }
        out_joysticks
    };

    loop {
        sdl2_input_joystick_subsystem.update();
        for curr_out_joy_index in 0..out_joysticks.len() {
            let out_joystick = &out_joysticks[curr_out_joy_index];
            let input_joystick_vector = &input_joysticks_vector_of_vectors[curr_out_joy_index];

            // Iterates over all of the axes we care about- each element of the NamedAxis enum
            for named_axis in NamedAxis::iter() {
                let (id, scalar_map) = snes_namedaxis_to_id_and_scalar(&named_axis);

                out_joystick.move_axis(
                    {
                        match named_axis {
                            NamedAxis::Xright => software_joystick::Axis::X,
                            NamedAxis::Yup => software_joystick::Axis::Y,
                        }
                    },
                    {
                        let sum: f32 = input_joystick_vector
                            .iter()
                            .map(|ijoy| {
                                let raw_axis_value = ijoy.axis(id).unwrap() as f32;
                                let normalized_axis_value = raw_axis_value / scalar_map;
                                normalized_axis_value
                            })
                            .sum();
                        let average: f32 = sum / (input_joystick_vector.len() as f32);
                        let exponent_scaled =
                            average.signum() * average.abs().powf(args.summed_axis_exponent);
                        let out_scaled = exponent_scaled * 512.0;
                        out_scaled.trunc() as i32
                    },
                )?;
            }

            // Iterates over all the buttons we care about
            for named_button in NamedButton::iter() {
                out_joystick.button_press(
                    match named_button {
                        NamedButton::X => software_joystick::Button::RightNorth,
                        NamedButton::A => software_joystick::Button::RightEast,
                        NamedButton::B => software_joystick::Button::RightSouth,
                        NamedButton::Y => software_joystick::Button::RightWest,
                        NamedButton::L => software_joystick::Button::L1,
                        NamedButton::R => software_joystick::Button::R1,
                        NamedButton::Start => software_joystick::Button::RightSpecial,
                        NamedButton::Select => software_joystick::Button::LeftSpecial,
                    },
                    {
                        let sdl_inputs_button_id = snes_namedbutton_to_id(&named_button);
                        let in_joys_to_check = {
                            if args.p1_controller1_buttons_always_down
                                && curr_out_joy_index == 0
                                && input_joystick_vector.len() > 1
                            {
                                &input_joystick_vector[1..]
                            } else {
                                &input_joystick_vector
                            }
                        };
                        let is_pressed = in_joys_to_check
                            .iter()
                            .all(|ijoy| ijoy.button(sdl_inputs_button_id).unwrap());
                        is_pressed
                    },
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
