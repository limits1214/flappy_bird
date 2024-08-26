pub mod assets;
pub mod main_menu;
pub mod game;
pub mod result;
pub mod mask;
pub mod resize;
pub mod bird;
pub mod ground;
pub mod touch;
pub mod pipe;
pub mod score;
pub mod tween_callback;

#[allow(unused)]
pub mod prelude {
    pub use crate::systems::assets::*;
    pub use crate::systems::main_menu::*;
    pub use crate::systems::game::*;
    pub use crate::systems::result::*;
    pub use crate::systems::mask::*;
    pub use crate::systems::resize::*;
    pub use crate::systems::bird::*;
    pub use crate::systems::ground::*;
    pub use crate::systems::touch::*;
    pub use crate::systems::pipe::*;
    pub use crate::systems::score::*;
    pub use crate::systems::tween_callback::*;
}