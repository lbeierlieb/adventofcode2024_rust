use crate::helpers::coords::index_to_coord;

#[derive(Debug, PartialEq, Eq)]
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
    guard_pos: (u64, u64),
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
        let guard_pos = index_to_coord(width, height, guard_index);
        Map {
            tiles,
            width,
            height,
            guard_pos,
        }
    }
}

pub fn task_one(input: String) -> u64 {
    let map = Map::parse(&input);
    dbg!(map.guard_pos);
    0
}

pub fn task_two(input: String) -> u64 {
    let map = Map::parse(&input);
    0
}
