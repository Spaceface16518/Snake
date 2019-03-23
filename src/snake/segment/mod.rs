use super::{display::TileRepr, Food};
use crate::CoordT;
pub use direction::{validate_next_direction, Direction};
use ggez::{graphics::Color, nalgebra::Point2};

mod direction;

pub const SEGMENT_COLOR: (u8, u8, u8) = (255, 0, 0);

#[derive(Clone, Debug)]
pub struct Segment {
    position: Point2<CoordT>,
    direction: Direction,
}

impl Segment {
    pub fn new<P: Into<Point2<CoordT>>>(
        position: P,
        direction: Direction,
    ) -> Self {
        Segment {
            position: position.into(),
            direction,
        }
    }

    pub const fn position(&self) -> Point2<CoordT> { self.position }

    pub const fn direction(&self) -> Direction { self.direction }
}

impl PartialEq<Segment> for Segment {
    fn eq(&self, rhs: &Segment) -> bool { self.position().eq(&rhs.position()) }
}

impl PartialEq<Point2<CoordT>> for Segment {
    fn eq(&self, rhs: &Point2<CoordT>) -> bool { self.position().eq(rhs) }
}

impl PartialEq<Food> for Segment {
    fn eq(&self, other: &Food) -> bool { self.position().eq(&other.position()) }
}

impl TileRepr for Segment {
    fn color(&self) -> Color { SEGMENT_COLOR.into() }

    fn bounds(&self) -> [CoordT; 2] {
        // TODO: There has to be a way to convert a Point2 to a a slice
        [self.position[0], self.position[1]]
    }
}
