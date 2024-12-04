#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Grid { data }
    }

    fn get_char_at(&self, x: usize, y: usize) -> Option<char> {
        let line = self.data.get(y)?;
        line.get(x).copied()
    }

    fn get_char_at_signed(&self, x: isize, y: isize) -> Option<char> {
        if x >= 0 || y >= 0 {
            self.get_char_at(x as usize, y as usize)
        } else {
            None
        }
    }

    fn get_word_at(&self, x: usize, y: usize, len: usize, dir: (isize, isize)) -> Option<String> {
        (0..len as isize)
            .map(|i| (x as isize + i * dir.0, y as isize + i * dir.1))
            .map(|(x, y)| self.get_char_at_signed(x, y))
            .collect()
    }

    fn get_all_words_at(&self, x: usize, y: usize, len: usize) -> Vec<String> {
        directions()
            .into_iter()
            .filter_map(|dir| self.get_word_at(x, y, len, dir))
            .collect()
    }

    fn get_width(&self) -> usize {
        self.data
            .get(0)
            .map(|line| line.len())
            .expect("cannot determine width of grid with height=0")
    }

    fn get_height(&self) -> usize {
        self.data.len()
    }

    fn get_all_coords(&self) -> Vec<(usize, usize)> {
        (0..self.get_width())
            .flat_map(|x| (0..self.get_height()).map(move |y| (x, y)))
            .collect()
    }
}

fn directions() -> Vec<(isize, isize)> {
    vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
}

pub fn task_one(input: String) -> u64 {
    let grid = Grid::parse(&input);
    grid.get_all_coords()
        .into_iter()
        .flat_map(|(x, y)| grid.get_all_words_at(x, y, 4))
        .filter(|word| word == "XMAS")
        .count() as u64
}

pub fn task_two(input: String) -> u64 {
    0
}
