use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::pattern_builder::component::property::PropView;

pub mod frame;
mod macros;
// pub mod shared_component;
pub mod property;
pub mod layer;

pub trait Component: Send + Sync + 'static {
    fn view_properties(&self) -> Vec<PropView>;
    fn detach(&mut self);
}

impl<T> Component for Box<T> where T: Component + ?Sized {
    fn view_properties(&self) -> Vec<PropView> {
        self.as_ref().view_properties()
    }
    fn detach(&mut self) {
        self.as_mut().detach()
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
