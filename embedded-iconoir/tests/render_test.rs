use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::image::ImageDrawable;
use embedded_graphics::mock_display::MockDisplay;
use embedded_graphics::pixelcolor::BinaryColor;

use embedded_iconoir::{icons, Icon};

macro_rules! test_render_icon {
    ( $module_path:path, $cat:ident::$icon:ident ) => {{
        test_render_icon!($module_path::$cat::$icon)
    }};
    ( $path:path ) => {{
        let mut disp = MockDisplay::new();
        let icon: Icon<_, $path> = Icon::new(BinaryColor::On);
        icon.draw(&mut disp)
    }};
}

macro_rules! test_render_icons {
    ( $name:ident, $res:ident::$module:ident, $($cat:ident::$icon:ident),*$(,)? ) => {
        #[test]
        fn $name() -> anyhow::Result<()> {
            $(
                println!("testing icon {}...", stringify!($res::$module::$cat::$icon));
                test_render_icon!($res::$module::$cat::$icon)?;
            )*
            Ok(())
        }
    }
}

#[test]
fn temp() {
    let disp: MockDisplay<BinaryColor> = MockDisplay::new();
    println!("dimensions: {}", disp.size());
}

macro_rules! test_render_icons_premade {
    ($name:ident, $res:ident::$module:ident) => {
        test_render_icons!(
            $name,
            $res::$module,
            activities::Archery,
            emojis::EmojiBlinkRight,
            cloud::CloudDesync,
            three_d_editor::ThreedEllipse,
            actions::Restart,
            animals::Fish,
            animations::Keyframes,
            database::DatabaseSettings,
            design_tools::BorderTr,
            devices::SaveFloppyDisk,
        );
    };
}

test_render_icons_premade!(test_12px, icons::size12px);
test_render_icons_premade!(test_16px, icons::size16px);
test_render_icons_premade!(test_18px, icons::size18px);
test_render_icons_premade!(test_24px, icons::size24px);
test_render_icons_premade!(test_32px, icons::size32px);
test_render_icons_premade!(test_48px, icons::size48px);

// doesn't work as MockDisplay as a hard-coded size of 64px for now
// test_render_icons_premade!(test_96px, icons::size96px);
// test_render_icons_premade!(test_144px, icons::size144px);
