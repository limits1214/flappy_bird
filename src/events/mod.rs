pub mod resize;
pub mod game;

pub mod prelude {
    pub use crate::events::resize::*;
    pub use crate::events::game::*;
}