#![no_std]

pub mod prelude {
    pub use crate::icon::IconoirNewIcon;
}

mod icon;
pub use icon::Icon;

pub mod icons;
pub use icons::*;
