mod structs;
mod systems;

mod plugin;
mod timeline;

pub mod prelude {
    pub use crate::plugin::SplashKitPlugin;
    pub use crate::timeline::Timeline;
}
