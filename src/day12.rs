fn neighbor_steps() -> Vec<(isize, isize)> {
    vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    letter: char,
    visited: bool,
}

impl Tile {
    fn new(c: char) -> Tile {
        Tile {
            letter: c,
            visited: false,
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::new(c)).collect())
            .collect();
        Grid { tiles }
    }

    fn get(&self, x: isize, y: isize) -> Option<&Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        self.tiles
            .get(y as usize)
            .and_then(|line| line.get(x as usize))
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        self.tiles
            .get_mut(y as usize)
            .and_then(|line| line.get_mut(x as usize))
    }

    fn find_first_free_tile(&self) -> Option<(isize, isize)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
            .filter(|(_, _, tile)| tile.visited == false)
            .map(|(x, y, _)| (x as isize, y as isize))
            .next()
    }

    fn visit(&mut self, x: isize, y: isize) {
        self.get_mut(x, y).unwrap().visited = true;
    }

    fn find_cluster(&mut self, x: isize, y: isize) -> (u64, u64) {}

    fn find_cluster_rec(&mut self, x: isize, y: isize, letter: char) -> (u64, u64) {
        let tile = self.get_mut(x, y);
        match tile {
            None => (0, 1),
            Some(tile) if tile.letter != letter => (0, 1),
            Some(tile) => {

                neighbor_steps().iter().
                todo!();
            }
        }
    }

    fn alg(&mut self) -> u64 {
        while let Some((x, y)) = self.find_first_free_tile() {
            let (area, perimeter) = self.find_cluster(x, y);
        }
        0
    }
}

pub fn task_one(input: String) -> u64 {
    let grid = Grid::parse(&input);
    dbg!(&grid);
    dbg!(grid.get(3, -1));
    todo!();
}

pub fn task_two(input: String) -> u64 {
    todo!();
}
