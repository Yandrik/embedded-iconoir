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
    ( $name:ident, $res:ident::$module:ident::$icon_set:ident, $($cat:ident::$icon:ident),*$(,)? ) => {
        #[test]
        fn $name() -> anyhow::Result<()> {
            $(
                println!("testing icon {}...", stringify!($res::$module::$icon_set::$cat::$icon));
                test_render_icon!($res::$module::$icon_set::$cat::$icon)?;
            )*
            Ok(())
        }
    }
}

macro_rules! test_render_icons_legacy {
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
    ($name:ident, $res:ident::$module:ident::$icon_set:ident) => {
        test_render_icons!(
            $name,
            $res::$module::$icon_set,
            activities::Archery,
            emojis::EmojiBlinkRight,
            cloud::CloudDesync,
            three_d_editor::EllipseThreed,
            actions::Restart,
            animals::Fish,
            animations::Keyframes,
            database::DatabaseSettings,
            design_tools::BorderTr,
            devices::FloppyDisk,
        );
    };
}

test_render_icons_premade!(test_12px, icons::size12px::regular);
test_render_icons_premade!(test_16px, icons::size16px::regular);
test_render_icons_premade!(test_18px, icons::size18px::regular);
test_render_icons_premade!(test_24px, icons::size24px::regular);
test_render_icons_premade!(test_32px, icons::size32px::regular);
test_render_icons_premade!(test_48px, icons::size48px::regular);

test_render_icons!(
    test_solid_icons,
    icons::size24px::solid,
    actions::PlusCircle,
    actions::XmarkCircle,
    organization::Star,
);

test_render_icons_legacy!(
    test_legacy_category_path,
    icons::size24px,
    actions::Download,
    organization::Star,
);

// doesn't work as MockDisplay as a hard-coded size of 64px for now
// test_render_icons_premade!(test_96px, icons::size96px::regular);
// test_render_icons_premade!(test_144px, icons::size144px::regular);
