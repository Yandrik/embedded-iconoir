use criterion::{criterion_group, criterion_main, Criterion};
use embedded_graphics::image::Image;
use embedded_graphics::mock_display::MockDisplay;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{ImageDrawable, Point};
use embedded_graphics::Drawable;
use embedded_iconoir::icons;
use embedded_iconoir::prelude::IconoirNewIcon;

fn criterion_benchmark(c: &mut Criterion) {
    let mut disp = MockDisplay::new();
    disp.set_allow_overdraw(true);
    disp.set_allow_out_of_bounds_drawing(true);

    let mut group = c.benchmark_group("Render Different Resolutions");

    let icon = icons::size144px::actions::AddCircle::new(BinaryColor::On);
    let icon = Image::new(&icon, Point::zero());
    group.bench_function("render 144px icon", |b| {
        b.iter(|| {
            icon.draw(&mut disp).unwrap();
        })
    });

    let icon = icons::size96px::actions::AddCircle::new(BinaryColor::On);
    let icon = Image::new(&icon, Point::zero());
    group.bench_function("render 96px icon", |b| {
        b.iter(|| {
            icon.draw(&mut disp).unwrap();
        })
    });

    let icon = icons::size48px::actions::AddCircle::new(BinaryColor::On);
    let icon = Image::new(&icon, Point::zero());
    group.bench_function("render 48px icon", |b| {
        b.iter(|| {
            icon.draw(&mut disp).unwrap();
        })
    });

    let icon = icons::size24px::actions::AddCircle::new(BinaryColor::On);
    let icon = Image::new(&icon, Point::zero());
    group.bench_function("render 24px icon", |b| {
        b.iter(|| {
            icon.draw(&mut disp).unwrap();
        })
    });

    let icon = icons::size12px::actions::AddCircle::new(BinaryColor::On);
    let icon = Image::new(&icon, Point::zero());
    group.bench_function("render 12px icon", |b| {
        b.iter(|| {
            icon.draw(&mut disp).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
