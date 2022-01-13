use colored::Colorize;
use crate::{games::game::GameStatus, constants::{COL_WIDTH_HOME, COL_WIDTH_AWAY}};

const RESULT_WIDTH: usize = 8;

struct GameResult {
    result: String,
    is_overtime: bool
}

impl GameResult {
    fn new(result_str: &str) -> Self {
        let is_overtime = result_str.starts_with("ja ") || result_str.starts_with("vl ");
        GameResult { result: result_str.to_owned(), is_overtime }
    }

    fn to_string(&self, color: &str) -> String {
        if self.is_overtime {
            format!("{result:>RESULT_WIDTH$}", result = self.result.bright_magenta())
        } else {
            format!("{result:>RESULT_WIDTH$}", result = self.result.color(color))
        }
    }
}

pub struct Teams {
    home_team: String,
    away_team: String,
    result: GameResult
}

impl Teams {
    pub(super) fn from_lines(lines: &Vec<&str>, line_number: &mut usize) -> Option<Self> {
            let curr_line = *line_number;
        if lines.len() <= curr_line {
            return None;
        }

        let line = &lines[curr_line];
        let teams = line.split(" - ").collect::<Vec<_>>();
        if teams.len() < 2 {
            return None;
        }

        let away_team_and_result_or_time = teams[1].split("  ").collect::<Vec<_>>();
        let home_team = teams[0].trim().to_owned();
        let away_team = away_team_and_result_or_time[0].trim().to_owned();
        let result = GameResult::new(away_team_and_result_or_time.last().unwrap().trim());
        *line_number += 1;

        Some(Teams { home_team, away_team, result })
    }

    pub(super) fn get_home_team_name(&self) -> &str {
        &self.home_team
    }

    pub(super) fn get_result(&self) -> &str {
        &self.result.result
    }

    pub(super) fn get_is_overtime(&self) -> bool {
        self.result.is_overtime
    }

    pub(super) fn print(&self, status: &GameStatus) {
        let color = status.to_color();
        println!(
            "{home_team:<COL_WIDTH_HOME$} - {away_team:<COL_WIDTH_AWAY$}{result}",
            home_team = self.home_team.color(color),
            away_team = self.away_team.color(color),
            result  = self.result.to_string(color)
        );
    }
}
