use colored::Colorize;
use crate::{games::game::GameStatus, constants::{COL_WIDTH_HOME, COL_WIDTH_AWAY}};

pub struct Scorers {
    scorers: Vec<String>
}

impl Scorers {
    pub(super) fn new() -> Self {
        Scorers { scorers: Vec::new() }
    }
}

pub struct Teams {
    home_team: String,
    away_team: String,
    result: String
}

impl Teams {
    pub(super) fn from_lines(lines: &Vec<&str>, line_number: &mut usize) -> Self {
        let (home_team, away_team, result) = Teams::parse_teams_and_result(lines, line_number);
        Teams { home_team, away_team, result }
    }

    pub(super) fn get_home_team_name(&self) -> &str {
        &self.home_team
    }

    pub(super) fn get_result(&self) -> &str {
        &self.result
    }

    pub(super) fn print(&self, status: &GameStatus) {
        let color = status.to_color();
        print!("{:<home_width$}", self.home_team.color(color), home_width = COL_WIDTH_HOME);
        print!(" - ");
        print!("{:<away_width$}", self.away_team.color(color), away_width = COL_WIDTH_AWAY);
        println!("{:>result_width$}", self.result.color(color), result_width = 8);
    }

    fn parse_teams_and_result(lines: &Vec<&str>, line_number: &mut usize) -> (String, String, String) {
        let curr_line = *line_number;
        if lines.len() <= curr_line {
            panic!("Not enough lines to parse game, current line {}, lines provided: {}", curr_line, lines.len());
        }

        let line = &lines[curr_line];
        let teams = line.split(" - ").collect::<Vec<_>>();
        let away_team_and_result_or_time = teams[1].split("  ").collect::<Vec<_>>();
        let home_team = teams[0].trim().to_owned();
        let away_team = away_team_and_result_or_time[0].trim().to_owned();
        let result_or_time = away_team_and_result_or_time.last().unwrap().trim().to_owned();
        *line_number += 1;

        (home_team, away_team, result_or_time)
    }
}
