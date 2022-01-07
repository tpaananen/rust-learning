use colored::Colorize;
use crate::{games::game::GameStatus, constants::{COL_WIDTH_HOME, COL_WIDTH_AWAY}};

pub struct Teams {
    home_team: String,
    away_team: String,
    result: String,
    is_overtime: bool
}

impl Teams {
    pub(super) fn from_lines(lines: &Vec<&str>, line_number: &mut usize) -> Option<Self> {
            let curr_line = *line_number;
        if lines.len() <= curr_line {
            return None;
        }

        let line = &lines[curr_line];
        let teams = line.split(" - ").collect::<Vec<_>>();
        let away_team_and_result_or_time = teams[1].split("  ").collect::<Vec<_>>();
        let home_team = teams[0].trim().to_owned();
        let away_team = away_team_and_result_or_time[0].trim().to_owned();
        let result = away_team_and_result_or_time.last().unwrap().trim().to_owned();
        let is_overtime = result.starts_with("ja ") || result.starts_with("vl ");
        *line_number += 1;

        Some(Teams { home_team, away_team, result, is_overtime })
    }

    pub(super) fn get_home_team_name(&self) -> &str {
        &self.home_team
    }

    pub(super) fn get_result(&self) -> &str {
        &self.result
    }

    pub(super) fn get_is_overtime(&self) -> bool {
        self.is_overtime
    }

    pub(super) fn print(&self, status: &GameStatus) {
        let color = status.to_color();
        print!("{:<home_width$}", self.home_team.color(color), home_width = COL_WIDTH_HOME);
        print!(" - ");
        print!("{:<away_width$}", self.away_team.color(color), away_width = COL_WIDTH_AWAY);

        if self.is_overtime {
            println!("{:>result_width$}", self.result.bright_magenta(), result_width = 8);
        } else {
            println!("{:>result_width$}", self.result.color(color), result_width = 8);
        }
    }
}
