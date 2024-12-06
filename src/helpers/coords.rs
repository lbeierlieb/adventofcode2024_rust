pub fn index_to_coord(width: usize, height: usize, index: usize) -> (u64, u64) {
    let x = (index % width) as u64;
    let y = (index / width) as u64;

    (x, y)
}

pub fn coord_to_index(width: usize, coord: (u64, u64)) -> usize {
    let x = coord.0 as usize;
    let y = coord.1 as usize;

    y * width + x
}

pub fn coord_to_index_checked(width: usize, height: usize, coord: (u64, u64)) -> Option<usize> {
    let x = coord.0 as usize;
    let y = coord.1 as usize;

    if x < 0 || x >= width || y < 0 || y >= height {
        return None;
    }

    Some(coord_to_index(width, coord))
}
