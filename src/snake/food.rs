use super::{display::TileRepr, CoordT, Segment};
use ggez::{graphics::Color, nalgebra::Point2};
use rand::Rng;

pub const FOOD_COLOR: (u8, u8, u8) = (255, 255, 0);

#[derive(Clone, Debug, PartialEq)]
pub struct Food {
    position: Point2<CoordT>,
}

impl Food {
    pub fn new<P: Into<Point2<CoordT>>>(point: P) -> Self {
        Food {
            position: point.into(),
        }
    }

    pub fn random<R: Rng>(rng: &mut R, max_w: CoordT, max_h: CoordT) -> Self {
        Food::new([rng.gen_range(0, max_w), rng.gen_range(0, max_h)])
    }

    pub fn position(&self) -> Point2<CoordT> { self.position }
}

impl PartialEq<Point2<CoordT>> for Food {
    fn eq(&self, other: &Point2<CoordT>) -> bool { self.position.eq(other) }
}

impl PartialEq<Segment> for Food {
    fn eq(&self, other: &Segment) -> bool {
        self.position.eq(&other.position())
    }
}

impl<T: Into<Point2<CoordT>>> From<T> for Food {
    fn from(point: T) -> Food {
        Food {
            position: point.into(),
        }
    }
}

impl TileRepr for Food {
    fn bounds(&self) -> [CoordT; 2] {
        // TODO: There has to be a way to convert a Point2 to a a slice
        [self.position[0], self.position[1]]
    }

    fn color(&self) -> Color { FOOD_COLOR.into() }
}
