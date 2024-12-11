use crate::helpers::parsing::numstring_to_numbers;

fn update_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 1 {
        vec![stone * 2024]
    } else {
        let fst = stone_str
            .chars()
            .take(stone_str.len() / 2)
            .collect::<String>()
            .parse()
            .unwrap();
        let snd = stone_str
            .chars()
            .skip(stone_str.len() / 2)
            .collect::<String>()
            .parse()
            .unwrap();
        vec![fst, snd]
    }
}

fn update_stones(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| update_stone(*stone))
        .collect()
}

fn update_stones_n_times(mut stones: Vec<u64>, n: usize) -> Vec<u64> {
    for i in 0..n {
        stones = update_stones(&stones);
    }
    stones
}

pub fn task_one(input: String) -> u64 {
    let stones = numstring_to_numbers(&input);
    update_stones_n_times(stones, 75).len() as u64
}

pub fn task_two(input: String) -> u64 {
    todo!();
}
