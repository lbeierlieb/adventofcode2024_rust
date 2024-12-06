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
    LoopDetected,
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
        let guard_pos = index_to_coord(width + 1, guard_index_offset);
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

    fn copy_with_additional_obstacle_at(&self, obstacle_index: usize) -> Map {
        // assumes the map has never been used and just been created with parse()
        let mut obstacles = self.obstacles.clone();
        obstacles[obstacle_index] = true;

        let mut visited = obstacles.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        let guard_index = coord_to_index(self.width, self.guard_pos.unwrap());
        visited[guard_index].push(self.guard_dir);
        Map {
            obstacles,
            visited,
            width: self.width,
            height: self.height,
            guard_pos: self.guard_pos,
            guard_dir: self.guard_dir,
        }
    }

    fn all_maps_with_additional_obstacle<'a>(&'a self) -> Box<dyn Iterator<Item = Map> + 'a> {
        let guard_index = coord_to_index(self.width, self.guard_pos.unwrap());
        Box::new(
            (0..self.obstacles.len())
                .filter(move |&index| self.obstacles[index] == false && index != guard_index)
                .map(|index| self.copy_with_additional_obstacle_at(index)),
        )
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
            let already_visited = self.mark_visited(current_index, self.guard_dir);
            if already_visited {
                return StepResponse::LoopDetected;
            }
        } else {
            // move one step forward and mark new field as visited
            self.guard_pos = new_guard_pos_maybe;
            let already_visited = self.mark_visited(new_index, self.guard_dir);
            if already_visited {
                return StepResponse::LoopDetected;
            }
        }
        StepResponse::NothingSpecial
    }

    fn mark_visited(&mut self, index: usize, dir: (i8, i8)) -> bool {
        if self.visited[index].contains(&dir) {
            return true;
        }

        self.visited[index].push(dir);
        false
    }

    fn step_loop(&mut self) -> StepResponse {
        loop {
            match self.step() {
                StepResponse::NothingSpecial => {}
                response => return response,
            }
        }
    }

    fn count_visited(&self) -> u64 {
        self.visited.iter().filter(|dirs| !dirs.is_empty()).count() as u64
    }
}

pub fn task_one(input: String) -> u64 {
    let mut map = Map::parse(&input);
    map.step_loop();
    map.count_visited()
}

pub fn task_two(input: String) -> u64 {
    let map = Map::parse(&input);
    let all_maps = map.all_maps_with_additional_obstacle();
    all_maps
        .map(|mut map| map.step_loop())
        .filter(|exit_reason| *exit_reason == StepResponse::LoopDetected)
        .count() as u64
}
