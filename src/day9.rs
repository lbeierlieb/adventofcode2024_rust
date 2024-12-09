use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Block {
    id: Option<u64>,
}

impl Block {
    fn create_file(id: u64, size: usize) -> Vec<Block> {
        let block = Block { id: Some(id) };
        vec![block; size]
    }

    fn create_empty(size: usize) -> Vec<Block> {
        let block = Block { id: None };
        vec![block; size]
    }

    fn is_empty(&self) -> bool {
        self.id.is_none()
    }
}

#[derive(Debug)]
struct Memory {
    blocks: Vec<Block>,
}

impl Memory {
    fn parse(input: &str) -> Memory {
        let blocks = (0..)
            .zip(input.chars().take_while(|&c| c != '\n'))
            .flat_map(|(i, c)| {
                let size = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    Block::create_file(i / 2, size)
                } else {
                    Block::create_empty(size)
                }
            })
            .collect::<Vec<_>>();
        Memory { blocks }
    }

    fn defragment(&mut self) {
        let mut i = 0;
        while i < self.blocks.len() {
            if !self.blocks[i].is_empty() {
                i += 1;
                continue;
            }

            let last_block = self.trim_end();
            if last_block <= i {
                return;
            }
            self.blocks[i] = self.blocks[last_block];
            self.blocks.remove(last_block);

            i += 1;
        }
    }

    fn trim_end(&mut self) -> usize {
        let mut i = self.blocks.len() - 1;
        while i > 0 {
            if self.blocks[i].is_empty() {
                self.blocks.remove(i);
            } else {
                return i;
            }
            i -= 1;
        }
        panic!();
    }

    fn print(&self) {
        let text = self
            .blocks
            .iter()
            .map(|block| block.id.map(|id| id.to_string()).unwrap_or(".".to_string()))
            .join("");
        println!("{}", text);
    }

    fn checksum(&self) -> u64 {
        (0..)
            .zip(self.blocks.iter())
            .map(|(i, block)| (i * block.id.unwrap()) as u64)
            .sum()
    }
}

pub fn task_one(input: String) -> u64 {
    let mut mem = Memory::parse(&input);
    mem.defragment();
    mem.checksum()
}

pub fn task_two(input: String) -> u64 {
    todo!();
}
