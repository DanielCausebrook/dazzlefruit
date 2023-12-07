use palette::{LinSrgba, Srgb, Srgba, WithAlpha};
use palette::blend::Compose;
use serde::{Deserialize, Serialize};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    AlphaMask,
}

pub type FrameSize = u16;
pub type Pixel = LinSrgba;
pub type PixelFrame = Vec<Pixel>;

pub trait Frame {
    fn blend(&self, active: PixelFrame, blend_mode: BlendMode) -> PixelFrame;
    fn into_srgb_components(self) -> Vec<(u8, u8, u8)>;
    fn into_srgba_components(self) -> Vec<(u8, u8, u8, u8)>;
}

impl Frame for PixelFrame {
    fn blend(&self, active: PixelFrame, blend_mode: BlendMode) -> PixelFrame {
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
    fn into_srgb_components(self) -> Vec<(u8, u8, u8)> {
        self.into_iter()
            .map(|c| Srgb::<u8>::from_linear(c.premultiply().into()).into_components())
            .collect()
    }
    fn into_srgba_components(self) -> Vec<(u8, u8, u8, u8)> {
        self.into_iter()
            .map(|c| Srgba::<u8>::from_linear(c.into()).into_components())
            .collect()
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
