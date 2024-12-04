#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Grid { data }
    }
}

pub fn task_one(input: String) -> u64 {
    let grid = Grid::parse(&input);
    dbg!(grid);
    0
}

pub fn task_two(input: String) -> u64 {
    0
}
