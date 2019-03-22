use super::TileRepr;
use ggez::{graphics::Mesh, Context, GameResult};

pub trait MeshRepr {
    fn get_mesh(&self, ctx: &mut Context) -> GameResult<Mesh>;
}

impl<T: TileRepr> MeshRepr for T {
    fn get_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        self.get_tile_mesh(ctx)
    }
}
