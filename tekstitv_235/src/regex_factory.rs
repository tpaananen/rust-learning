use regex::Regex;

pub struct RegexFactory {
    pub regex_on_going_matches: Regex,
    pub regex_on_going_matches_by_time: Regex,
    pub regex_not_started_by_time: Regex,
    pub regex_overtime_goal_home: Regex,
    pub regex_overtime_goal_away: Regex,
    pub regex_assistant_home: Regex,
    pub regex_assistant_away: Regex,
}

impl RegexFactory {
    pub fn new() -> Self {
        RegexFactory {
            regex_on_going_matches: Regex::new(r"[0-9]+-[0-9]+$").unwrap(),
            regex_on_going_matches_by_time: Regex::new(r"^[0-9]{2}.[0-9]{2}").unwrap(),
            regex_not_started_by_time: Regex::new(r"[0-9]{2}.[0-9]{2}$").unwrap(),
            regex_overtime_goal_home: Regex::new(r"^(\s){1}(\S)+(\s)+([6]{1}[0-5]{1})").unwrap(),
            regex_overtime_goal_away: Regex::new(r"(\s){1}(\S)+(\s)+([6]{1}[0-5]{1})").unwrap(),
            regex_assistant_home: Regex::new(r"^[(\s)]{1}[(]{1}[(\S)]+[)]{1}").unwrap(),
            regex_assistant_away: Regex::new(r"[(]{1}[(\S)]+[)]{1}[\s]?$").unwrap(),
        }
    }
}
