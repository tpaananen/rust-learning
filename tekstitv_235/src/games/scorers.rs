use colored::Colorize;
use crate::{regex_factory::RegexFactory, constants::COL_WIDTH_HOME};

struct Scorer {
    name: String,
    is_finnish_player: bool,
    is_overtime: bool
}

impl Scorer {
    fn new(line: &str, regex_factory: &RegexFactory, finnish_players: &Vec<String>, is_on_overtime: bool) -> Self {
        let is_finnish_player = is_finnish_player(line, finnish_players);
        let is_overtime = is_on_overtime && regex_factory.regex_overtime_goal.is_match(line);
        Scorer { name: line.to_owned(), is_finnish_player, is_overtime}
    }

    fn to_string(&self) -> String {
        if self.is_overtime {
            format!("{name}", name = self.name.bright_magenta())
        } else if self.is_finnish_player {
            format!("{name}", name = self.name.bright_green())
        } else {
            format!("{name}", name = self.name.bright_cyan())
        }
    }
}

fn is_finnish_player(line: &str, finnish_players: &Vec<String>) -> bool {
    line.len() > 2
        && (line.starts_with("(")
            || line.starts_with(" (")
            || finnish_players.iter().any(|p| line.contains(p))
            || line.contains("torjunta"))
}

pub struct Scorers {
    scorers: Vec<(Scorer, Scorer)>
}

impl Scorers {
    pub(super) fn from_lines(lines: &Vec<&str>, regex_factory: &RegexFactory, finnish_players: &Vec<String>, is_on_overtime: bool, line_number: &mut usize) -> Self {
        Scorers { scorers: parse_scores(lines, regex_factory, finnish_players, is_on_overtime, line_number) }
    }

    pub(super) fn print(&self) {
        for (home, away) in &self.scorers {
            println!("{home_score} {away_score}", home_score = home.to_string(), away_score = away.to_string());
        }
    }
}

fn parse_scores(lines: &Vec<&str>, regex_factory: &RegexFactory, finnish_players: &Vec<String>, is_on_overtime: bool, line_number: &mut usize) -> Vec<(Scorer, Scorer)> {
    let mut scorers: Vec<(Scorer, Scorer)> = Vec::new();
    for line in lines.iter().skip(*line_number) {
        let home_scorer: String = line.chars().take(COL_WIDTH_HOME + 2).collect();
        let away_scorer: String = line.chars().skip(COL_WIDTH_HOME + 3).collect();
        scorers.push((
            Scorer::new(&home_scorer, regex_factory, finnish_players, is_on_overtime),
            Scorer::new(&away_scorer, regex_factory, finnish_players, is_on_overtime)
        ));
    }
    *line_number += scorers.len();
    scorers
}
