mod gui;
mod injoy;
mod joypaths;
mod manipulate_emulator;
mod outjoy;
mod wordhash;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "config.json")]
    config: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    team_allocation: Vec<u32>,
    path_hash_salt: u32,
    team_hash_salt: u32,
    path_common_name_max_length: u32,
    hat_only_players: Vec<String>,
    number_of_multi_port_controllers_to_use: u32,
}

use std::collections::HashMap;

use anyhow;

fn team_name_logo(
    words: &wordhash::Wordhash,
    config: &Config,
    team: &Vec<String>,
) -> anyhow::Result<String> {
    let team_bytes = {
        let mut team_bytes = Vec::new();
        for member in team {
            team_bytes.extend_from_slice(member.as_bytes());
        }
        team_bytes
    };
    let word = words.team_name(team_bytes.as_slice());

    use std::io::Write;
    let logo_name = format!("images/{}.jpg", word);
    //Test if the file exists
    if std::path::Path::new(&logo_name).exists() {
    } else {
        let prompt = format!("Sports team logo for a team called the {}", word);
        dbg!(prompt);
        return Ok(word);

        // Submit the prompt to deepai.org to generate a logo for the team, with grid size = 1
        let mut res = reqwest::blocking::Client::new()
            .post("https://api.deepai.org/api/logo-generator")
            .header("Api-Key", "8979b9e6-9eb9-404c-a182-bb8d180a1a69")
            .form(&[("text", prompt), ("grid_size", "1".to_string())])
            .send()?;
        dbg!(&res);

        // Get the url of the generated image.
        let url = res.json::<serde_json::Value>();
        let url2 = url.unwrap();
        let url3 = url2["output_url"]
            .as_str()
            .ok_or(anyhow::anyhow!("no url"))?;
        dbg!(&url3);
        // Download the image.
        res = reqwest::blocking::Client::new().get(url3).send()?;

        // Save the image to disk.
        let mut file = std::fs::File::create(logo_name)?;
        let it = &res.bytes()?;
        let mut mit = &mut it.clone();
        file.write_all(&mut mit)?;
    }
    Ok(word)
}

#[derive(Debug, Deserialize, Serialize)]
struct Team {
    name: String,
    players: Vec<String>,
    out_index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct TeamLock {
    teams: Vec<Team>,
}

fn main() {
    let args = Cli::parse();
    let config = serde_json::from_str::<Config>(&std::fs::read_to_string(&args.config).unwrap())
        .expect("Failed to parse config file");
    dbg!(&config);
    let words = wordhash::Wordhash::new(config.path_hash_salt, config.team_hash_salt);

    gui::do_cubes();

    // Read configuration file .json file
    let joy_paths = joypaths::repath_joys(&words, &config);
    let mpl: joypaths::MinimalPathLookup = joy_paths.unwrap().into();
    let mut minimal_paths: Vec<&String> = mpl.0.keys().collect();
    minimal_paths.sort();
    minimal_paths.reverse();

    for path in minimal_paths.iter() {
        let joy = &mpl.0[*path];
        println!("{: <15} -> {: <20}", joy.common_name, path);
    }

    let frozen_path = "teamlock.json";
    // Check for a teamlock.json file
    let frozen = if std::path::Path::new(&frozen_path).exists() {
        // If it exists, read it and return it
        let frozen =
            serde_json::from_str::<TeamLock>(&std::fs::read_to_string(&frozen_path).unwrap())
                .expect("Failed to parse frozen file");
        frozen
    } else {
        let num_required: u32 = config.team_allocation.iter().sum();
        let num_required = num_required as usize;
        if num_required != minimal_paths.len() {
            println!(
                "Incorrect number of joysticks connected. Expected {}, found {}",
                num_required,
                minimal_paths.len()
            );
            std::process::exit(1);
        }

        let mut frozen: TeamLock = TeamLock { teams: Vec::new() };
        let mut minimal_path_index = 0;
        for team_index in 0..config.team_allocation.len() {
            let mut team = Vec::new();
            for _ in 0..config.team_allocation[team_index] {
                let path = &minimal_paths[minimal_path_index];
                let joy = &mpl.0[*path];
                team.push(joy.common_name.clone());
                minimal_path_index += 1;
            }
            let team_name = team_name_logo(&words, &config, &team).unwrap();
            let team = Team {
                name: team_name,
                players: team,
                out_index: team_index as u32,
            };
            frozen.teams.push(team);
        }
        frozen
    };
    dbg!(&frozen);

    // Check frozen
    let mut total_player_count = 0;
    let mut missing_players = Vec::new();
    for team in frozen.teams.iter() {
        for player in team.players.iter() {
            let mut fail = true;
            for joy in mpl.0.values() {
                if joy.common_name == *player {
                    fail = false;
                    break;
                }
            }
            if fail {
                missing_players.push(player);
            }
            total_player_count += 1;
        }
    }

    if missing_players.len() > 0 {
        println!("Missing players:");
        for player in missing_players.iter() {
            println!("\t{}", player);
        }
        std::process::exit(1);
    }

    if total_player_count != minimal_paths.len() {
        println!(
            "Incorrect number of joysticks connected. Expected {}, found {}",
            total_player_count,
            minimal_paths.len()
        );
        std::process::exit(1);
    }

    let frozen_json = serde_json::to_string_pretty(&frozen).unwrap();
    std::fs::write(frozen_path, frozen_json).unwrap();

    use gilrs;
    let mut gilrs = gilrs::Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!(
            "{} is {:?} {}",
            gamepad.name(),
            gamepad.power_info(),
            gamepad.devpath()
        );
    }
    let mut joy_lookup: joypaths::EventPathLookup =
        joypaths::repath_joys(&words, &config).unwrap().into();
    loop {
        // Examine new events
        let event = gilrs.next_event();
        match &event {
            Some(gilrs::Event {
                event: gilrs::EventType::Connected | gilrs::EventType::Disconnected,
                ..
            }) => {
                joy_lookup = joypaths::repath_joys(&words, &config).unwrap().into();
            }
            _ => {}
        }

        if let Some(gilrs::Event { id, event, time }) = event {
            let gp = gilrs.gamepad(id);
            let devpath = gp.devpath();
            let common_name = joy_lookup.0[devpath].common_name.clone();
            println!("{:?} New event from {}: {:?}", time, common_name, event);
        }
    }
}

////Exit with code 1
//std::process::exit(1);
//

//let input_joysticks_vector_of_vectors = {
//let mut input_vec_of_vecs = vec![];
//for _ in 0..number_of_output_controllers {
//input_vec_of_vecs.push(Vec::new())
//}
//let mut output_joystick_index = 0;
//let mut num_assigned_to_current_output_index = 0;
//let mut mayflash_count = 0;
//for i in 0..num_inputs_detected {
//if sdl2_input_joystick_subsystem
//.name_for_index(i)?
//.contains("MAYFLASH")
//{
//if mayflash_count >= args.mayflash_controllers {
//continue;
//} else {
//mayflash_count += 1;
//}
//}
//let current_in_joy = sdl2_input_joystick_subsystem.open(i)?;
//let data = current_in_joy.guid().raw().data;

//// Calculate sha256 for the data
//let mut hasher = sha2::Sha256::new();
//hasher.update(data);
//let by = current_in_joy.instance_id().to_le_bytes();
//hasher.update(by);
//let result = hasher.finalize();
//println!("GUID: {}", current_in_joy.guid());
//let num = result[0] as u16;
//let num = num << 3;
//let num = num | ((result[1] & 0b00000111) as u16);
//println!("myname: {}", english_words[num as usize]);

//input_vec_of_vecs[output_joystick_index].push(current_in_joy);
//num_assigned_to_current_output_index += 1;

//// Test: should we move on and start assigning joysticks for the next output contorller?
//if {
//let currently_assigning_last_output =
//output_joystick_index >= (number_of_output_controllers - 1) as usize;
//let minimim_joysticks_per_output_controller = ((num_inputs_detected as f32)
//// (number_of_output_controllers as f32))
//.floor() as i32;
//num_assigned_to_current_output_index >= minimim_joysticks_per_output_controller
//&& !currently_assigning_last_output
//} {
//output_joystick_index += 1;
//num_assigned_to_current_output_index = 0;
//}
//}
//input_vec_of_vecs
//};

//let out_joysticks = {
//let mut out_joysticks = Vec::new();
//for i in 0..number_of_output_controllers {
//let name = format!("Buster{}", i);
//let outjoy = software_joystick::Joystick::new(name)?;
//out_joysticks.push(outjoy);
//}
//out_joysticks
//};

//loop {
//sdl2_input_joystick_subsystem.update();
//for curr_out_joy_index in 0..out_joysticks.len() {
//let out_joystick = &out_joysticks[curr_out_joy_index];
//let input_joystick_vector = &input_joysticks_vector_of_vectors[curr_out_joy_index];

//// Iterates over all of the axes we care about- each element of the NamedAxis enum
//for named_axis in NamedAxis::iter() {
//let (id, scalar_map) = snes_namedaxis_to_id_and_scalar(&named_axis);

//out_joystick.move_axis(
//{
//match named_axis {
//NamedAxis::Xright => software_joystick::Axis::X,
//NamedAxis::Yup => software_joystick::Axis::Y,
//}
//},
//{
//let sum: f32 = input_joystick_vector
//.iter()
//.map(|ijoy| {
//let raw_axis_value = ijoy.axis(id).unwrap() as f32;
//let normalized_axis_value = raw_axis_value / scalar_map;
//normalized_axis_value
//})
//.sum();
//let average: f32 = sum / (input_joystick_vector.len() as f32);
//let exponent_scaled =
//average.signum() * average.abs().powf(args.summed_axis_exponent);
//let out_scaled = exponent_scaled * 512.0;
//out_scaled.trunc() as i32
//},
//)?;
//}

//// Iterates over all the buttons we care about
//for named_button in NamedButton::iter() {
//out_joystick.button_press(
//match named_button {
//NamedButton::X => software_joystick::Button::RightNorth,
//NamedButton::A => software_joystick::Button::RightEast,
//NamedButton::B => software_joystick::Button::RightSouth,
//NamedButton::Y => software_joystick::Button::RightWest,
//NamedButton::L => software_joystick::Button::L1,
//NamedButton::R => software_joystick::Button::R1,
//NamedButton::Start => software_joystick::Button::RightSpecial,
//NamedButton::Select => software_joystick::Button::LeftSpecial,
//},
//{
//let sdl_inputs_button_id = snes_namedbutton_to_id(&named_button);
//let in_joys_to_check = {
//if args.p1_controller1_buttons_always_down
//&& curr_out_joy_index == 0
//&& input_joystick_vector.len() > 1
//{
//&input_joystick_vector[1..]
//} else {
//&input_joystick_vector
//}
//};
//let is_pressed = in_joys_to_check
//.iter()
//.all(|ijoy| ijoy.button(sdl_inputs_button_id).unwrap());
//is_pressed
//},
//)?;
//}
//out_joystick.synchronise()?;
//}
//}

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
//}

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
