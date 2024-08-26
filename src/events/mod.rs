pub mod game;
pub mod resize;

pub mod prelude {
    pub use crate::events::game::*;
    pub use crate::events::resize::*;
}
