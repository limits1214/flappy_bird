pub mod game;
pub mod main_menu;
pub mod mask;
pub mod resize;
pub mod states;
pub mod timer;

pub mod prelude {
    pub use crate::components::game::*;
    pub use crate::components::main_menu::*;
    pub use crate::components::mask::*;
    pub use crate::components::resize::*;
    pub use crate::components::states::*;
    pub use crate::components::timer::*;
}
