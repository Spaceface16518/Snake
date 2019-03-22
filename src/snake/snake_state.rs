use super::{Food, Snake};

pub enum SnakeState {
    Segment,
    Food,
    Nothing,
}

pub fn state(snake: &Snake, food: &Food) -> SnakeState {
    if snake.overlaps_head(food) {
        SnakeState::Food
    } else if snake.overlaps_body(
        snake.head().expect("Snake was empty; could not compute state"),
    ) {
        SnakeState::Segment
    } else {
        SnakeState::Nothing
    }
}
