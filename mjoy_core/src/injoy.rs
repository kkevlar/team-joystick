use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum NamedButton {
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
pub enum NamedAxis {
    Xright,
    Yup,
}

pub fn snes_namedaxis_to_id_and_scalar(a: &NamedAxis) -> (u32, f32) {
    use NamedAxis::*;
    match a {
        Xright => (0, 32767.0),
        Yup => (1, -32767.0),
    }
}

pub fn snes_namedbutton_to_id(b: &NamedButton) -> gilrs::Button {
    use gilrs::Button;
    use NamedButton::*;
    match b {
        X => Button::North,
        A => Button::East,
        B => Button::South,
        Y => Button::West,
        Start => Button::Start,
        Select => Button::Select,
        L => Button::LeftTrigger,
        R => Button::RightTrigger,
    }
}
