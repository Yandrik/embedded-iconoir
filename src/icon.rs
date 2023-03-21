use embedded_graphics::prelude::*;

#[allow(unused)]
macro_rules! get_bit_unsafe {
    ($num:expr, $bit:expr) => {{
        // static_assertions::const_assert!(::std::mem::size($num) <= $bit);
        ($num & { 1 << $bit }) != 0
    }};
}

#[macro_export]
macro_rules! make_icon {
    ($name:ident, $size:expr, $category:expr, $file:expr) => {
        pub struct $name<C: embedded_graphics::pixelcolor::PixelColor> {
            color: C,
        }

        impl<C: ::embedded_graphics::pixelcolor::PixelColor> $crate::icon::Icon<C> for $name<C> {
            #[inline(always)]
            fn new(color: C) -> Self {
                Self { color }
            }

            #[inline(always)]
            fn set_color(&mut self, color: C) {
                self.color = color;
            }

            #[inline(always)]
            fn get_color(&self) -> C {
                self.color
            }
        }

        impl<C: embedded_graphics::pixelcolor::PixelColor> $crate::icon::RawIcon<C> for $name<C> {
            #[inline(always)]
            fn get_data_raw(&self) -> &'static [u8] {
                include_bytes!(concat!(
                    "../rendered/",
                    stringify!($size),
                    "px/",
                    $category,
                    "/",
                    $file,
                    ".bits"
                ))
            }
        }

        // ImageDrawable implementation
        impl<C: ::embedded_graphics::pixelcolor::PixelColor>
            ::embedded_graphics::prelude::ImageDrawable for $name<C>
        {
            type Color = C;

            fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
            where
                D: ::embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
            {
                let data = self.get_data_raw();

                for y in 0..$size {
                    for x in 0..$size {
                        if data.get_bit((x + y * $size) as usize) {
                            ::embedded_graphics::prelude::Pixel(
                                ::embedded_graphics::prelude::Point::new(x as i32, y as i32),
                                self.get_color(),
                            )
                            .draw(target)?;
                        }
                    }
                }
                Ok(())
            }

            fn draw_sub_image<D>(
                &self,
                target: &mut D,
                area: &::embedded_graphics::primitives::Rectangle,
            ) -> Result<(), D::Error>
            where
                D: ::embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
            {
                // from tinytga
                self.draw(&mut target.translated(-area.top_left).clipped(area))
            }
        }

        impl<C: ::embedded_graphics::prelude::PixelColor>
            ::embedded_graphics::prelude::OriginDimensions for $name<C>
        {
            fn size(&self) -> ::embedded_graphics::prelude::Size {
                ::embedded_graphics::prelude::Size {
                    width: $size,
                    height: $size,
                }
            }
        }
        // macro end
    };
}

#[macro_export]
macro_rules! make_icon_category {
    ($name:ident, $size:expr, $category:expr, [ $(($icon_name:ident, $file:expr)),* $(,)? ] ) => {
    paste::paste ! {
        pub mod [<$name:snake>] {
            use super::*;
            $(
            make_icon!( $icon_name, $size, $category, $file);

            )*
        }
    }
    }
}

pub trait Icon<C: PixelColor>: Sized {
    fn new(color: C) -> Self;

    fn set_color(&mut self, color: C);

    fn get_color(&self) -> C;
}

pub(crate) trait RawIcon<C: PixelColor>: Sized + ImageDrawable<Color = C> {
    /// Get the icon's raw data.
    ///
    /// This data will be included using `include_bytes!` in most cases.
    ///
    /// The length of the result slice has to be at least `SIZE * SIZE / 8 + SIZE` (1/8th of the
    /// pixel count, rounded up)
    fn get_data_raw(&self) -> &'static [u8];
}

/*
impl<C, T, const SIZE: u32> Icon<C, T, SIZE> where T: RawIcon<C, SIZE>, C: PixelColor {
    pub fn new(color: C) -> Self {
        Self {
            inner: T::new(color),
            color_marker: PhantomData,
        }
    }

    pub fn set_color(&mut self, color: C) {
        self.inner.set_color(color)
    }
}

impl<C, T, const SIZE: u32> From<T> for Icon<C, T, SIZE> where T: RawIcon<C, SIZE>, C: PixelColor {
    fn from(value: T) -> Self {
        Self {
            inner: value,
            color_marker: PhantomData,
        }
    }
}



impl<C, T, const SIZE: u32> ImageDrawable for Icon<C, T, SIZE> where T: RawIcon<C, SIZE>, C: PixelColor {
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error> where D: DrawTarget<Color=Self::Color> {
        let data = self.inner.get_data_raw();

        for y in 0..SIZE {
            for x in 0..SIZE {
                get_bit_unsafe!(x, 10);
                if data.get_bit((x + y * SIZE) as usize) {
                    println!("Drawing pixel {}, {}", x, y);
                    Pixel(
                        Point::new(x as i32, y as i32),
                        self.inner.get_color(),
                    ).draw(target)?;
                }
            }
        }
        Ok(())


        /*
        self.inner.get_data_raw()   // get data, whereby every bit is a pixel (8 pixels per byte)
            .iter()
            .enumerate()
            /*
            .flat_map(move |(num, eight_pixels)| {
                (0..8).into_iter()  // from 0 to 8:
                    .filter(move |i| (eight_pixels | (0xFF >> i)) == 0)  // filter pixels
                    .map(move |i|
                        {
                            println!("index: {}, num: {}, pixel: ({}, {})", i, num,
                                     ((num + i) as u32 % SIZE) as i32,  // find pixel position in row
                                     ((num + i) as u32 / SIZE).min((SIZE - 1)) as i32,  // find pixel row (and make sure it's not larger than the draw area) (which should be impossible)
                            );
                            Pixel(
                                Point {
                                    x: ((num + i) as u32 % SIZE) as i32,  // find pixel position in row
                                    y: ((num + i) as u32 / SIZE).min((SIZE - 1)) as i32,  // find pixel row (and make sure it's not larger than the draw area) (which should be impossible)
                                },
                                self.inner.get_color(),
                            )
                        }
                    )
            })
            */
            .map(|e| e)
    )*/
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error> where D: DrawTarget<Color=Self::Color> {
        // from tinytga
        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}


impl<C, T, const SIZE: u32> OriginDimensions for Icon<C, T, SIZE> where T: RawIcon<C, SIZE>, C: PixelColor {
    fn size(&self) -> Size {
        Size {
            width: SIZE,
            height: SIZE,
        }
    }
}


*/
