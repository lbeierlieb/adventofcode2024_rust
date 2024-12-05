use regex::Regex;

use crate::helpers::parsing::numstring_to_numbers;

#[derive(Debug)]
struct Rule {
    smaller: u64,
    bigger: u64,
}

impl Rule {
    fn parse(line: &str) -> Option<Rule> {
        let re_num = Regex::new(r"([0-9]+)\|([0-9]+)").ok()?;
        let cap = re_num.captures(line)?;
        let smaller = cap[1].parse::<u64>().ok()?;
        let bigger = cap[2].parse::<u64>().ok()?;
        Some(Rule { smaller, bigger })
    }

    fn is_broken(&self, fst: u64, snd: u64) -> bool {
        fst == self.bigger && snd == self.smaller
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<u64>,
}

impl Update {
    fn parse(line: &str) -> Update {
        let pages = numstring_to_numbers(line);
        Update { pages }
    }

    fn breaks_rule(&self, rule: &Rule) -> bool {
        self.pages
            .windows(2)
            .any(|win| rule.is_broken(win[0], win[1]))
    }

    fn is_valid(&self, rules: &[Rule]) -> bool {
        !rules.iter().any(|rule| self.breaks_rule(rule))
    }

    fn get_middle_value(&self) -> u64 {
        let middle_index = self.pages.len() / 2;
        self.pages[middle_index]
    }
}

pub fn task_one(input: String) -> u64 {
    let mut split_input = input.split("\n\n");
    let rules = split_input.next().unwrap();
    let updates = split_input.next().unwrap();

    let rules = rules
        .lines()
        .map(|line| Rule::parse(line))
        .collect::<Option<Vec<_>>>()
        .unwrap();
    let updates = updates
        .lines()
        .map(|line| Update::parse(line))
        .collect::<Vec<_>>();

    updates
        .iter()
        .filter(|update| update.is_valid(&rules))
        .map(|update| update.get_middle_value())
        .sum()
}

pub fn task_two(input: String) -> u64 {
    0
}
