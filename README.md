# embedded-iconoir - Icons for every device, ever.

## What is embedded-iconor?
`embedded-iconoir` is a library that allows you to use [Iconoir](https://github.com/iconoir-icons/iconoir)
on embedded devices , using Rust and the [`embedded-graphics` library](https://github.com/embedded-graphics/embedded-graphics).

It was initially written to be used in the [Kolibri embedded GUI framework](https://github.com/Yandrik/kolibri).

## How many Icons?
All the over 1300 icons offered by [Iconoir](https://github.com/iconoir-icons/iconoir) are also supported in this library.
They are ordered by categories, so if you need a preview look at [the Iconoir Website](https://iconoir.com)
to pick out the exact icon you need.

### How much size does this add to my binary?
Not a lot, actually. Only icons that you actually use are included, and they are tiny.
Every pixel only takes a single bit, so each icon is `resolution^2 / 8` bytes in size.
For `24px` resolution, each icon is only 72 bytes. That means you could fit all 1300+ icons in `24px`
resolution into less than 100kb! That's more than 20% smaller than this extremely over-compressed 
image of a cat:
![](./cat-picture-small.jpg)
([Image Source](https://stocksnap.io/photo/street-art-7KZHK83LSQ))

## Usage

### Selecting a resolution

`embedded-iconoir` gives you feature flags for the available rendering resolutions:
`12px`, `16px`, `18px`, `24px`, `32px`, `48px`, `96px` and `144px`.
You can also enable the `all-resolutions` feature to just get all of them, but note
that that will significantly increase your compile time.

### Using the icons

The icons are structured into modules by **size** and then **category**.
So, `icons::sizeXXpx::CATEGORY::CamelCaseIconName` is how you can select a specific
icon in a specific resolution.
If you're only using one resolution, it's a good idea to import `embedded_iconoir::icons::sizeXXpx::*` so
that you have direct access to all categories.

Here's an example:
```rust
fn main() -> anyhow::Result<()> {
    // init display

    let color = Rgb888::CSS_CYAN;

    // Color is specified during icon creation
    let mut icon_tiny = icons::size12px::development::CodeBracketsSquare::new(color);
    let mut icon_normal = icons::size24px::development::CodeBracketsSquare::new(color);
    let mut icon_large = icons::size48px::development::CodeBracketsSquare::new(color);
    let mut icon_huge = icons::size144px::development::CodeBracketsSquare::new(color);

    // Icons must be wrapped into images to draw them properly
    let image_tiny = Image::new(&icon_tiny, Point::new(10, 10));
    // ...

    image_tiny.draw(&mut display)?;
    // ...


    // Changing colors after creation:
    let mut idx = 0u8;
    let colorscheme = colorous::RAINBOW;

    loop {
        let Color { r, g, b } = colorscheme.eval_rational(idx as usize, u8::MAX as usize);
        let idx = idx.overflowing_add(1).0;

        // Colors can be changed dynamically during runtime
        icon_tiny.set_color(Rgb888::new(r, g, b));
        // ...

        // But the images need to be recreated each time, because of their reference to the icon
        let image_tiny = Image::new(&icon_tiny, Point::new(10, 10));
        // ...

        image_tiny.draw(&mut display)?;
        // ...
    }

    Ok(())
}
```
If you run this, you'll get something like this:

![](./example.gif)

A complete version of this code can also be viewed in 
[`examples/src/bin/cover_example.rs`](https://github.com/Yandrik/embedded-iconoir/blob/main/examples/src/bin/cover_example.rs).


## "But I want a different resolution!"

That is certainly doable. All resolutions are dynamically generated, and the ones that are
available are so because they seemed to fit a fairly broad spectrum of use cases.
If something is missing in your opinion, please open an issue on [GitHub](https://github.com/Yandrik/embedded-iconoir).
Note that bigger resolutions as well as a high amount of different resolutions
(e.g `10px`, `11px`, ..., `47px`, `48px`) will at some point conflict with [crates.io](https://crates.io)'s
max package size of 10MB, as the icons are pre-baked. So, if this is relevant for your use case,
forking the repository and using it as a Git dependency is a better idea.


## Developing locally
To develop `embedded-iconoir` locally, clone the repository, and then execute `git submodule init`
and `git submodule update` to pull `Iconoir` into the repository. After that, run `cargo xtask generate`
to render the icons to `.bits` files, and generate the code.

## Changelog

### 0.2.2: 16px icons

- Icons are now available in `16px` too!
- Rebuilt icons (more icons from `Iconoir` are now available)

### 0.2.1: Marker Trait for Icons

- `IconoirIcon` trait that doesn't rely on `embedded-graphics`' `PixelColor` is now available

### 0.2.0: Compatibility Patch for `embedded-graphics` 0.8

- Updated `embedded-graphics` dependency to `0.8`
- Fixed `embedded-graphics` compatibility issues
- Rebuilt icons (more icons from `Iconoir` are now available)

### 0.1.0: Initial Release

- All icons are available in resolutions `12px`, `18px`, `24px`, `32px`, `48px`, `96px` and `144px`
- Icons can be colored dynamically
- Icons can be drawn on displays using `embedded-graphics`


## Contributing

If you found a bug, or think that a feature is missing, please open an issue on [GitHub](https://github.com/yandrik/embedded-iconoir).
Of course, Pull Requests are also very much appreciated.

All intentional contributions, unless explicitly otherwise specified, are licensed under the MIT license.
