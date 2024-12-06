use crate::helpers::coords::{
    coord_to_index, coord_to_index_checked, index_to_coord, position_after_move, rotate_dir,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum TileState {
    Unvisited,
    Visited,
    Obstacle,
}

impl TileState {
    fn try_from_char(c: char) -> Option<TileState> {
        match c {
            '.' => Some(TileState::Unvisited),
            '#' => Some(TileState::Obstacle),
            '^' => Some(TileState::Visited),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<TileState>,
    width: usize,
    height: usize,
    guard_pos: Option<(u64, u64)>,
    guard_dir: (i8, i8),
}

impl Map {
    fn parse(input: &str) -> Map {
        let tiles = input
            .chars()
            .filter_map(TileState::try_from_char)
            .collect::<Vec<_>>();
        let width = input.find('\n').unwrap();
        let height = tiles.len() / width;
        let guard_index = tiles
            .iter()
            .position(|tile| *tile == TileState::Visited)
            .unwrap();
        let guard_pos = Some(index_to_coord(width, height, guard_index));
        Map {
            tiles,
            width,
            height,
            guard_pos,
            guard_dir: (0, -1),
        }
    }

    fn step(&mut self) -> bool {
        let guard_pos = self
            .guard_pos
            .expect("step should not be called anymore when guard is out of map already");
        let new_guard_pos_maybe =
            position_after_move(self.width, self.height, guard_pos, self.guard_dir);
        if new_guard_pos_maybe.is_none() {
            // we are leaving the map, just update the position and indicate we are finished
            self.guard_pos = None;
            return false;
        }
        let new_index = coord_to_index(self.width, new_guard_pos_maybe.unwrap());
        let tile_at_new_pos = &self.tiles[new_index];

        match *tile_at_new_pos {
            TileState::Obstacle => {
                // tile is blocked, rotate right, don't move in this step
                self.guard_dir = rotate_dir(self.guard_dir);
            }
            _ => {
                // tile is empty, move one step forward and mark new field as visited (even if
                // already is)
                self.tiles[new_index] = TileState::Visited;
                self.guard_pos = new_guard_pos_maybe;
            }
        }
        true
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
                let tile = &self.tiles[index];
                let character = match *tile {
                    TileState::Visited => 'X',
                    TileState::Obstacle => '#',
                    TileState::Unvisited => '.',
                };
                print!("{}", character);
            }
            println!("");
        }
        println!("");
    }

    fn count_visited(&self) -> u64 {
        self.tiles
            .iter()
            .filter(|tile| **tile == TileState::Visited)
            .count() as u64
    }
}

pub fn task_one(input: String) -> u64 {
    let mut map = Map::parse(&input);
    while map.step() {}
    map.count_visited()
}

pub fn task_two(input: String) -> u64 {
    let map = Map::parse(&input);
    0
}
