use super::GRID_DIMENSIONS;
use arith::ModuloSigned;
pub use display::{clear_screen, MeshRepr, TileRepr};
pub use food::Food;
use ggez::{
    graphics::{Mesh, MeshBuilder},
    nalgebra::{Point2, Vector2},
    Context,
    GameResult,
};
pub use segment::{validate_next_direction, Direction, Segment};
use std::collections::VecDeque;

pub type CoordT = isize;
pub const DEFAULT_DIRECTION: Direction = Direction::Right;
pub const DEFAULT_SNAKE_COORD: [CoordT; 2] =
    [GRID_DIMENSIONS.0 / 2, GRID_DIMENSIONS.1 / 2];

mod arith;
mod display;
mod food;
mod segment;
pub mod snake_state;

#[derive(Debug, PartialEq)]
pub struct Snake {
    segements: VecDeque<Segment>,
}

impl Snake {
    pub fn new<P: Into<Point2<CoordT>>>(head_coord: P) -> Self {
        Snake {
            segements: {
                let mut tmp = VecDeque::new();
                tmp.push_front(Segment::new(
                    head_coord.into(),
                    Direction::Right,
                ));
                tmp
            },
        }
    }

    /// Internally computes the next frame of the snake. Returns `None` if
    /// popping from the snake returns `None`
    pub fn compute_next_frame(&mut self, new_head: Segment) -> Option<()> {
        self.segements.pop_back()?;
        self.segements.push_front(new_head);
        Some(())
    }

    pub fn compute_next_frame_with_new(&mut self, new_head: Segment) {
        self.segements.push_front(new_head);
    }

    /// Get a reference to the head of the snake. Should only return `None` if
    /// the snake is empty.
    pub fn head(&self) -> Option<&Segment> { (self.segements.front()) }

    /// Computes the next virtual head of the snake. Returns an error if the
    /// head is out of bounds.

    /// TODO: return more comprehensive error
    ///
    /// BUG: using the next head to calculate overlaps could be the source of
    /// bugs
    pub fn compute_next_head(
        &self,
        new_direction: Direction,
    ) -> Option<Segment> {
        if let Some(head) = self.head() {
            let new_point = match new_direction {
                Direction::Up => head.position() - Vector2::new(0, 1),
                Direction::Down => head.position() + Vector2::new(0, 1),
                Direction::Left => head.position() - Vector2::new(1, 0),
                Direction::Right => head.position() + Vector2::new(1, 0),
            };
            Some(Segment::new(
                {
                    let x = new_point[0].modulo(GRID_DIMENSIONS.0);
                    let y = new_point[1].modulo(GRID_DIMENSIONS.1);
                    Point2::new(x, y)
                },
                new_direction,
            ))
        } else {
            None
        }
    }

    ///  Checks if an arbitrary segment overlaps with any of the snake's
    /// segments.
    ///
    /// WARNING: This checks the entire snake! To check only the body segments,
    /// use the `overlaps_body` method
    #[allow(unused)]
    pub fn overlaps<S>(&self, other: &S) -> bool
    where
        Segment: PartialEq<S>,
    {
        for segment in self.segements.iter() {
            if segment == other {
                return true
            }
        }
        false
    }

    /// Checks if an arbitrary segment overlaps with any of snake's body
    /// segments.
    ///
    /// WARNING: This checks only the snake's body! It is the exact same as
    /// `overlaps` but the internal iterator skips one segment.
    pub fn overlaps_body<S>(&self, other: &S) -> bool
    where
        Segment: PartialEq<S>,
    {
        for segment in self.segements.iter().skip(1) {
            if segment == other {
                return true
            }
        }
        false
    }

    /// Checks if an arbitrary segment overlaps with the snake's head. This will
    /// return `false` if the snake's `head` method returns a `None` value; that
    /// is, if the snake is empty, this method will return `false`.
    pub fn overlaps_head<S>(&self, other: &S) -> bool
    where
        Segment: PartialEq<S>,
    {
        if let Some(head) = self.head() {
            head == other
        } else {
            false
        }
    }

    pub fn head_direction(&self) -> Option<Direction> {
        Some(self.head()?.direction())
    }
}

impl MeshRepr for Snake {
    fn get_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let mut builder = MeshBuilder::new();

        for segment in self.segements.iter() {
            segment.add_to_mesh_builder(&mut builder);
        }
        builder.build(ctx)
    }
}
