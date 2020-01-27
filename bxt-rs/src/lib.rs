#[macro_use]
extern crate rental;

mod cvar;
pub use cvar::rs_init_cvar;

mod ffi;
mod globals;

mod server;
pub use server::{rs_init_graph, rs_server_clear};

mod tas_editor;
pub use tas_editor::{rs_create_tas_editor, rs_drop_tas_editor};

mod triangle_api;

mod triangle_drawing;
pub use triangle_drawing::rs_tri_draw;

mod utils;