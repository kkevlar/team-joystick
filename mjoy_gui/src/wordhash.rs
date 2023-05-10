use sha2;
use sha2::Digest;

fn get_hash_integers(input: &[u8], salt: u32) -> (u16, u16) {
    let mut sha = sha2::Sha256::new();
    sha.update(input);
    sha.update(&salt.to_be_bytes());
    let num = sha.finalize();
    let mut num1 = num[0] as u16;
    num1 = num1 << 8;
    num1 = num1 | num[1] as u16;
    let mut num2 = num[2] as u16;
    num2 = num2 << 8;
    num2 = num2 | num[3] as u16;
    (num1, num2)
}

const TEAMS_PATH: &'static str = "./resources/teams.txt";
const ADJECTIVES_PATH: &'static str = "./resources/adjectives.txt";
const NOUNS_PATH: &'static str = "./resources/nouns.txt";

struct Words {
    adjectives: Vec<String>,
    nouns: Vec<String>,
    teams: Vec<String>,
    noun_salt: u32,
    team_salt: u32,
}

fn give_word(index: u16, list: &Vec<String>) -> String {
    let index = index % (list.len() as u16);
    list[index as usize].clone()
}

pub struct Wordhash(Words);

impl Wordhash {
    pub fn new(noun_salt: u32, team_salt: u32) -> Wordhash {
        let adjectives = std::fs::read_to_string(ADJECTIVES_PATH).unwrap();
        let adjectives: Vec<String> = adjectives.lines().map(|s| s.to_string()).collect();
        let nouns = std::fs::read_to_string(NOUNS_PATH).unwrap();
        let nouns: Vec<String> = nouns.lines().map(|s| s.to_string()).collect();
        let teams = std::fs::read_to_string(TEAMS_PATH).unwrap();
        let teams: Vec<String> = teams.lines().map(|s| s.to_string()).collect();
        Wordhash(Words {
            adjectives,
            nouns,
            teams,
            noun_salt,
            team_salt,
        })
    }

    pub fn object_name(&self, input: &mut Vec<u8>, max_length: u32) -> String {
        loop {
            let (aindex, nindex) = get_hash_integers(input, self.0.noun_salt);
            let candidate = format!(
                "{}{}",
                give_word(aindex, &self.0.adjectives),
                give_word(nindex, &self.0.nouns)
            );
            if candidate.len() as u32 <= max_length {
                return candidate;
            }
            input.push(11);
        }
    }

    pub fn team_name(&self, input: &[u8]) -> String {
        let (aindex, tindex) = get_hash_integers(input, self.0.team_salt);
        let candidate = format!(
            "{} {}",
            give_word(aindex, &self.0.adjectives),
            give_word(tindex, &self.0.teams)
        );
        candidate
    }
}
