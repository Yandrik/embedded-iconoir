
// imports for include! file
#[allow(unused)]
use crate::{make_icon, make_icon_category};
#[allow(unused)]
use crate::icon::*;
#[allow(unused)]
use bit_field::*;
#[allow(unused)]
use embedded_graphics::prelude::*;

/*
make_icon_category!(actions, 24, "Actions", [
    (AddCircle, "add-circle"),
    (Cancel, "cancel"),
    (Check, "check"),
    (DeleteCircle, "delete-circle"),
]);
 */

include!("./icons.gen.rs");
