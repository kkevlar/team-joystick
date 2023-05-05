use kiss3d::light::Light;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};
use kiss3d::window::Window;

use crate::wordhash;

use self::feedback_info::FeedbackInfo;
use self::team_color::{ColoredTeam, HintedColors};

pub mod feedback_info;
mod team_color;

struct Hz(f32);
impl Hz {
    fn new(hz: f32) -> Hz {
        Hz(hz)
    }
    fn apply(&self, duration: &std::time::Duration) -> f32 {
        (duration.as_secs_f32() * self.0 * 3.14159265).cos()
    }
}

pub struct Ui {
    window: Window,
    teams: Vec<String>,
    logos: Vec<kiss3d::scene::PlanarSceneNode>,
    logos_locations: Vec<kiss3d::nalgebra::Translation2<f32>>,
    hc: HintedColors,
    font: std::rc::Rc<kiss3d::text::Font>,
}

impl Ui {
    pub fn new(teams: &[String]) -> Self {
        let mut window = Window::new_with_size("Cool project", 1920, 1080);
        window.set_background_color(0.1, 0.1, 0.1);
        window.set_light(Light::StickToCamera);

        let mut logos: Vec<_> = Vec::new();
        let mut trans: Vec<_> = Vec::new();
        for (i, team) in teams.iter().enumerate() {
            let mut r = window.add_rectangle(220.0, 220.0);
            let translate = &kiss3d::nalgebra::Translation2::new(
                845.0 * if i < 2 { -1 } else { 1 } as f32,
                250.0 * if i % 2 == 0 { 1 } else { -1 } as f32 + 150f32,
            );
            r.append_translation(translate);
            trans.push(translate.to_owned());
            let path = format!("./resources/images/{}.jpg", team);
            r.set_texture_from_file(std::path::Path::new(&path), team);
            logos.push(r);
        }

        let mut ui = Ui {
            window,
            teams: teams.iter().map(|t| t.to_string()).collect(),
            logos,
            logos_locations: trans,
            hc: HintedColors::new(),
            font: kiss3d::text::Font::new(std::path::Path::new("./resources/impact.ttf")).unwrap(),
        };
        ui
    }

    pub fn render(&mut self, feedback: &FeedbackInfo) {
        let teams: Vec<team_color::Team> = self.teams.iter().map(|n| team_color::Team(n)).collect();
        let colors_to_use = self.hc.color_teams(&teams);

        for (_, c) in colors_to_use.iter().enumerate() {
            let idx = self.teams.iter().position(|t| t == c.team.0).unwrap();
            //assert_eq!(feedback.teams[0].team_name, c.team.0);

            for i in 0..5 {
                let text = if i > 0 {
                    &feedback.teams[idx].players[i - 1].player_name
                } else {
                    c.team.0
                };

                self.window.draw_text(
                    text,
                    &kiss3d::nalgebra::Point2::new(
                        1920f32
                            + -300f32
                            + if i == 0 { 0f32 } else { 100f32 }
                            + (self.logos_locations[idx].x)
                                * (1.90f32 + if i == 0 { 0f32 } else { 0.1f32 }),
                        1080f32
                            + 150f32
                            + (self.logos_locations[idx].y) * -2f32
                            + 120f32 * i as f32,
                    ),
                    if i == 0 { 100.0 } else { 90f32 },
                    &self.font,
                    &c.color.0,
                );
            }
        }
        self.window.render();
    }
}

//pub fn do_cubes() {
////env_logger::init();

//let mut cubes = Vec::new();
//for i in 0..5 {
//cubes.push(window.add_cube(1.0, 1.0, 1.0));
//}

//let len = cubes.len();
//for (i, cube) in cubes.iter_mut().enumerate() {
//let diff = (i as f32 - (len as f32 / 2f32)) as f32;
//let diff = diff * 1f32;
//let tx = kiss3d::nalgebra::Translation3::new(0f32, diff, 10.0);
//cube.append_translation(&tx);
//cube.set_color(1.0, diff, 0.0);
//}

//let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

////crate::manipulate_emulator::mute::mute("dolphin-emu");
////crate::manipulate_emulator::resize::resize_and_focus_matching(
////&regex::Regex::new(r"[Dd]olphin.*FPS").unwrap(),
////);

//// Record start time
//let start_time = std::time::SystemTime::now();

//let square_lateral_hz = Hz::new(1f32);

//let font = kiss3d::text::Font::new(std::path::Path::new("./impact.ttf")).unwrap();

//let wh = wordhash::Wordhash::new(134, 134);
//let team_names: Vec<String> = (0i32..10i32)
//.map(|i| wh.team_name(&i.to_be_bytes()))
//.collect();
//let teams: Vec<team_color::Team> = team_names.iter().map(|n| team_color::Team(n)).collect();
//let hc = team_color::HintedColors::new();
//let colors_to_use = hc.color_teams(&teams.as_slice());

//while window.render() {
//for (num, c) in colors_to_use.iter().enumerate() {
//window.draw_text(
//c.team.0,
//&kiss3d::nalgebra::Point2::new(10f32, 10f32 + 200f32 * num as f32),
//110.0,
//&font,
//&c.color.0,
//);
//window.draw_text(
//"<",
//&kiss3d::nalgebra::Point2::new(10f32, 90f32 + 200f32 * num as f32),
//90.0,
//&font,
//&c.color.0,
//);
//window.draw_text(
//if num % 2 == 0 { "ABXYLR" } else { "A XY R" },
//&kiss3d::nalgebra::Point2::new(80f32, 90f32 + 200f32 * num as f32),
//90.0,
//&font,
//&c.color.0,
//);
//}
//for num in 0..10 {
//window.draw_text(
//"Hello birds!",
//&kiss3d::nalgebra::Point2::new(3400f32, 10f32 + 200f32 * num as f32),
//110.0,
//&font,
//&kiss3d::nalgebra::Point3::new(0.6, 0f32, 0.9),
//);
//}
//let duration = start_time.elapsed().unwrap();

//for (i, cube) in cubes.iter_mut().enumerate() {
//let diff = (i as f32 - (len as f32 / 2f32)) as f32;
//let diff = diff * 1f32;

//cube.set_local_translation(kiss3d::nalgebra::Translation3::new(
//square_lateral_hz.apply(&duration) * 10f32,
//diff,
//10.0,
//));
//cube.prepend_to_local_rotation(&rot);
//}
//}
//}
