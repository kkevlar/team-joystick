pub struct FeedbackInfo<'a> {
    pub teams: Vec<Team<'a>>,
}

pub enum Analog {
    VeryLeft,
    Left,
    Neutral,
    Right,
    VeryRight,
}
pub enum DigitalLR {
    Left,
    Right,
    Neutral,
}
pub enum DigitalUD {
    Up,
    Down,
    Neutral,
}

pub struct SticksAnalog {
    pub l: Analog,
    pub u: DigitalUD,
}
impl SticksAnalog {
    pub fn default() -> Self {
        Self {
            l: Analog::Neutral,
            u: DigitalUD::Neutral,
        }
    }
}

pub struct SticksDigital {
    pub l: DigitalLR,
    pub u: DigitalUD,
}

impl SticksDigital {
    pub fn default() -> Self {
        Self {
            l: DigitalLR::Neutral,
            u: DigitalUD::Neutral,
        }
    }
}
pub struct Player {
    pub player_name: String,
    pub button: Buttons,
    pub sticks: SticksDigital,
}
pub enum ButtonPress {
    Pressed,
    Unpressed,
}
pub struct Buttons {
    pub a: ButtonPress,
    pub b: ButtonPress,
    pub x: ButtonPress,
    pub y: ButtonPress,
    pub l: ButtonPress,
    pub r: ButtonPress,
    pub t: ButtonPress,
    pub e: ButtonPress,
}
impl Buttons {
    pub fn default() -> Self {
        Buttons {
            a: ButtonPress::Unpressed,
            b: ButtonPress::Unpressed,
            x: ButtonPress::Unpressed,
            y: ButtonPress::Unpressed,
            l: ButtonPress::Unpressed,
            r: ButtonPress::Unpressed,
            t: ButtonPress::Unpressed,
            e: ButtonPress::Unpressed,
        }
    }
}

pub struct Team<'a> {
    pub team_name: &'a str,
    pub players: Vec<Player>,
    pub button: Buttons,
    pub sticks: SticksAnalog,
}
