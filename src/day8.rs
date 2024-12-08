use std::collections::HashMap;

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

    fn parse_towers_from_map(map: &str) -> Vec<Tower> {
        let width = map.chars().position(|c| c == '\n').unwrap() as u64;
        let chars = map.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
        let height = chars.len() as u64 / width;
        dbg!((width, height));
        (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .zip(chars)
            .filter_map(|spot| Tower::parse(spot))
            .collect()
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
}

#[derive(Debug)]
struct AntiNode {
    frequency: char,
    location: (u64, u64),
}

impl AntiNode {
    fn new(frequency: char, location: (u64, u64)) -> AntiNode {
        AntiNode {
            frequency,
            location,
        }
    }
}

pub fn task_one(input: String) -> u64 {
    let towers = Tower::parse_towers_from_map(&input);
    let groups = Tower::group_towers(towers);
    dbg!(groups);
    0
}

pub fn task_two(input: String) -> u64 {
    0
}
