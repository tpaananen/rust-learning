use colored::Colorize;

use crate::{regex_factory::RegexFactory, constants::{COL_WIDTH_HOME, COL_WIDTH_AWAY}};

struct Scorer {
    name: String,
    is_assistant: bool
}

impl Scorer {
    fn new(line: &str) -> Self {
        Scorer { name: line.to_owned(), is_assistant: line.starts_with("(") }
    }

    fn to_string(&self, width: usize) -> String {
        // TODO: color -> overtime (magenta)
        // TODO 2: is finnish goal scorer
        if self.is_assistant {
            format!("{}", self.name.bright_green())
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
            println!(" {} {}", home.to_string(COL_WIDTH_HOME - 1), away.to_string(COL_WIDTH_AWAY - 1));
        }
    }
}

fn parse_scores(lines: &Vec<&str>, regex_factory: &RegexFactory, line_number: &mut usize) -> Vec<(Scorer, Scorer)> {
    let mut scorers: Vec<(Scorer, Scorer)> = Vec::new();
    for line in lines.iter().skip(*line_number) {
        let home_scorer: String = line.chars().take(COL_WIDTH_HOME + 1).collect();
        let away_scorer: String = line.chars().skip(COL_WIDTH_HOME + 2).take(COL_WIDTH_HOME + 2).collect();
        scorers.push((Scorer::new(&home_scorer), Scorer::new(&away_scorer)));
    }
    *line_number += scorers.len();
    scorers
}
