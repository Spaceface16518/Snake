use crate::{CoordT, GRID_DIMENSIONS, TILE_SIZE};
use ggez::{
    graphics::{drawable_size, Color, DrawMode, Mesh, MeshBuilder},
    Context,
    GameResult,
};
use  ggez::graphics::Rect;
use crate::WINDOW_DIMENSIONS;

pub trait TileRepr {
    /// Get's the bounds of the tile. This method should be pretty much the same
    /// for most implementations so a default implementation may be added at
    /// some point.
    fn bounds(&self) -> [CoordT; 2];

    /// The color of this tile
    fn color(&self) -> Color;

    fn get_rect(&self) -> Rect {
        let bounds = self.bounds();

        let x = TILE_SIZE * bounds[0] as f32;
        let y = TILE_SIZE * bounds[1] as f32;

        Rect {
            x,
            y,
            w: TILE_SIZE,
            h: TILE_SIZE
        }
    }

    fn get_tile_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        Mesh::new_rectangle(ctx, DrawMode::fill(), self.get_rect(), self.color())
    }

    fn add_to_mesh_builder(&self, ctx: &Context, builder: &mut MeshBuilder) {
        builder.rectangle(DrawMode::fill(), self.get_rect(), self.color());
    }
}
