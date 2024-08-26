pub mod assets;
pub mod game;
pub mod resize_scale;

pub mod prelude {
    pub use crate::resources::assets::*;
    pub use crate::resources::game::*;
    pub use crate::resources::resize_scale::*;
}
