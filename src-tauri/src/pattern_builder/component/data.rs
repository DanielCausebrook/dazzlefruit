use palette::{LinSrgba, Srgb, Srgba, WithAlpha};
use palette::blend::Compose;
use serde::{Deserialize, Serialize};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::vec::IntoIter;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    AlphaMask,
}

pub type Pixel = LinSrgba;
#[derive(Clone)]
pub struct PixelFrame(Vec<Pixel>);

impl PixelFrame {
    pub fn empty(num_pixels: usize) -> Self {
        vec![palette::named::BLACK.into_linear().transparent(); num_pixels].into()
    }

    pub fn resize_with_transparent(&mut self, num_pixels: usize) {
        self.0.resize_with(num_pixels, || palette::named::BLACK.into_linear().transparent());
    }

    pub fn blend(&self, active: Self, blend_mode: BlendMode) -> Self {
        match blend_mode {
            BlendMode::Normal => {
                self.iter()
                    .zip(active)
                    .map(|(c1, c2)| c1.over(c2))
                    .collect()
            }
            BlendMode::AlphaMask => {
                self.iter()
                    .zip(active)
                    .map(|(c1, c2)| c2.with_alpha(c2.alpha * c1.alpha))
                    .collect()
            }
        }
    }
    pub fn into_srgb_components(self) -> Vec<(u8, u8, u8)> {
        self.into_iter()
            .map(|c| Srgb::<u8>::from_linear(c.premultiply().into()).into_components())
            .collect()
    }
    pub fn into_srgba_components(self) -> Vec<(u8, u8, u8, u8)> {
        self.into_iter()
            .map(|c| Srgba::<u8>::from_linear(c.into()).into_components())
            .collect()
    }
}

impl Deref for PixelFrame {
    type Target = Vec<Pixel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PixelFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Pixel>> for PixelFrame {
    fn from(value: Vec<Pixel>) -> Self {
        Self(value)
    }
}

impl FromIterator<Pixel> for PixelFrame {
    fn from_iter<T: IntoIterator<Item=Pixel>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for PixelFrame {
    type Item = Pixel;
    type IntoIter = IntoIter<Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(try_from="String", into="String")]
pub struct RandId(u64);

impl Display for RandId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Display::fmt(&self.0, f) }
}

impl TryFrom<String> for RandId {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(RandId(u64::from_str(&value)?))
    }
}

impl From<RandId> for String {
    fn from(value: RandId) -> Self { value.0.to_string() }
}

impl Distribution<RandId> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RandId { RandId(rng.gen()) }
}

#[derive(Copy, Clone, Serialize)]
pub enum DisplayPane {
    Tree,
    Config,
    TreeAndConfig,
}
