pub mod animation;
pub mod assets;
pub mod bird;
pub mod ffi;
pub mod mask;
pub mod movement;
pub mod on_game;
pub mod on_main_menu;
pub mod on_result;
pub mod picking_callback;
pub mod resize;
pub mod score;
pub mod tween_callback;

pub mod prelude {
    pub use crate::systems::assets::*;
    pub use crate::systems::bird::*;
    pub use crate::systems::mask::*;
    pub use crate::systems::on_game::*;
    pub use crate::systems::on_main_menu::*;
    pub use crate::systems::on_result::*;
    pub use crate::systems::resize::*;
    pub use crate::systems::score::*;
    pub use crate::systems::tween_callback::*;
}
