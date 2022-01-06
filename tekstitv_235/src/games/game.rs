use colored::Colorize;
use crate::regex_factory::RegexFactory;
use super::{
    scorers::Scorers,
    teams::Teams
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameStatus {
    NotStarted,
    Started,
    Completed
}

impl GameStatus {
    pub(super) fn to_color(&self) -> &str {
        match self {
            GameStatus::Completed => "bright green",
            GameStatus::NotStarted => "bright white",
            GameStatus::Started => "bright yellow",
        }
    }
}

pub struct Game {
    status: GameStatus,
    period_results: Option<String>,
    teams: Teams,
    scorers: Scorers
}

impl Game {
    pub fn from_lines(lines: Vec<&str>, finnish_players: &Vec<String>, regex_factory: &RegexFactory) -> Option<Self> {
        let mut line_number: usize = 0;
        let period_results = parse_period_results(&lines, regex_factory, &mut line_number);
        let teams_opt = Teams::from_lines(&lines, &mut line_number);
        if teams_opt.is_none() {
            return None;
        }

        let teams = teams_opt.unwrap();
        let status = parse_status(&period_results, teams.get_result(), regex_factory);
        let scorers = Scorers::from_lines(&lines, &regex_factory, finnish_players, teams.get_is_overtime(), &mut line_number);
        Some(Game { status, period_results, teams, scorers })
    }

    pub fn get_home_team_name(&self) -> &str {
        self.teams.get_home_team_name()
    }

    pub fn get_status(&self) -> GameStatus {
        self.status
    }

    pub fn print(&self) {
        let res = self.period_results.as_ref();
        if res.is_some() {
            println!("{}", res.unwrap().yellow());
        }

        self.teams.print(&self.status);
        self.scorers.print();
    }
}

fn parse_period_results(lines: &Vec<&str>, regex_factory: &RegexFactory, line_number: &mut usize) -> Option<String> {
    if lines.len() > 0 {
        let regx = &regex_factory.regex_on_going_matches_by_time;
        let trimmed_line = lines[0].trim_start();
        if regx.is_match(trimmed_line) {
            *line_number += 1;
            return Some(trimmed_line.to_owned());
        }
    }

    None
}

fn parse_status(period_results: &Option<String>, result: &str, regex_factory: &RegexFactory) -> GameStatus {
    if period_results.is_some() {
        GameStatus::Started
    } else if regex_factory.regex_on_going_matches_by_time.is_match(result) {
        GameStatus::NotStarted
    } else {
        GameStatus::Completed
    }
}
