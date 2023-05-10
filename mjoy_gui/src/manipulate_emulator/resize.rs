use regex::Regex;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResizeError {
    #[error("Failed to list the windows ")]
    FailedList,
    #[error("No match from windows based on provided pattern")]
    NoMatch,
    #[error("Failed to capture from the matched line")]
    FailedCapture,
    #[error("Failed to resize the window")]
    FailedResize,
    #[error("Failed to focus the window")]
    FailedFocus,
}

fn give_matching_wmctl_l(regex: &Regex) -> Result<String, ResizeError> {
    let output = Command::new("wmctrl")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        return Err(ResizeError::FailedList);
    }
    let mut my_match = None;
    for line in String::from_utf8(output.stdout).unwrap().lines() {
        if regex.is_match(line) {
            my_match = Some(line.to_owned());
        }
    }

    my_match.ok_or(ResizeError::NoMatch)
}

fn give_capture_for_wmctrl(regex: &Regex) -> Result<String, ResizeError> {
    let capture_regex = Regex::new(r"0x[0-9a-f]+\s+[0-9]\s+[^ ]+\s+(.{30})").unwrap();

    let line = give_matching_wmctl_l(regex)?;
    Ok(capture_regex
        .captures(&line)
        .ok_or(ResizeError::FailedCapture)?
        .get(1)
        .ok_or(ResizeError::FailedCapture)?
        .as_str()
        .to_owned())
}

pub fn resize_and_focus_matching(match_regex: &Regex) -> Result<(), ResizeError> {
    let capture = give_capture_for_wmctrl(match_regex)?;

    let output = Command::new("wmctrl")
        .arg("-v")
        .arg("-r")
        .arg(&capture)
        .arg("-e")
        .arg("0,300,0,1300,1080")
        .output()
        .map_err(|_| ResizeError::FailedResize)?;
    if !output.status.success() {
        return Err(ResizeError::FailedResize);
    }

    let output = Command::new("wmctrl")
        .arg("-v")
        .arg("-a")
        .arg(&capture)
        .output()
        .map_err(|_| ResizeError::FailedFocus)?;
    if !output.status.success() {
        return Err(ResizeError::FailedFocus);
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    //use super::*;

    //#[test]
    //fn wm_simple() -> Result<(), ResizeError> {
    //assert_eq!(
    //"Dolphin 5.0 | JIT64 DC | OpenG",
    //give_capture_for_wmctrl(&Regex::new(r"Dolphin.*FPS").unwrap())?
    //);
    //Ok(())
    //}
    //#[test]
    //fn wm_all() -> Result<(), ResizeError> {
    //resize_and_focus_matching(&Regex::new(r"Dolphin.*FPS").unwrap())
    //}
}
