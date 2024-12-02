use std::f64::consts::E;
use palette::{LinSrgba, Srgb, Srgba, WithAlpha};
use palette::blend::Compose;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
}
pub trait Blend {
    fn blend(self, active: Self, blend_mode: BlendMode) -> Self;
}

pub trait Mix {
    fn mix(self, other: Self, amount: f64) -> Self;
}

pub trait Opacity {
    fn scale_opacity(self, amount: f64) -> Self;
    fn decay_opacity(self, decay_rate: f64, delta_t: f64) -> Self where Self: Sized {
        self.scale_opacity(E.powf(-decay_rate * delta_t))
    }
}

pub trait Pixel: Blend + Mix + Opacity + Clone + Send + Sync + 'static {
    fn empty() -> Self;
}

pub type ColorPixel = LinSrgba<f64>;
const EMPTY_COLOR: ColorPixel = ColorPixel::new(0.0, 0.0, 0.0, 0.0);

impl Blend for ColorPixel {

    fn blend(self, active: Self, blend_mode: BlendMode) -> Self {
        match blend_mode {
            BlendMode::Normal => self.over(active),
        }
    }
}

impl Opacity for ColorPixel {
    fn scale_opacity(self, amount: f64) -> Self {
        self.with_alpha(self.alpha.scale_opacity(amount.clamp(0.0, 1.0)))
    }
}

impl Mix for ColorPixel {
    fn mix(self, other: Self, amount: f64) -> Self {
        palette::Mix::mix(self, other, amount)
    }
}

impl Pixel for ColorPixel {
    fn empty() -> Self {
        EMPTY_COLOR
    }
}

pub type ScalarPixel = f64;

impl Blend for ScalarPixel {

    fn blend(self, active: f64, blend_mode: BlendMode) -> f64 {
        match blend_mode {
            BlendMode::Normal => self + active * (1.0 - self),
            // ScalarBlendMode::Multiply => self * active,
            // ScalarBlendMode::Max => f64::max(self, active),
            // ScalarBlendMode::Min => f64::max(self, active),
        }
    }
}

impl Opacity for ScalarPixel {
    fn scale_opacity(self, amount: f64) -> Self {
        self * amount
    }
}

impl Mix for ScalarPixel {
    fn mix(self, other: Self, amount: f64) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        (1.0 - amount) * self + amount * other
    }
}

impl Pixel for ScalarPixel {
    fn empty() -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Frame<P: Pixel>(Vec<P>);

impl<P: Pixel> Frame<P> {
    pub fn empty(num_pixels: usize) -> Self {
        Self(vec![P::empty();num_pixels])
    }

    pub fn resize_with_empty(&mut self, new_len: usize) {
        self.resize_with(new_len, P::empty);
    }

    pub fn blend_using<F>(self, active: Self, f: F) -> Self where F: Fn(P, P) -> P {
        self.into_iter()
            .zip_longest(active.into_iter())
            .map(|pixels| pixels.or_else(P::empty, P::empty))
            .map(|(pixel, active)| f(pixel, active))
            .collect()
    }
}

impl Frame<ColorPixel> {
    pub fn into_srgb_components(self) -> Vec<(u8, u8, u8)> {
        self.into_iter()
            .map(|c| Srgb::<u8>::from_linear(c.premultiply().into()).into_components())
            .collect()
    }
    pub fn into_srgba_components(self) -> Vec<(u8, u8, u8, u8)> {
        self.into_iter()
            .map(|c| Srgba::<u8>::from_linear(c).into_components())
            .collect()
    }
}

impl<P: Pixel> Blend for Frame<P> {
    fn blend(self, active: Self, blend_mode: BlendMode) -> Self {
        self.blend_using(active, |pixel, active| pixel.blend(active, blend_mode.clone()))
    }
}

impl<P> Opacity for Frame<P> where P: Pixel {
    fn scale_opacity(self, amount: f64) -> Self {
        self.into_iter()
            .map(|p| p.scale_opacity(amount))
            .collect()
    }
}

impl<P> Mix for Frame<P> where P: Pixel {
    fn mix(self, other: Self, amount: f64) -> Self {
        self.blend_using(other, |pixel, other| pixel.mix(other, amount))
    }
}

impl<P: Pixel> Deref for Frame<P> {
    type Target = Vec<P>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Pixel> DerefMut for Frame<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P: Pixel> From<Vec<P>> for Frame<P> {
    fn from(value: Vec<P>) -> Self {
        Self(value)
    }
}

impl<P: Pixel> From<Frame<P>> for Vec<P> {
    fn from(value: Frame<P>) -> Self {
        value.0
    }
}

impl<P: Pixel> IntoIterator for Frame<P> {
    type Item = P;
    type IntoIter = IntoIter<P>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<P: Pixel> FromIterator<P> for Frame<P> {
    fn from_iter<T: IntoIterator<Item=P>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
