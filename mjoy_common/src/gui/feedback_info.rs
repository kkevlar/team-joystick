#[derive(Clone)]
pub struct FeedbackInfo<'a> {
    pub teams: Vec<Team<'a>>,
}
#[derive(Clone)]
pub struct Player {
    pub player_name: String,
    pub feedback: Presses,
}
#[derive(Clone)]
pub enum PressState {
    Pressed,
    Unpressed,
}
#[derive(Clone)]
pub struct ButtonPress {
    pub button: String,
    pub state: PressState,
}
#[derive(Clone)]
pub struct Presses(pub Vec<ButtonPress>);

#[derive(Clone)]
pub struct Team<'a> {
    pub team_name: &'a str,
    pub players: Vec<Player>,
    pub feedback: Presses,
}
