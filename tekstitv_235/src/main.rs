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

    let mut games_on_going: Vec<String> = Vec::new();
    read_and_print_pages(&pages, &mut games_on_going);

    println!();
    println!("{}", "================================================================".bright_blue());
    println!();
    println!("{} {}", "> Lennän seuraavaksi:".bright_blue(), &find_target(&games_on_going));
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

// TODO: refactor into smaller functions
fn read_and_print_pages(pages: &Vec<String>, is_on_going: &mut Vec<String>) {
    let regex_on_going_matches = Regex::new(r"[0-9]+-[0-9]+$").unwrap();
    let regex_on_going_matches_by_time = Regex::new(r"^[0-9]{2}.[0-9]{2}").unwrap();
    let regex_overtime_goal_home = Regex::new(r"^(\s){1}(\S)+(\s)+([6]{1}[0-5]{1})").unwrap();
    let regex_overtime_goal_away = Regex::new(r"(\s){1}(\S)+(\s)+([6]{1}[0-5]{1})").unwrap();

    let selector = Selector::parse(".boxbox > pre").unwrap();
    for page in pages {
        Html::parse_document(&page)
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .for_each(|token| {
                let mut previous = "";
                token
                    .split("\n")
                    .filter(|token| { !is_empty_or_whitespace(token) && !token.contains("NHL-") })
                    .for_each(|token| {
                        let was_previous_by_time = regex_on_going_matches_by_time.is_match(previous);
                        let is_on_going_or_end = regex_on_going_matches.is_match(token);
                        if is_on_going_or_end && !was_previous_by_time {
                            println!();
                        }

                        if is_on_going_or_end && was_previous_by_time {
                            let mut value = String::from(token.trim());
                            value.push_str(" << käynnissä");
                            println!();
                            println!("{}", previous.trim());
                            println!("{}", &value.bright_yellow());
                            is_on_going.push(value);
                        } else if !regex_on_going_matches_by_time.is_match(token) {
                            let line = token.trim();
                            let split = line.split(" ").collect::<Vec<_>>();
                            if is_on_going_or_end {
                                // game result part
                                const OVERTIME: &'static str = "ja";
                                for (index, part) in split.iter().enumerate() {
                                    if is_empty_or_whitespace(&part) {
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
                            } else {
                                // goal scorer and finnish assists part
                                if regex_overtime_goal_home.is_match(&token) {
                                    let end_pos = regex_overtime_goal_home.find(&token).unwrap().end();
                                    print!("{}", &token[..end_pos].bright_magenta());
                                    if token.len() > end_pos + 1 {
                                        println!("{}", &token[end_pos..].bright_cyan());
                                    } else {
                                        println!();
                                    }
                                } else if regex_overtime_goal_away.is_match(&token) {
                                    let start_pos = regex_overtime_goal_away.find(&token).unwrap().start();
                                    print!("{}", &token[..start_pos].bright_cyan());
                                    if token.len() > start_pos + 1 {
                                        println!("{}", &token[start_pos..].bright_magenta());
                                    } else {
                                        println!();
                                    }
                                } else {
                                    println!("{}", &token.bright_cyan());
                                }
                            }
                        }
                        previous = token;
                    });
            });
    }
}

fn is_empty_or_whitespace(token: &str) -> bool {
    token.is_empty() || token.chars().all(|c| c.is_whitespace())
}
