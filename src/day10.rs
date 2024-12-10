use std::{collections::HashSet, iter};

#[derive(Debug)]
struct TopoMap {
    heights: Vec<Vec<u8>>,
}

impl TopoMap {
    fn parse(input: &str) -> TopoMap {
        let heights = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        TopoMap { heights }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }
        self.heights.get(y as usize)?.get(x as usize).copied()
    }

    fn num_trails(&self) -> u64 {
        let map_height = self.heights.len();
        let map_width = self.heights[0].len();

        (0..map_height)
            .flat_map(|y| {
                (0..map_width).map(move |x| self.trail_ends_reachable_from(x, y).len() as u64)
            })
            .sum()
    }

    fn trail_ends_reachable_from(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        self.num_trails_at_rec(x as isize, y as isize, 0)
    }

    fn num_trails_at_rec(&self, x: isize, y: isize, height: u8) -> HashSet<(usize, usize)> {
        let cur_height = match self.get(x, y) {
            None => return HashSet::new(), // x, y out of bounds
            Some(height) => height,
        };
        if height != cur_height {
            return HashSet::new();
        }
        if height == 9 {
            return iter::once((x as usize, y as usize)).collect();
        }
        vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .flat_map(|(dx, dy)| self.num_trails_at_rec(x + dx, y + dy, height + 1))
            .collect()
    }
}

pub fn task_one(input: String) -> u64 {
    let map = TopoMap::parse(&input);
    map.num_trails()
}

pub fn task_two(input: String) -> u64 {
    todo!();
}
