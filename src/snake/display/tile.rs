use crate::{CoordT, GRID_DIMENSIONS};
use ggez::{
    graphics::{drawable_size, Color, DrawMode, Mesh, MeshBuilder},
    Context,
    GameResult,
};

pub trait TileRepr {
    /// Get's the bounds of the tile. This method should be pretty much the same
    /// for most implementations so a default implementation may be added at
    /// some point.
    fn bounds(&self) -> [CoordT; 2];

    /// The color of this tile
    fn color(&self) -> Color;

    fn get_tile_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let (arena_w, arena_h) = drawable_size(ctx);
        let bounds = self.bounds();
        let tile_width = arena_w as f32 / GRID_DIMENSIONS.0 as f32;
        let tile_height = arena_h as f32 / GRID_DIMENSIONS.1 as f32;

        let x = tile_width * bounds[0] as f32;
        let y = tile_height * bounds[1] as f32;

        let rect = [x, y, tile_width, tile_height];
        Mesh::new_rectangle(ctx, DrawMode::fill(), rect.into(), self.color())
    }

    fn add_to_mesh_builder(&self, ctx: &Context, builder: &mut MeshBuilder) {
        let (arena_w, arena_h) = drawable_size(ctx);
        let bounds = self.bounds();
        let tile_width = arena_w as f32 / GRID_DIMENSIONS.0 as f32;
        let tile_height = arena_h as f32 / GRID_DIMENSIONS.1 as f32;

        let x = tile_width * bounds[0] as f32;
        let y = tile_height * bounds[1] as f32;

        let rect = [x, y, tile_width, tile_height].into();
        builder.rectangle(DrawMode::fill(), rect, self.color());
    }
}
