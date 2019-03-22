use ggez::{graphics::clear, Context};
pub use mesh::MeshRepr;
pub use tile::TileRepr;

mod mesh;
mod tile;

pub const BACKGROUND_COLOR: (u8, u8, u8) = (0, 128, 0);

pub fn clear_screen(ctx: &mut Context) { clear(ctx, BACKGROUND_COLOR.into()) }
