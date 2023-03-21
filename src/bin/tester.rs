use embedded_graphics::image::Image;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, StyledDrawable};
use embedded_graphics::text;
use embedded_graphics::text::Text;
use embedded_graphics_core::pixelcolor::{BinaryColor, Rgb888};
use embedded_graphics_core::prelude::*;
use embedded_graphics_simulator::BinaryColorTheme::Default;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use iconoir_rs_embedded::size18px::{actions, activities, buildings, cloud};
use iconoir_rs_embedded::{size18px, size32px, Icon};
use std::ops::Add;

type Color = Rgb888;

fn draw_icon(
    display: &mut impl DrawTarget<Color = Color>,
    icon: &impl ImageDrawable<Color = Color>,
    xpos: u32,
    ypos: u32,
    xincr: u32,
    yincr: u32,
) {
    Image::new(
        icon,
        Point::new((10 + xincr * xpos) as i32, (10 + yincr * ypos) as i32),
    )
    .draw(display)
    .ok();
}

fn main() {
    let mut display = SimulatorDisplay::<Color>::new(Size::new(320, 240));

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    draw_icon(
        &mut display,
        &actions::Cancel::new(Rgb888::CSS_GOLD),
        1,
        1,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &activities::Archery::new(Rgb888::CSS_FLORAL_WHITE),
        2,
        1,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &buildings::Church::new(Rgb888::CSS_LIGHT_SALMON),
        3,
        1,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &buildings::ChurchAlt::new(Rgb888::CSS_SEA_GREEN),
        1,
        2,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &cloud::CloudSync::new(Rgb888::CSS_AQUA),
        2,
        2,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &size18px::development::ElectronicsChip::new(Rgb888::CSS_BEIGE),
        3,
        2,
        20,
        20,
    );

    draw_icon(
        &mut display,
        &size32px::buildings::Church::new(Rgb888::CSS_CORAL),
        3,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &size32px::actions::Cancel::new(Rgb888::CSS_DARK_SLATE_GRAY),
        1,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &size32px::activities::Archery::new(Rgb888::CSS_ALICE_BLUE),
        2,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &size32px::buildings::Church::new(Rgb888::CSS_CORAL),
        3,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &size32px::buildings::ChurchAlt::new(Rgb888::CSS_CHOCOLATE),
        1,
        3,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &size32px::cloud::CloudSync::new(Rgb888::CSS_CYAN),
        2,
        3,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &size32px::development::ElectronicsChip::new(Rgb888::CSS_CRIMSON),
        3,
        3,
        40,
        40,
    );

    let output_settings = OutputSettingsBuilder::new()
        .scale(1)
        // .theme(Theme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);
}
