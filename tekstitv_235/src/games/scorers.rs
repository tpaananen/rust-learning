use colored::Colorize;
use crate::{regex_factory::RegexFactory, constants::COL_WIDTH_HOME};

struct Scorer {
    name: String,
    time: String,
    is_finnish_player: bool,
    is_overtime: bool
}

impl Scorer {
    fn new(line: &str, regex_factory: &RegexFactory, finnish_players: &Vec<String>) -> Self {
        let name = if line.len() > 2 { line[..line.len() - 2].to_owned() } else { "".to_owned() };
        let is_finnish_player = name.len() > 2 && (name.starts_with("(") || name.starts_with(" (") || finnish_players.iter().any(|p| { name.contains(p) }));
        let time = if line.len() > 2 { line[line.len() - 2..].to_owned() } else { "".to_owned() };
        let is_overtime = regex_factory.regex_overtime_goal.is_match(line);
        Scorer { name, time, is_finnish_player, is_overtime}
    }

    fn to_string(&self) -> String {
        if self.is_overtime {
            format!("{}{}", self.name.bright_magenta(), self.time.bright_magenta())
        } else if self.is_finnish_player {
            format!("{}{}", self.name.bright_green(), self.time.bright_green())
        } else {
            format!("{}{}", self.name.bright_cyan(), self.time.bright_cyan())
        }
    }
}


pub struct Scorers {
    scorers: Vec<(Scorer, Scorer)>
}

impl Scorers {
    pub(super) fn from_lines(lines: &Vec<&str>, regex_factory: &RegexFactory, finnish_players: &Vec<String>, line_number: &mut usize) -> Self {
        Scorers { scorers: parse_scores(lines, regex_factory, finnish_players, line_number) }
    }

    pub(super) fn print(&self) {
        for (home, away) in &self.scorers {
            println!("{} {}", home.to_string(), away.to_string());
        }
    }
}

fn parse_scores(lines: &Vec<&str>, regex_factory: &RegexFactory, finnish_players: &Vec<String>, line_number: &mut usize) -> Vec<(Scorer, Scorer)> {
    let mut scorers: Vec<(Scorer, Scorer)> = Vec::new();
    for line in lines.iter().skip(*line_number) {
        let home_scorer: String = line.chars().take(COL_WIDTH_HOME + 2).collect();
        let away_scorer: String = line.chars().skip(COL_WIDTH_HOME + 3).collect();
        scorers.push((
            Scorer::new(&home_scorer, regex_factory, finnish_players),
            Scorer::new(&away_scorer, regex_factory, finnish_players)
        ));
    }
    *line_number += scorers.len();
    scorers
}
