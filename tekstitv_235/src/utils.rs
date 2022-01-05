use colored::Colorize;

pub fn is_empty_or_whitespace(line: &str) -> bool {
    line.is_empty() || line.chars().all(|c| c.is_whitespace())
}

pub fn print_line() {
    println!("{}{}{}", "=================".bright_red(), "=============".bright_white(), "==================================".bright_blue());
}

pub fn print_tonight() {
    println!("{}{}{}", "=================".bright_red(), " NHL TONIGHT ".bright_white(), "==================================".bright_blue());
}
