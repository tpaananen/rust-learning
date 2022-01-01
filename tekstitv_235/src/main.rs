use rand::Rng;
use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};

const MESSAGE: &'static str = "Jäämiehet varmaan hommissa...";

#[tokio::main]
async fn main() {
    let pages = fetch_pages().await;
    if pages.len() == 0 {
        println!("{}", MESSAGE);
        return;
    }

    let mut games_on_going: Vec<String> = Vec::new();
    read_pages(&pages, &mut games_on_going);
    println!("Lennän seuraavaksi: {}", &find_target(&games_on_going));
}

async fn fetch_pages() -> Vec<String> {
    let mut index = 1;
    let mut pages: Vec<String> = Vec::new();
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

fn read_pages(pages: &Vec<String>, is_on_going: &mut Vec<String>) {
    let regex_on_going_matches = Regex::new(r"[0-9]+-[0-9]+$").unwrap();
    let regex_on_going_matches_by_time = Regex::new(r"^[0-9]{2}.[0-9]{2}").unwrap();
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
                    .filter(|token| { token.trim().len() > 0 && (regex_on_going_matches_by_time.is_match(token) || token.contains(" - ")) })
                    .for_each(|token| {
                        let was_previous_by_time = regex_on_going_matches_by_time.is_match(previous);
                        if regex_on_going_matches.is_match(token) && was_previous_by_time {
                            let mut value = String::from(token);
                            value.push_str(" << käynnissä");
                            if was_previous_by_time {
                                println!("{}", &previous);
                            }
                            println!("{}", &value);
                            is_on_going.push(value);
                        } else if !regex_on_going_matches_by_time.is_match(token) {
                            println!("{}", &token);
                        }
                        previous = token;
                    });
            });
    }
}