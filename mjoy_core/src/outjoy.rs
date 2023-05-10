use crate::joypaths;
use crate::Team;
use crate::TeamLock;
use gilrs;
use software_joystick::*;
use strum::IntoEnumIterator;

pub struct Outjoys<'a> {
    pub outjoys: Vec<Outjoy<'a>>,
}

pub struct Outjoy<'a> {
    team: &'a Team,
    joy: Joystick,
}

fn inbutton_to_outbutton(b: &crate::injoy::NamedButton) -> software_joystick::Button {
    use crate::injoy::NamedButton;
    match b {
        NamedButton::X => software_joystick::Button::RightNorth,
        NamedButton::A => software_joystick::Button::RightEast,
        NamedButton::B => software_joystick::Button::RightSouth,
        NamedButton::Y => software_joystick::Button::RightWest,
        NamedButton::L => software_joystick::Button::L1,
        NamedButton::R => software_joystick::Button::R1,
        NamedButton::Start => software_joystick::Button::RightSpecial,
        NamedButton::Select => software_joystick::Button::LeftSpecial,
    }
}

fn inaxis_to_outaxis(a: &crate::injoy::NamedAxis) -> software_joystick::Axis {
    use crate::injoy::NamedAxis;
    match a {
        NamedAxis::Xright => software_joystick::Axis::X,
        NamedAxis::Yup => software_joystick::Axis::Y,
    }
}

impl<'a> Outjoy<'a> {
    pub fn new(team: &'a Team, index: u32) -> Self {
        let joy = Joystick::new(format!("Buster{}", index)).unwrap();
        Self { team, joy }
    }

    fn update_axes<'b, 'c, 'd>(&'a self, context: &'d UpdateContext<'b, 'c>) {
        use crate::injoy::NamedAxis;

        for (i, inaxis) in crate::injoy::NamedAxis::iter().enumerate() {
            let mut sum = 0 as f32;
            let mut count = 0;

            let out_axis = inaxis_to_outaxis(&inaxis);

            for (_id, gamepad) in context.gilrs.gamepads() {
                let devpath = gamepad.devpath();
                let namedpath = &context.event_path_lookup.0.get(devpath);
                if namedpath.is_none() {
                    continue;
                }
                let namedpath = namedpath.unwrap();
                let common_name = &namedpath.common_name;

                if self.team.players.contains(&common_name) {
                    let (axis_id, scalar) = crate::injoy::snes_namedaxis_to_id_and_scalar(&inaxis);
                    let gilrs_axis = match axis_id {
                        0 => gilrs::Button::DPadRight,
                        1 => gilrs::Button::DPadUp,
                        _ => panic!("Invalid axis_id"),
                    };

                    let value = gamepad.button_data(gilrs_axis);
                    let value = match value {
                        Some(value) => {
                            let vv = value.value();
                            let vvv = match vv {
                                v if v < 0.1 => -1,
                                v if v > 0.9 => 1,
                                _ => 0,
                            } as f32;
                            vvv
                        }
                        None => 0 as f32,
                    };
                    let value = value * scalar.signum();
                    sum += value;
                    count += 1;
                }
            }

            let average = match count {
                0 => panic!("No players found for team"),
                _ => sum / count as f32,
            };

            let average = (average * 512f32) as i32;
            self.joy.move_axis(out_axis, average).unwrap();
        }
    }

    fn update_buttons<'b, 'c, 'd>(&'a self, context: &'d UpdateContext<'b, 'c>) {
        use crate::injoy::NamedButton;

        for (i, inbutton) in crate::injoy::NamedButton::iter().enumerate() {
            let mut sum = 0 as f32;
            let mut count = 0;

            let outbutton = inbutton_to_outbutton(&inbutton);

            for (_id, gamepad) in context.gilrs.gamepads() {
                let devpath = gamepad.devpath();
                let namedpath = &context.event_path_lookup.0.get(devpath);
                if namedpath.is_none() {
                    continue;
                }
                let namedpath = namedpath.unwrap();
                let common_name = &namedpath.common_name;

                if self.team.players.contains(&common_name) {
                    let button_id: gilrs::Button = crate::injoy::snes_namedbutton_to_id(&inbutton);
                    let value = gamepad.button_data(button_id);
                    let value = match value {
                        Some(value) => {
                            let vv = value.value();
                            let vvv = match vv {
                                v if v < 0.1 => 0,
                                v if v > 0.9 => 1,
                                _ => 0,
                            } as f32;
                            vvv
                        }
                        None => 0 as f32,
                    };

                    sum += value;
                    count += 1;
                }
            }

            let average = match count {
                0 => panic!("No players found for team"),
                _ => sum / count as f32,
            };

            self.joy.button_press(outbutton, average > 0.8f32).unwrap();
        }
    }

    pub fn update<'b, 'c, 'd>(&'a self, context: &'d UpdateContext<'b, 'c>) {
        self.update_axes(context);
        self.update_buttons(context);
        self.joy.synchronise().unwrap();
    }
}

pub struct UpdateContext<'b, 'c> {
    pub event_path_lookup: &'b joypaths::EventPathLookup,
    pub gilrs: &'c mut gilrs::Gilrs,
}

impl<'a> Outjoys<'a> {
    pub fn new(tl: &'a TeamLock) -> Self {
        let mut outjoys = Vec::new();
        for team in tl.teams.iter() {
            outjoys.push(Outjoy::new(team, team.out_index));
        }
        Self { outjoys }
    }

    pub fn update<'b, 'c, 'd>(&'a self, context: &'d UpdateContext<'b, 'c>) {
        for outjoy in self.outjoys.iter() {
            outjoy.update(context);
        }
    }
}
