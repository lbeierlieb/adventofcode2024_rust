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

    fn is_solvable<const USE_CONCAT: bool>(&self) -> bool {
        let fst = self.operands[0];
        let snd = self.operands[1];
        let mut partial = PartiallySolvedEquation {
            result: self.result,
            accumulator: fst * snd,
            operands_left: &self.operands[2..],
        };
        if partial.is_solvable::<USE_CONCAT>() {
            return true;
        }
        if USE_CONCAT {
            partial.accumulator = format!("{}{}", fst, snd).parse().unwrap();
            if partial.is_solvable::<USE_CONCAT>() {
                return true;
            }
        }
        partial.accumulator = fst + snd;
        partial.is_solvable::<USE_CONCAT>()
    }
}

#[derive(Debug)]
struct PartiallySolvedEquation<'a> {
    result: u64,
    accumulator: u64,
    operands_left: &'a [u64],
}

impl<'a> PartiallySolvedEquation<'a> {
    fn is_solvable<const USE_CONCAT: bool>(&self) -> bool {
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
        if next_partial.is_solvable::<USE_CONCAT>() {
            return true;
        }
        if USE_CONCAT {
            next_partial.accumulator = format!("{}{}", self.accumulator, self.operands_left[0])
                .parse()
                .unwrap();
            if next_partial.is_solvable::<USE_CONCAT>() {
                return true;
            }
        }
        next_partial.accumulator = self.accumulator + self.operands_left[0];
        next_partial.is_solvable::<USE_CONCAT>()
    }
}

pub fn process<const USE_CONCAT: bool>(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Equation::parse(line))
        .filter(Equation::is_solvable::<USE_CONCAT>)
        .map(|equation| equation.result)
        .sum()
}

pub fn task_one(input: String) -> u64 {
    process::<false>(&input)
}

pub fn task_two(input: String) -> u64 {
    process::<true>(&input)
}
