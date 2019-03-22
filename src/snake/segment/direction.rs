#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn validate_next_direction(attempt: Direction, current: Direction) -> bool {
    match (attempt, current) {
        (Direction::Left, Direction::Right)
        | (Direction::Right, Direction::Left)
        | (Direction::Up, Direction::Down)
        | (Direction::Down, Direction::Up) => false,
        _ => true,
    }
}
