// imports for include! file
#[allow(unused)]
use crate::icon::*;
#[allow(unused)]
use crate::{make_icon, make_icon_category};
#[allow(unused)]
use bit_field::*;
#[allow(unused)]
use embedded_graphics::prelude::*;
#[allow(unused)]
use static_assertions::const_assert;

/*
approximate structure per category & resolution:
pub mod size24px {
use super::*;
make_icon_category!(actions, 24, "Actions", [
    (AddCircle, "add-circle"),
    (Cancel, "cancel"),
    (Check, "check"),
    (DeleteCircle, "delete-circle"),
    ...
]);
...
}
...
 */

include!("./icons.gen.rs");
