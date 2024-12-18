pub fn index_to_coord(width: usize, index: usize) -> (u64, u64) {
    let x = (index % width) as u64;
    let y = (index / width) as u64;

    (x, y)
}

pub fn coord_to_index(width: usize, coord: (u64, u64)) -> usize {
    let x = coord.0 as usize;
    let y = coord.1 as usize;

    y * width + x
}

pub fn rotate_dir(dir: (i8, i8)) -> (i8, i8) {
    (-dir.1, dir.0)
}

pub fn position_after_move(
    width: usize,
    height: usize,
    coord: (u64, u64),
    dir: (i8, i8),
) -> Option<(u64, u64)> {
    let x = coord.0 as i64 + dir.0 as i64;
    let y = coord.1 as i64 + dir.1 as i64;

    if x < 0 || x as usize >= width || y < 0 || y as usize >= height {
        return None;
    }

    Some((x as u64, y as u64))
}
