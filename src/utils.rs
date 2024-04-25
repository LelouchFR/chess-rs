
pub type Position = (usize, usize);

pub fn get_valid_position(position: (isize, isize)) -> Option<Position> {
    let (row, col) = position;

    if row >= 0 && row < 8 && col >= 0 && col < 8 {
        println!("{:?}", Some((row as usize, col as usize)));
        Some((row as usize, col as usize))
    } else {
        None
    }
}
