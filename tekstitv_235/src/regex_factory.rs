use regex::Regex;

pub struct RegexFactory {
    pub regex_on_going_matches_by_time: Regex,
    pub regex_overtime_goal: Regex,
}

impl Default for RegexFactory {
    fn default() -> Self {
        Self {
            regex_on_going_matches_by_time: Regex::new(r"^\d{2}\.\d{2}")
                .expect("time regex should compile"),
            regex_overtime_goal: Regex::new(r"6[0-5]$").expect("overtime regex should compile"),
        }
    }
}

impl RegexFactory {
    pub fn new() -> Self {
        Self::default()
    }
}
