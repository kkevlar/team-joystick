use sha2;

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

pub fn team_hash(team_salt: u32, data: &[u8]) -> String {
    let team_options = team_options();

    let seed = team_salt;

    let mut sha = sha2::Sha256::new();
    sha.update((seed as i32).to_be_bytes());
    sha.update(data);
    let result = sha.finalize();
    let num = result[0] as u16;
    let num = num << 8 | result[1] as u16;

    team_options[num as usize % team_options.len()].clone()
}
