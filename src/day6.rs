use crate::helpers::coords::{coord_to_index, index_to_coord, position_after_move, rotate_dir};

fn is_obstacle(c: char) -> Option<bool> {
    match c {
        '#' => Some(true),
        '.' => Some(false),
        '^' => Some(false),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum StepResponse {
    NothingSpecial,
    WentOutOfMap,
}

#[derive(Debug)]
struct Map {
    obstacles: Vec<bool>,
    visited: Vec<Vec<(i8, i8)>>,
    width: usize,
    height: usize,
    guard_pos: Option<(u64, u64)>,
    guard_dir: (i8, i8),
}

impl Map {
    fn parse(input: &str) -> Map {
        let obstacles = input
            .chars()
            .filter_map(|c| is_obstacle(c))
            .collect::<Vec<_>>();
        let width = input.find('\n').unwrap();
        let height = obstacles.len() / width;
        let guard_index_offset = input.chars().position(|c| c == '^').unwrap();
        let guard_pos = index_to_coord(width + 1, height, guard_index_offset);
        let guard_index = coord_to_index(width, guard_pos);
        let guard_dir = (0, -1);
        let mut visited = obstacles.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        visited[guard_index].push(guard_dir);
        Map {
            obstacles,
            visited,
            width,
            height,
            guard_pos: Some(guard_pos),
            guard_dir: (0, -1),
        }
    }

    fn step(&mut self) -> StepResponse {
        let guard_pos = self
            .guard_pos
            .expect("step should not be called anymore when guard is out of map already");
        let new_guard_pos_maybe =
            position_after_move(self.width, self.height, guard_pos, self.guard_dir);
        if new_guard_pos_maybe.is_none() {
            // we are leaving the map, just update the position and indicate we are finished
            self.guard_pos = None;
            return StepResponse::WentOutOfMap;
        }
        let new_index = coord_to_index(self.width, new_guard_pos_maybe.unwrap());
        let obstacle_at_new_pos = self.obstacles[new_index];

        if obstacle_at_new_pos {
            // rotate right, don't move in this step
            self.guard_dir = rotate_dir(self.guard_dir);
            let current_index = coord_to_index(self.width, guard_pos);
            self.visited[current_index].push(self.guard_dir)
        } else {
            // tile is empty, move one step forward and mark new field as visited (even if
            // already is)
            self.visited[new_index].push(self.guard_dir);
            self.guard_pos = new_guard_pos_maybe;
        }
        StepResponse::NothingSpecial
    }

    fn print(&self) {
        let player_character = match self.guard_dir {
            (1, 0) => '>',
            (0, 1) => 'v',
            (-1, 0) => '<',
            (0, -1) => '^',
            _ => panic!("invalid dir"),
        };
        for y in 0..self.height as u64 {
            for x in 0..self.width as u64 {
                let coord = (x, y);
                if Some(coord) == self.guard_pos {
                    print!("{}", player_character);
                    continue;
                }
                let index = coord_to_index(self.width, coord);
                let is_obstacle = self.obstacles[index];
                let character = match is_obstacle {
                    true => '#',
                    false => '.',
                };
                print!("{}", character);
            }
            println!("");
        }
        println!("");
    }

    fn count_visited(&self) -> u64 {
        self.visited.iter().filter(|dirs| !dirs.is_empty()).count() as u64
    }
}

pub fn task_one(input: String) -> u64 {
    let mut map = Map::parse(&input);
    while map.step() == StepResponse::NothingSpecial {
        map.print();
    }
    map.count_visited()
}

pub fn task_two(input: String) -> u64 {
    let map = Map::parse(&input);
    0
}
