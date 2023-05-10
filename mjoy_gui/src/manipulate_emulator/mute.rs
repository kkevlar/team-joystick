use regex::Regex;
use std::process::Command;
use thiserror;

enum MuteUnmute {
    Mute,
    Unmute,
}

#[derive(Debug, thiserror::Error)]
pub enum MuteError {
    #[error("Failed to list the sinks ")]
    ListSinksFailed,
    #[error("I can't determine which source to mute")]
    TooManyNameMatches,
    #[error("I can't determine which source to mute")]
    NoNameMatches,
    #[error("Error parsing the input from the sink list cmd")]
    MismatchRegexes,
    #[error("The command to mute the sink failed EARLY: {0}")]
    MuteSinkFailed1(#[from] std::io::Error),
    #[error("The command to mute the sink failed EARLY ")]
    MuteSinkFailed,
}

#[derive(Debug, Eq, PartialEq)]
struct ExecAble<'a> {
    binary: &'a str,
    args: Vec<&'a str>,
}

fn mute_unmute_command<'a, 'b>(
    list_sinks_lines: &'a Vec<String>,
    name: &'b str,
    mute: MuteUnmute,
) -> Result<ExecAble<'a>, MuteError> {
    let index_regex = Regex::new(r"\s+index:\s+([0-9]+)").expect("Compile regex");
    let client_regex = Regex::new(r"\s+client:\s(.*)").expect("Compile regex");

    let mut index_matches: Vec<&str> = Vec::new();
    let mut client_matches: Vec<&str> = Vec::new();

    for line in list_sinks_lines.iter() {
        if index_regex.is_match(line) {
            index_matches.push(index_regex.captures(line).unwrap().get(1).unwrap().as_str());
        } else if client_regex.is_match(line) {
            client_matches.push(
                client_regex
                    .captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str(),
            );
        }
    }

    let name_lower = name.to_lowercase();
    let mut index = None;

    for (i, client) in client_matches.iter().enumerate() {
        if client.to_ascii_lowercase().contains(&name_lower) {
            match index {
                Some(_) => Err(MuteError::TooManyNameMatches),
                None => {
                    index = Some(i);
                    Ok(())
                }
            }?;
        }
    }

    match index {
        None => Err(MuteError::NoNameMatches),
        Some(i) => match index_matches.get(i) {
            None => Err(MuteError::MismatchRegexes),
            Some(s) => {
                use MuteUnmute::*;
                Ok(ExecAble {
                    binary: "pacmd",
                    args: vec![
                        "set-sink-input-mute",
                        s,
                        match mute {
                            Mute => "true",
                            Unmute => "false",
                        },
                    ],
                })
            }
        },
    }
}

fn do_it(name: &str, mute: MuteUnmute) -> Result<(), MuteError> {
    let output = Command::new("pacmd")
        .arg("list-sink-inputs")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        return Err(MuteError::ListSinksFailed);
    }
    let mut lines = Vec::new();
    for line in String::from_utf8(output.stdout).unwrap().lines() {
        lines.push(line.to_owned());
    }

    let ExecAble { binary, args } = mute_unmute_command(&lines, name, mute)?;

    let status = Command::new(binary)
        .args(args)
        .output()
        .map_err(MuteError::MuteSinkFailed1)?
        .status;

    if !status.success() {
        return Err(MuteError::MuteSinkFailed);
    }

    Ok(())
}

pub fn mute(name: &str) -> Result<(), MuteError> {
    do_it(name, MuteUnmute::Mute)
}
pub fn unmute(name: &str) -> Result<(), MuteError> {
    do_it(name, MuteUnmute::Unmute)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mute_unmute_string_simple() -> Result<(), MuteError> {
        use std::io::BufRead;

        let path = "src/manipulate_emulator/pulse_audio_list_example.txt";
        let file = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);

        let mut in_vec = Vec::new();
        for line in reader.lines() {
            in_vec.push(line.unwrap());
        }

        let result = mute_unmute_command(&in_vec, "Dolphin", MuteUnmute::Mute)?;
        assert_eq!(
            result,
            ExecAble {
                binary: "pacmd",
                args: vec!["set-sink-input-mute", "4", "true"],
            }
        );

        Ok(())
    }

    //#[test]
    //fn mute_chromium() -> Result<(), MuteError> {
    //unmute(&"dolphin-emu")
    //}
}
