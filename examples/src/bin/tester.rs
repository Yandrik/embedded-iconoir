use embedded_graphics::image::Image;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_iconoir::prelude::IconoirNewIcon;
use embedded_iconoir::size18px::regular::{actions, activities, buildings, cloud};
use embedded_iconoir::{
    size12px::regular as regular12, size18px::regular as regular18, size24px::regular as regular24,
    size32px::regular as regular32,
};

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

    draw_icon(
        &mut display,
        &actions::Xmark::new(Rgb888::CSS_GOLD),
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
        &buildings::ChurchSide::new(Rgb888::CSS_SEA_GREEN),
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
        &regular18::development::ElectronicsChip::new(Rgb888::CSS_BEIGE),
        3,
        2,
        20,
        20,
    );

    draw_icon(
        &mut display,
        &regular32::buildings::Church::new(Rgb888::CSS_CORAL),
        3,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &regular32::actions::Xmark::new(Rgb888::CSS_DARK_SLATE_GRAY),
        1,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &regular32::activities::Archery::new(Rgb888::CSS_ALICE_BLUE),
        2,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &regular32::buildings::Church::new(Rgb888::CSS_CORAL),
        3,
        2,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &regular32::buildings::ChurchSide::new(Rgb888::CSS_CHOCOLATE),
        1,
        3,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &regular32::cloud::CloudSync::new(Rgb888::CSS_CYAN),
        2,
        3,
        40,
        40,
    );
    draw_icon(
        &mut display,
        &regular32::development::ElectronicsChip::new(Rgb888::CSS_CRIMSON),
        3,
        3,
        40,
        40,
    );

    draw_icon(
        &mut display,
        &regular24::activities::Bonfire::new(Rgb888::CSS_FUCHSIA),
        8,
        2,
        30,
        30,
    );

    // 12px

    draw_icon(
        &mut display,
        &regular12::buildings::Church::new(Rgb888::CSS_CORAL),
        7,
        2,
        12,
        12,
    );
    draw_icon(
        &mut display,
        &regular12::actions::Xmark::new(Rgb888::CSS_DARK_SLATE_GRAY),
        8,
        2,
        12,
        12,
    );
    draw_icon(
        &mut display,
        &regular12::activities::Archery::new(Rgb888::CSS_ALICE_BLUE),
        9,
        2,
        12,
        12,
    );
    draw_icon(
        &mut display,
        &regular12::buildings::Church::new(Rgb888::CSS_CORAL),
        10,
        2,
        12,
        12,
    );
    draw_icon(
        &mut display,
        &regular12::buildings::ChurchSide::new(Rgb888::CSS_CHOCOLATE),
        8,
        3,
        12,
        12,
    );
    draw_icon(
        &mut display,
        &regular12::cloud::CloudSync::new(Rgb888::CSS_CYAN),
        9,
        3,
        12,
        12,
    );
    draw_icon(
        &mut display,
        &regular12::development::ElectronicsChip::new(Rgb888::CSS_CRIMSON),
        10,
        3,
        12,
        12,
    );

    let output_settings = OutputSettingsBuilder::new()
        .scale(1)
        // .theme(Theme::OledBlue)
        .build();

    let mut window = Window::new("Hello World", &output_settings);

    'outer: loop {
        window.update(&display);
        for evt in window.events() {
            if let SimulatorEvent::Quit = evt {
                break 'outer;
            }
        }
    }
}
