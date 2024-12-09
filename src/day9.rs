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

    fn checksum(&self) -> u64 {
        (0..)
            .zip(self.blocks.iter())
            .map(|(i, block)| (i * block.id.unwrap()) as u64)
            .sum()
    }
}

#[derive(Debug)]
struct Chunk {
    id: Option<u64>,
    size: usize,
}

impl Chunk {
    fn file(id: u64, size: usize) -> Chunk {
        Chunk { id: Some(id), size }
    }
    fn gap(size: usize) -> Chunk {
        Chunk { id: None, size }
    }

    fn is_empty(&self) -> bool {
        self.id.is_none()
    }
}

#[derive(Debug)]
struct ChunkMemory {
    chunks: Vec<Chunk>,
}

impl ChunkMemory {
    fn parse(input: &str) -> ChunkMemory {
        let chunks = (0..)
            .zip(input.chars().take_while(|&c| c != '\n'))
            .map(|(i, c)| {
                let size = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    Chunk::file(i / 2, size)
                } else {
                    Chunk::gap(size)
                }
            })
            .collect::<Vec<_>>();
        ChunkMemory { chunks }
    }

    fn defragment(&mut self) {
        let highest_id = self.chunks.last().unwrap().id.unwrap();
        for id in (1..=highest_id).rev() {
            let index_file = self
                .chunks
                .iter()
                .position(|chunk| chunk.id == Some(id))
                .unwrap();
            let file_size = self.chunks[index_file].size;
            let maybe_index_gap = self
                .chunks
                .iter()
                .position(|chunk| chunk.is_empty() && chunk.size >= file_size);
            if let Some(index_gap) = maybe_index_gap {
                if index_gap < index_file {
                    self.chunks[index_file].id = None;
                    self.chunks[index_gap].size -= file_size;
                    self.chunks.insert(index_gap, Chunk::file(id, file_size));
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut chunk_start_index = 0;
        let mut acc = 0;
        for chunk in &self.chunks {
            acc += chunk
                .id
                .map(|id| {
                    id * (chunk_start_index..chunk_start_index + chunk.size as u64).sum::<u64>()
                })
                .unwrap_or(0);
            chunk_start_index += chunk.size as u64;
        }
        acc
    }
}

pub fn task_one(input: String) -> u64 {
    let mut mem = Memory::parse(&input);
    mem.defragment();
    mem.checksum()
}

pub fn task_two(input: String) -> u64 {
    let mut mem = ChunkMemory::parse(&input);
    mem.defragment();
    mem.checksum()
}
