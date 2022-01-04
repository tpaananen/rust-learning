use colored::Colorize;

use crate::{regex_factory::RegexFactory, constants::COL_WIDTH_HOME};

struct Scorer {
    name: String,
    is_assistant: bool,
    is_overtime: bool
}

impl Scorer {
    fn new(line: &str, regex_factory: &RegexFactory) -> Self {
        Scorer {
            name: line.to_owned(),
            is_assistant: line.starts_with("("),
            is_overtime: regex_factory.regex_overtime_goal.is_match(line)
        }
    }

    fn to_string(&self) -> String {
        // TODO 2: is finnish goal scorer
        if self.is_assistant {
            format!("{}", self.name.bright_green())
        } else if self.is_overtime {
            format!("{}", self.name.bright_magenta())
        } else {
            format!("{}", self.name.bright_cyan())
        }
    }
}


pub struct Scorers {
    scorers: Vec<(Scorer, Scorer)>
}

impl Scorers {
    pub(super) fn from_lines(lines: &Vec<&str>, regex_factory: &RegexFactory, line_number: &mut usize) -> Self {
        Scorers { scorers: parse_scores(lines, regex_factory, line_number) }
    }

    pub(super) fn print(&self) {
        for (home, away) in &self.scorers {
            println!(" {} {}", home.to_string(), away.to_string());
        }
    }
}

fn parse_scores(lines: &Vec<&str>, regex_factory: &RegexFactory, line_number: &mut usize) -> Vec<(Scorer, Scorer)> {
    let mut scorers: Vec<(Scorer, Scorer)> = Vec::new();
    for line in lines.iter().skip(*line_number) {
        let home_scorer: String = line.chars().take(COL_WIDTH_HOME + 1).collect();
        let away_scorer: String = line.chars().skip(COL_WIDTH_HOME + 2).take(COL_WIDTH_HOME + 2).collect();
        scorers.push((Scorer::new(&home_scorer, regex_factory), Scorer::new(&away_scorer, regex_factory)));
    }
    *line_number += scorers.len();
    scorers
}