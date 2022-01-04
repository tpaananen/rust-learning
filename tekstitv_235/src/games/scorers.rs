use crate::regex_factory::RegexFactory;

pub struct Scorers {
    scorers: Vec<String>
}

impl Scorers {
    pub(super) fn from_lines(lines: &Vec<&str>, regex_factory: &RegexFactory, line_number: &mut usize) -> Self {
        let scorers = parse_scores(&lines, line_number);
        Scorers { scorers }
    }

    pub(super) fn print(&self) {
        for scorer in &self.scorers {
            println!(" {}", scorer);
        }
    }
}

fn parse_scores(lines: &Vec<&str>, line_number: &mut usize) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for line in lines.iter().skip(*line_number) {
        let ref_str = *line;
        *line_number += 1;
        vec.push(ref_str.to_owned());
    }
    vec
}
