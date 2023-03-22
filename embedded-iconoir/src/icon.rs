use bit_field::BitField;
use embedded_graphics::prelude::*;
use embedded_graphics_core::primitives::Rectangle;
use static_assertions::const_assert;

#[macro_export]
macro_rules! make_icon {
    ($name:ident, $size:expr, $category:expr, $file:expr) => {
        pub struct $name;

        impl $crate::icon::sealed::IconoirInternal for $name {
            const SIZE: u32 = $size;
            const DATA: &'static [u8] = include_bytes!(concat!(
                "../rendered/",
                stringify!($size),
                "px/",
                $category,
                "/",
                $file,
                ".bits"
            ));
            const INSTANCE: Self = $name;
        }

        const_assert!(<$name as $crate::icon::sealed::IconoirInternal>::DATA.len() >= <$name as $crate::icon::sealed::IconoirInternal>::BYTE_COUNT);
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

make_icon!(SomeIcon, 24, "Animals", "fish");

pub struct Icon<C, Ico>
where
    C: PixelColor,
    Ico: sealed::IconoirInternal,
{
    color: C,
    _icon: Ico,
}

impl<C: PixelColor, Ico: sealed::IconoirInternal> Icon<C, Ico> {
    pub fn new(color: C) -> Self {
        Self {
            color,
            _icon: Ico::INSTANCE,
        }
    }

    pub fn set_color(&mut self, color: C) {
        self.color = color;
    }

    pub fn get_color(&self) -> C {
        self.color
    }
}


pub trait IconoirNewIcon<C: PixelColor>: Sized
where
    Self: sealed::IconoirInternal,
{
    fn new(color: C) -> Icon<C, Self>;
}

impl<C: PixelColor, T> IconoirNewIcon<C> for T
where
    T: sealed::IconoirInternal,
{
    fn new(color: C) -> Icon<C, Self> {
        Icon {
            color,
            _icon: Self::INSTANCE,
        }
    }
}

pub(crate) mod sealed {
    pub trait IconoirInternal: Sized {
        const SIZE: u32;
        const BIT_COUNT: usize = { Self::SIZE as usize * Self::SIZE as usize };
        const BYTE_COUNT: usize = { Self::BIT_COUNT / 8 + if Self::BIT_COUNT % 8 > 0 { 1 } else { 0 } };
        const DATA: &'static [u8];
        const INSTANCE: Self;
    }
}

impl<C, T> ImageDrawable for Icon<C, T>
where
    T: sealed::IconoirInternal,
    C: PixelColor,
{
    type Color = C;
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let data = T::DATA;
        for y in 0..T::SIZE {
            for x in 0..T::SIZE {
                if get_bit_unchecked(data, (x + y * T::SIZE) as usize) {
                    Pixel(Point::new(x as i32, y as i32), self.get_color()).draw(target)?;
                }
            }
        }
        Ok(())
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // from tinytga
        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}

impl<C, T> OriginDimensions for Icon<C, T>
where
    T: sealed::IconoirInternal,
    C: PixelColor,
{
    fn size(&self) -> Size {
        Size {
            width: T::SIZE as u32,
            height: T::SIZE as u32,
        }
    }
}

/// Retrieve the n-th bit from a slice of bytes
/// without performing in-bounds checking
fn get_bit_unchecked(target: &[u8], bit: usize) -> bool {
    let slice_index = bit / 8;
    let bit_index = bit % 8;
    (target[slice_index] & (1 << bit_index)) != 0
}
