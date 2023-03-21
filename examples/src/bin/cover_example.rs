use std::thread;
use std::time::Duration;
use colorous::Color;
use embedded_graphics::image::Image;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window};
use embedded_iconoir::{Icon, icons};
use embedded_iconoir::size12px::maps::Position;

#[test]
fn main() -> anyhow::Result<()> {
    let mut display = SimulatorDisplay::new(Size::new(250, 150));

    let color = Rgb888::CSS_CYAN;

    // Color is specified during icon creation
    let mut icon_tiny = icons::size12px::development::CodeBracketsSquare::new(color);
    let mut icon_normal = icons::size24px::development::CodeBracketsSquare::new(color);
    let mut icon_large = icons::size48px::development::CodeBracketsSquare::new(color);
    let mut icon_huge = icons::size144px::development::CodeBracketsSquare::new(color);

    // Icons must be wrapped into images to draw them properly
    let image_tiny = Image::new(&icon_tiny, Point::new(10, 10));
    let image_normal = Image::new(&icon_normal, Point::new(24, 10));
    let image_large = Image::new(&icon_large, Point::new(52, 10));
    let image_huge = Image::new(&icon_huge, Point::new(110, 10));

    image_tiny.draw(&mut display)?;
    image_normal.draw(&mut display)?;
    image_large.draw(&mut display)?;
    image_huge.draw(&mut display)?;


    let mut window = Window::new("Cover Example", &OutputSettingsBuilder::new().scale(2).build());
    window.update(&display);

    thread::sleep(Duration::from_secs(1));


    let mut idx = 0u8;
    let colorscheme = colorous::RAINBOW;

    loop {
        let Color { r, g, b } = colorscheme.eval_rational(idx as usize, u8::MAX as usize);
        idx = idx.overflowing_add(1).0;

        // Colors can be changed dynamically during runtime
        icon_tiny.set_color(Rgb888::new(r, g, b));
        icon_normal.set_color(Rgb888::new(r, g, b));
        icon_large.set_color(Rgb888::new(r, g, b));
        icon_huge.set_color(Rgb888::new(r, g, b));

        // But the images need to be recreated each time, because of their reference to the icon
        let image_tiny = Image::new(&icon_tiny, Point::new(10, 10));
        let image_normal = Image::new(&icon_normal, Point::new(24, 10));
        let image_large = Image::new(&icon_large, Point::new(52, 10));
        let image_huge = Image::new(&icon_huge, Point::new(110, 10));

        image_tiny.draw(&mut display)?;
        image_normal.draw(&mut display)?;
        image_large.draw(&mut display)?;
        image_huge.draw(&mut display)?;

        window.update(&display);

        if window.events().fold(false, |acc, evt| acc | matches!(evt, SimulatorEvent::Quit)) {
            break;
        }

        thread::sleep(Duration::from_millis(1000 / 60));
    }

    Ok(())
}
