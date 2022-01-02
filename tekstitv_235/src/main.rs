use rand::Rng;
use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};
use colored::*;

const MESSAGE: &'static str = "Jäämiehet varmaan hommissa...";

#[tokio::main]
async fn main() {
    let pages = fetch_pages().await;
    if pages.len() == 0 {
        println!("{}", MESSAGE);
        return;
    }

    let games_on_going = read_and_print_pages(&pages);
    print_selected_target(&find_target(&games_on_going));
}

fn print_selected_target(target: &str) {
    println!();
    println!("{}", "================================================================".bright_blue());
    println!();
    println!("{} {}", "> Lennän seuraavaksi:".bright_blue(), target);
    println!();
    println!("{}", "================================================================".bright_blue());
    println!();
}

async fn fetch_pages() -> Vec<String> {
    let mut index = 1;
    let mut pages: Vec<String> = Vec::new();

    // for some testing
    // let contents = std::fs::read_to_string("./assets/sivu0001.htm")
    //     .expect("Something went wrong reading the file");
    // pages.push(contents);

    loop {
        let url_str = format!("https://yle.fi/tekstitv/txt/235_{:0>4}.htm", index);
        let url = Url::parse(&url_str).unwrap();
        let res = reqwest::get(url)
            .await
            .unwrap();

        if !res.status().is_success() {
            break;
        }

        let html = res.text()
            .await
            .unwrap_or("".to_string());

        if html.len() == 0 {
            break;
        }
        pages.push(html);
        index += 1;
    }
    pages
}

fn find_target(games: &Vec<String>) -> String {
    if games.len() == 0 {
        return MESSAGE.to_string();
    }

    let mut rng = rand::thread_rng();
    let rand_value: f64 = rng.gen();
    let game = &games[(rand_value * 1000.0) as usize % games.len()];
    let teams = game.split(" - ").map(|a| a.to_owned()).collect::<Vec<String>>();
    teams.first().unwrap_or(&MESSAGE.to_string()).trim().to_string()
}

fn read_and_print_pages(pages: &Vec<String>) -> Vec<String> {
    let regex_on_going_matches = Regex::new(r"[0-9]+-[0-9]+$").unwrap();
    let regex_on_going_matches_by_time = Regex::new(r"^[0-9]{2}.[0-9]{2}").unwrap();
    let regex_overtime_goal_home = Regex::new(r"^(\s){1}(\S)+(\s)+([6]{1}[0-5]{1})").unwrap();
    let regex_overtime_goal_away = Regex::new(r"(\s){1}(\S)+(\s)+([6]{1}[0-5]{1})").unwrap();
    let selector = Selector::parse(".boxbox > pre").unwrap();

    let mut previous = "".to_owned();
    let mut games_on_going: Vec<String> = Vec::new();

    for page in pages {
        Html::parse_document(&page)
            .select(&selector)
            .flat_map(|element| { element.text().flat_map(|text| { text.lines() })})
            .filter(|line| { !is_empty_or_whitespace(line) && !line.contains("NHL-") })
            .for_each(|line| {
                let was_previous_by_time = regex_on_going_matches_by_time.is_match(&previous);
                let is_on_going_or_end = regex_on_going_matches.is_match(line);
                if is_on_going_or_end && !was_previous_by_time {
                    println!();
                }

                if is_on_going_or_end && was_previous_by_time {
                    let current = String::from(line.trim());
                    print_on_going_result_row(&previous, &current);
                    games_on_going.push(current);
                } else if !regex_on_going_matches_by_time.is_match(line) {
                    if is_on_going_or_end {
                        process_game_result_row_when_end(line.trim());
                    } else {
                        process_goal_scorer_row(&regex_overtime_goal_home, &regex_overtime_goal_away, &line);
                    }
                }
                previous = line.to_owned();
            });
    }
    games_on_going
}

fn print_on_going_result_row(previous: &str, current: &str) {
    println!();
    println!("{}", previous.trim());
    print!("{}", current.bright_yellow());
    println!("{}", " <<< käynnissä".bright_yellow());
}

fn process_game_result_row_when_end(line: &str) {
    const OVERTIME: &'static str = "ja";
    // can't use split_whitespace since it takes whites out of the result
    let split = line.split(" ").collect::<Vec<_>>();
    for (index, part) in split.iter().enumerate() {
        if is_empty_or_whitespace(part) {
            print!(" ");
        } else if *part == OVERTIME {
            print!(" {}", part.bright_green());
        } else if index < split.len() - 1 {
            if index == 0 {
                print!("{}", part);
            } else {
                print!(" {}", part);
            }
        } else {
            println!(" {}", part.bright_green());
        }
    }
}

fn process_goal_scorer_row(regex_overtime_goal_home: &Regex, regex_overtime_goal_away: &Regex, line: &str) {
    if regex_overtime_goal_home.is_match(line) {
        let end_pos = regex_overtime_goal_home.find(line).unwrap().end();
        print!("{}", line[..end_pos].bright_magenta());
        if line.len() > end_pos + 1 {
            println!("{}", line[end_pos..].bright_cyan());
        } else {
            println!();
        }
    } else if regex_overtime_goal_away.is_match(line) {
        let start_pos = regex_overtime_goal_away.find(line).unwrap().start();
        print!("{}", line[..start_pos].bright_cyan());
        if line.len() > start_pos + 1 {
            println!("{}", line[start_pos..].bright_magenta());
        } else {
            println!();
        }
    } else {
        println!("{}", line.bright_cyan());
    }
}

fn is_empty_or_whitespace(line: &str) -> bool {
    line.is_empty() || line.chars().all(|c| c.is_whitespace())
}
