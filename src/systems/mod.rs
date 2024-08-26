pub mod assets;
pub mod bird;
pub mod game;
pub mod ground;
pub mod main_menu;
pub mod mask;
pub mod pipe;
pub mod resize;
pub mod result;
pub mod score;
pub mod touch;
pub mod tween_callback;

#[allow(unused)]
pub mod prelude {
    pub use crate::systems::assets::*;
    pub use crate::systems::bird::*;
    pub use crate::systems::game::*;
    pub use crate::systems::ground::*;
    pub use crate::systems::main_menu::*;
    pub use crate::systems::mask::*;
    pub use crate::systems::pipe::*;
    pub use crate::systems::resize::*;
    pub use crate::systems::result::*;
    pub use crate::systems::score::*;
    pub use crate::systems::touch::*;
    pub use crate::systems::tween_callback::*;
}
