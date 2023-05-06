use mjoy_common::gui::{
    self,
    feedback_info::{Buttons, SticksAnalog, SticksDigital},
};
use regex::Regex;
use sha2::Digest;

fn team_options() -> Vec<String> {
    let paths = std::fs::read_dir("./resources/images").unwrap();
    let re = Regex::new(r"images/(.*)\.jpg").expect("Compile regex");
    let mut teams = Vec::new();

    for path in paths {
        let path = path.expect("Path conversion failed").path();
        let full_path = path.to_str().expect("Path tostring failed");
        if re.is_match(&full_path) {
            let tn = re
                .captures(&full_path)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            teams.push(tn);
        }
    }
    teams
}

fn main() {
    let team_options = team_options();

    let seed = 14;
    let teams = {
        let mut teams = Vec::new();
        for i in 0..4 {
            let mut sha = sha2::Sha256::new();
            sha.update((seed as i32).to_be_bytes());
            sha.update((i as i32).to_be_bytes());
            let result = sha.finalize();
            let num = result[0] as u16;
            let num = num << 8 | result[1] as u16;
            teams.push(team_options[num as usize % team_options.len()].clone());
        }
        teams
    };

    let mut gui = gui::Ui::new(&teams.as_slice(), gui::WidthHeight::new(1920, 1080));

    let wh = mjoy_common::wordhash::Wordhash::new(seed, seed);

    let fb = gui::feedback_info::FeedbackInfo {
        teams: teams
            .iter()
            .enumerate()
            .map(|(outer, n)| gui::feedback_info::Team {
                team_name: n,
                players: {
                    let players = (0..4)
                        .map(|i| {
                            let i = i as u32;
                            let i = i + 9 * outer as u32;
                            let mut by = i.to_be_bytes().to_vec();
                            let name = wh.object_name(&mut by, 13);
                            gui::feedback_info::Player {
                                player_name: name,
                                button: gui::feedback_info::Buttons::default(),
                                sticks: SticksDigital::default(),
                            }
                        })
                        .collect();
                    players
                },
                button: Buttons::default(),
                sticks: SticksAnalog::default(),
            })
            .collect(),
    };

    loop {
        gui.render(&fb);
    }
}
