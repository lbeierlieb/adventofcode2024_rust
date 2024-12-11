use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

fn len_rec(stones: Vec<u64>, n: usize, cache: &[RefCell<HashMap<u64, usize>>]) -> usize {
    if n == 0 {
        return stones.len();
    }

    let mut acc = 0;
    for stone in stones {
        if let Some(cached) = cache[0].borrow().get(&stone) {
            acc += cached;
            continue;
        }
        let step = update_stone(stone);
        let len = len_rec(step, n - 1, &cache[1..]);
        acc += len;

        cache[0].borrow_mut().insert(stone, len);
    }
    acc
}

fn process(input: &str, n: usize) -> u64 {
    let stones = numstring_to_numbers(input);
    let cache = std::iter::repeat_with(|| RefCell::new(HashMap::new()))
        .take(n)
        .collect::<Vec<_>>();
    len_rec(stones, n, &cache[..]) as u64
}

pub fn task_one(input: String) -> u64 {
    process(&input, 25)
}

pub fn task_two(input: String) -> u64 {
    process(&input, 75)
}
