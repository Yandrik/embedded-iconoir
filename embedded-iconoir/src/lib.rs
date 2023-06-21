#![no_std]

//! # Icons for Embedded and Resource Constrained Systems
//! The embedded-iconoir crate allows you to use over 1300 icons on all platforms and displays
//! that support [embedded_graphics].
//!
//! ## Usage
//!
//! ```rust
//! # use embedded_graphics::image::Image;
//! # use embedded_graphics::pixelcolor::{BinaryColor};
//! # use embedded_graphics::prelude::*;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut  display = MockDisplay::new();
//! // Import icons and traits
//! use embedded_iconoir::prelude::*;
//!
//! // Create an icon
//! let icon = icons::size24px::actions::Download::new(BinaryColor::On);
//!
//! // Wrap it in an embedded_graphics image
//! let image = Image::new(&icon, Point::zero());
//!
//! // Draw it to a display
//! image.draw(&mut display).unwrap();
//! ```
//!
//! ## Storage Size
//! Using the crate will - without using any icons - not increase your binary size at all.
//! All icons that you use are automatically included in the binary. If you use an icon
//! multiple times, it'll only increase your binary size *once per resolution*.
//!
//! ## Resolutions
//! You can choose which resolutions to enable via features. Available resolutions are:
//!
//! | Resolution |   Module    | Bytes per Icon |
//! |------------|-------------|----------------|
//! | 12px       | [size12px]  | 18             |
//! | 18px       | [size18px]  | 41             |
//! | 24px       | [size24px]  | 72             |
//! | 32px       | [size32px]  | 128            |
//! | 48px       | [size48px]  | 288            |
//! | 96px       | [size96px]  | 1152 (== 1.2kb)|
//! | 144px      | [size144px] | 2592 (== 2.6kb)|
//!
//! ## Preview or find an icon
//! To see a preview of the included icons, please check out the
//! [Iconoir Website](https://iconoir.com). All categories on the website are available
//! as sub-modules of the resolution modules (e.g. the `download` icon in the `Actions` category
//! can be found at the path [`icons::size__px::actions::Download`](icons::size24px::actions::Download).
//!
//! ## "But I want a different resolution!"
//! That is certainly doable. All resolutions are dynamically generated, and the ones that are
//! available are so because they seemed to fit a fairly broad spectrum of use cases.
//! If something is missing in your opinion, please open an issue on [GitHub](https://github.com/Yandrik/embedded-iconoir).
//! Note that bigger resolutions as well as a high amount of different resolutions
//! (e.g `10px`, `11px`, ..., `47px`, `48px`) will at some point conflict with [crates.io](https://crates.io)'s
//! max package size of 10MB, as the icons are pre-baked. So, if this is relevant for your use case,
//! forking the repository and using it as a Git dependency is a better idea.
//!
//! ## Developing locally
//! To develop `embedded-iconoir` locally, clone the repository
//! [from GitHub](https://github.com/Yandrik/embedded-iconoir), and then execute `git submodule init`
//! and `git submodule update` to pull `Iconoir` into the repository. After that, run `cargo xtask generate`
//! to render the icons to `.bits` files, and generate the code.
//!
//!
//! ## Contributing
//!
//! If you found a bug, or think that a feature is missing, please open an issue on [GitHub](https://github.com/yandrik/embedded-iconoir).
//! Of course, Pull Requests are also very much appreciated.
//!
//!
//!

pub mod prelude {
    pub use crate::icon::IconoirNewIcon;
    pub use crate::icons;
}

mod icon;
pub use icon::Icon;

pub mod icons;
pub use icons::*;
