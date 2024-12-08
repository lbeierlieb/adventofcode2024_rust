use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct TowerMap {
    width: u64,
    height: u64,
    towers: HashMap<char, Vec<Tower>>,
}

impl TowerMap {
    fn parse(input: &str) -> TowerMap {
        let width = input.chars().position(|c| c == '\n').unwrap() as u64;
        let chars = input.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
        let height = chars.len() as u64 / width;
        let towers_ungrouped = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .zip(chars)
            .filter_map(|spot| Tower::parse(spot))
            .collect();
        let towers_grouped = TowerMap::group_towers(towers_ungrouped);
        TowerMap {
            width,
            height,
            towers: towers_grouped,
        }
    }

    fn group_towers(towers: Vec<Tower>) -> HashMap<char, Vec<Tower>> {
        let all_freqs = towers
            .iter()
            .map(|tower| tower.frequency)
            .collect::<Vec<_>>();
        let mut hmap = HashMap::new();
        for freq in all_freqs {
            let all_towers_with_freq = towers
                .iter()
                .cloned()
                .filter(|tower| tower.frequency == freq)
                .collect::<Vec<_>>();
            hmap.insert(freq, all_towers_with_freq);
        }
        hmap
    }
    fn get_all_antinodes<const PROPAGATE: bool>(&self) -> HashSet<(u64, u64)> {
        self.towers
            .values()
            .flat_map(|towers| self.get_all_antinodes_from_towers(towers))
            .collect()
    }

    fn get_all_antinodes_from_towers<const PROPAGATE: bool>(
        &self,
        towers: &[Tower],
    ) -> Vec<(u64, u64)> {
        towers
            .iter()
            .flat_map(|tower0| {
                towers.iter().filter_map(|tower1| {
                    tower0.calculate_antinode_with(tower1, self.width, self.height)
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Tower {
    frequency: char,
    location: (u64, u64),
}

impl Tower {
    fn parse(spot: ((u64, u64), char)) -> Option<Tower> {
        let (location, frequency) = spot;
        if frequency != '.' {
            Some(Tower {
                frequency,
                location,
            })
        } else {
            None
        }
    }

    fn calculate_antinode_with(
        &self,
        other: &Tower,
        width: u64,
        height: u64,
    ) -> Option<(u64, u64)> {
        if self.location == other.location {
            return None;
        }
        let dx = other.location.0 as i64 - self.location.0 as i64;
        let dy = other.location.1 as i64 - self.location.1 as i64;
        let antinode_x = self.location.0 as i64 + 2 * dx;
        let antinode_y = self.location.1 as i64 + 2 * dy;

        if antinode_x < 0 || antinode_y < 0 {
            return None;
        }
        let antinode_x = antinode_x as u64;
        let antinode_y = antinode_y as u64;
        if antinode_x >= width || antinode_y >= height {
            return None;
        }

        Some((antinode_x, antinode_y))
    }
}

pub fn task_one(input: String) -> u64 {
    TowerMap::parse(&input).get_all_antinodes::<false>().len() as u64
}

pub fn task_two(input: String) -> u64 {
    0
}
