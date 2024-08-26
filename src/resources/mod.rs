pub mod assets;
pub mod resize_scale;
pub mod game;

pub mod prelude {
    pub use crate::resources::assets::*;
    pub use crate::resources::resize_scale::*;
    pub use crate::resources::game::*;
}