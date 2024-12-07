use crate::helpers::parsing::numstring_to_numbers;

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn parse(line: &str) -> Equation {
        let mut nums = numstring_to_numbers(line);
        let result = nums.remove(0);

        Equation {
            result,
            operands: nums,
        }
    }

    fn is_solvable(&self) -> bool {
        let fst = self.operands[0];
        let snd = self.operands[1];
        let mut partial = PartiallySolvedEquation {
            result: self.result,
            accumulator: fst * snd,
            operands_left: &self.operands[2..],
        };
        if partial.is_solvable() {
            return true;
        }
        partial.accumulator = fst + snd;
        partial.is_solvable()
    }
}

#[derive(Debug)]
struct PartiallySolvedEquation<'a> {
    result: u64,
    accumulator: u64,
    operands_left: &'a [u64],
}

impl<'a> PartiallySolvedEquation<'a> {
    fn is_solvable(&self) -> bool {
        if self.result == self.accumulator {
            return true;
        }
        if self.result < self.accumulator {
            return false;
        }
        if self.operands_left.is_empty() {
            return false;
        }

        let mut next_partial = PartiallySolvedEquation {
            result: self.result,
            accumulator: self.accumulator * self.operands_left[0],
            operands_left: &self.operands_left[1..],
        };
        if next_partial.is_solvable() {
            return true;
        }
        next_partial.accumulator = self.accumulator + self.operands_left[0];
        next_partial.is_solvable()
    }
}

pub fn task_one(input: String) -> u64 {
    input
        .lines()
        .map(|line| Equation::parse(line))
        .filter(Equation::is_solvable)
        .map(|equation| equation.result)
        .sum()
}

pub fn task_two(input: String) -> u64 {
    0
}
