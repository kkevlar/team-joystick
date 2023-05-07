use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
