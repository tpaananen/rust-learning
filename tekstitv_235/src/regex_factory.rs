use regex::Regex;

pub struct RegexFactory {
    pub regex_on_going_matches_by_time: Regex,
    pub regex_overtime_goal: Regex
}

impl RegexFactory {
    pub fn new() -> Self {
        RegexFactory {
            regex_on_going_matches_by_time: Regex::new(r"^[0-9]{2}.[0-9]{2}").unwrap(),
            regex_overtime_goal: Regex::new(r"([6]{1}[0-5]{1})$").unwrap()
        }
    }
}
