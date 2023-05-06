use kiss3d::nalgebra::Point3;

pub struct HintedColor {
    color: Color,
    hints: Vec<&'static str>,
}

pub struct HintedColors {
    elements: Vec<HintedColor>,
}

#[derive(Debug)]
struct Score {
    points: i32,
    index: usize,
}

#[derive(Clone, Debug)]
pub struct Team<'a>(pub &'a str);

#[derive(Debug, PartialEq, Clone)]
pub struct Color(pub Point3<f32>);

#[derive(Debug)]
pub struct ColoredTeam {
    pub team: String,
    pub color: Color,
}

#[derive(Debug)]
pub struct ColoredTeams(pub Vec<ColoredTeam>);

fn calc_score_points(team: &Team, hc: &HintedColor) -> i32 {
    const START_POINTS: i32 = 100;
    for (count, hint) in hc.hints.iter().enumerate() {
        if team.0.to_lowercase().contains(hint) {
            return START_POINTS - count as i32;
        }
    }
    0
}

impl HintedColors {
    pub fn new() -> Self {
        let mut temp = vec![
            HintedColor {
                color: Color(Point3::new(0.7, 0.15, 0.15)),
                hints: vec![
                    "red", "burgandy", "hot", "sand", "rock", "maple", "cursed", "volcano",
                ],
            },
            HintedColor {
                color: Color(Point3::new(0f32 / 255f32, 179f32 / 255f32, 179f32 / 255f32)),
                hints: vec![
                    "cyan", "teal", "water", "carp", "fish", "mermaid", "whals", "lake",
                ],
            },
            HintedColor {
                color: Color(Point3::new(0.1, 0.6, 0.1)),
                hints: vec![
                    "green", "lime", "olive", "lumber", "broccoli", "liberty", "freedom", "venom",
                    "gator", "celtic",
                ],
            },
            HintedColor {
                color: Color(Point3::new(
                    192f32 / 255f32,
                    191f32 / 255f32,
                    191f32 / 255f32,
                )),
                hints: vec!["grey", "silver", "metal", "tin", "iron", "rhino", "blessed"],
            },
            HintedColor {
                color: Color(Point3::new(180.0 / 255.0, 70.0 / 255.0, 0.05)),
                hints: vec![
                    "orange", "rust", "copper", "hot", "maple", "burnt", "shrimp", "cursed",
                    "volcano",
                ],
            },
            HintedColor {
                color: Color(Point3::new(160f32 / 255f32, 170f32 / 255f32, 0.1)),
                hints: vec![
                    "yellow",
                    "lemon",
                    "gold",
                    "shiny",
                    "olive",
                    "gild",
                    "lightning",
                ],
            },
            HintedColor {
                color: Color(Point3::new(51f32 / 255f32, 51f32 / 255f32, 0.8f32)),
                hints: vec!["blue", "navy", "denim", "royal", "lake"],
            },
            HintedColor {
                color: Color(Point3::new(
                    225f32 / 255f32,
                    41f32 / 255f32,
                    190f32 / 255f32,
                )),
                hints: vec!["pink", "jolly", "melon", "pig", "blush"],
            },
            HintedColor {
                color: Color(Point3::new(
                    153f32 / 255f32,
                    51f32 / 255f32,
                    255f32 / 255f32,
                )),
                hints: vec!["purple", "royal", "roman", "king", "queen", "silly"],
            },
            HintedColor {
                color: Color(Point3::new(92f32 / 255f32, 92f32 / 255f32, 138f32 / 255f32)),
                hints: vec![
                    "dark",
                    "invisible",
                    "old",
                    "black",
                    "smoke",
                    "demon",
                    "cursed",
                    "evil",
                    "pirate",
                    "mys",
                ],
            },
            HintedColor {
                color: Color(Point3::new(153f32 / 255f32, 102f32 / 255f32, 0f32 / 255f32)),
                hints: vec![
                    "brown", "beige", "bronze", "dust", "potato", "sand", "rhino", "carmel",
                ],
            },
        ];
        Self { elements: temp }
    }

    fn high_score_team<'a, 'b, 'c>(&self, team: &'b Team<'a>, used_colors: &'c [usize]) -> Score {
        let mut score: Option<Score> = None;

        for (index, hc) in self.elements.iter().enumerate() {
            if used_colors.contains(&index) {
                continue;
            }
            let candidate = Score {
                points: calc_score_points(team, hc),
                index,
            };

            score = match score {
                Some(s) => {
                    if s.points >= candidate.points {
                        Some(s)
                    } else {
                        Some(candidate)
                    }
                }
                None => Some(candidate),
            }
        }

        let mut score = score.unwrap();

        if &score.points == &0 {
            use sha2::Digest;

            for i in 0..10 {
                let mut sha = sha2::Sha256::new();
                sha.update(&"color_salt_woof".as_bytes());
                sha.update(team.0.as_bytes());
                let ii = i as u32;
                sha.update(ii.to_be_bytes());
                let result = sha.finalize();
                let num = (result[0] as i32) % (self.elements.len() as i32);
                let num = num as usize;
                if !used_colors.contains(&num) {
                    score = Score {
                        points: 10 - i,
                        index: num,
                    };
                    break;
                }
            }
        }

        //println!("My best score for {:?} is {:?}", team, score);
        score
    }

    pub fn color_teams<'a>(&self, init_teams: &[Team<'a>]) -> ColoredTeams {
        let mut used_colors: Vec<usize> = Vec::new();
        let mut teams_to_color = init_teams.to_vec();
        let mut colored_teams = Vec::new();

        for i in 0..init_teams.len() {
            let mut high_score: Option<(usize, Score)> = None;
            for (team_index, team) in teams_to_color.iter().enumerate() {
                let score = self.high_score_team(team, &used_colors.as_slice());
                high_score = match high_score {
                    Some(hs) => {
                        if hs.1.points >= score.points {
                            Some(hs)
                        } else {
                            Some((team_index, score))
                        }
                    }
                    None => Some((team_index, score)),
                }
            }

            let (team_index, high_score) = high_score.unwrap();
            colored_teams.push(ColoredTeam {
                team: teams_to_color[team_index].0.to_string(),
                color: self.elements[high_score.index].color.clone(),
            });
            used_colors.push(high_score.index);
            teams_to_color.remove(team_index);

            //println!(
            //"Locked it in {i} {:?}",
            //&colored_teams[colored_teams.len() - 1]
            //);
        }

        ColoredTeams(colored_teams)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn close_enough(a: &Color, b: &Color) -> bool {
        let a: Point3<f32> = a.0;
        let b: Point3<f32> = b.0;
        let d: f32 = kiss3d::nalgebra::distance(&a.xyz(), &b.xyz());
        return d < 0.01;
    }

    #[test]
    fn simple() {
        let team_names = [
            "Haunted Spiders",
            "Shiny Sillies",
            "woof woof woof",
            "Boiled Shrimp",
            "Prank candles",
            "Lime Gators",
            "Red Riders",
            "Royal Kings",
        ];
        let teams: Vec<Team> = team_names.iter().map(|n| Team(n)).collect();

        let answers = [-1, 5, -1, 4, -1, 2, 0, 8];

        let hc = HintedColors::new();

        let woof = hc.color_teams(&teams.as_slice());
        for c in woof.0.iter() {
            let index = team_names.iter().position(|tn| tn == &c.team).unwrap();
            let answer = answers[index];
            if answer >= 0 {
                let answer = answer as usize;
                dbg!((answer, &hc.elements[answer].color, c.color.0));
                assert!(close_enough(&hc.elements[answer].color, &c.color));
            }
        }
    }
}
