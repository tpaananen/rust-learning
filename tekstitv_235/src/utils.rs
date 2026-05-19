use std::io::{stdout, Write};

use colored::Colorize;

use crate::localization::Locale;

pub fn is_empty_or_whitespace(line: &str) -> bool {
    line.is_empty() || line.chars().all(|c| c.is_whitespace())
}

pub fn print_line() {
    println!(
        "{}{}{}",
        "================= ".bright_red(),
        "=============".bright_white(),
        " ==================================".bright_blue()
    );
    stdout().flush().expect("flushing failed");
}

pub fn print_tonight(locale: Locale) {
    println!(
        "{}{}{}",
        "================= ".bright_red(),
        locale.tonight_heading().bright_white(),
        "  ==================================".bright_blue()
    );
    stdout().flush().expect("flushing failed");
}

pub fn print_tomorrow(locale: Locale) {
    println!(
        "{}{}{}",
        "================= ".bright_red(),
        locale.tomorrow_heading().bright_white(),
        " ==================================".bright_blue()
    );
    stdout().flush().expect("flushing failed");
}

pub fn print_loading(locale: Locale) {
    println!(
        "{}{}{}",
        "================= ".bright_red(),
        locale.loading_heading().bright_white(),
        "  ==================================".bright_blue()
    );
    stdout().flush().expect("flushing failed");
}

pub fn print_selection(locale: Locale) {
    println!(
        "{}{}{}",
        "================= ".bright_red(),
        locale.selection_heading().bright_white(),
        " ==================================".bright_blue()
    );
    stdout().flush().expect("flushing failed");
}
