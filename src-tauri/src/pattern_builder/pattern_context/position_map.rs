use std::fmt::{Formatter, Write};
use std::ops::Range;
use nalgebra_glm::DVec3;
use serde::{de, Deserialize, Deserializer};
use serde::de::{MapAccess, SeqAccess, Visitor};

pub enum PositionMap<'a> {
    Vec(Vec<Option<DVec3>>),
    Slice(&'a[Option<DVec3>]),
}

impl Clone for PositionMap<'_> {
    fn clone(&self) -> Self {
        Self::Vec(
            match self {
                PositionMap::Vec(v) => v.clone(),
                PositionMap::Slice(s) => s.to_vec(),
            }
        )
    }
}

impl PositionMap<'static> {
    pub fn new(positions: Vec<Option<DVec3>>) -> Self {
        Self::Vec(positions)
    }

    pub fn new_linear(num_pixels: usize) -> Self {
        Self::Vec(
            (0..num_pixels)
                .map(|i| Some(DVec3::new(i as f64,0.0, 0.0 )))
                .collect(),
        )
    }
}

impl<'de> Deserialize<'de> for PositionMap<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct PositionMapVisitor;

        impl<'de> Visitor<'de> for PositionMapVisitor {
            type Value = PositionMap<'static>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of nullable 3D vectors")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
                struct DVec3Wrapper(DVec3);
                struct DVec3Visitor;
                
                impl<'de> Visitor<'de> for DVec3Visitor {
                    type Value = DVec3Wrapper;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("a 3D vector")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
                        let x = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let z = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        Ok(DVec3Wrapper(DVec3::new(x, y, z)))
                    }
                }

                impl<'de> Deserialize<'de> for DVec3Wrapper {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                        deserializer.deserialize_seq(DVec3Visitor{})
                    }
                }
                
                let mut positions = vec![];
                while let Some(value) = seq.next_element::<Option<DVec3Wrapper>>()? {
                    positions.push(value.map(|wrapper| wrapper.0))
                }
                Ok(PositionMap::Vec(positions))
            }
        }

        deserializer.deserialize_seq(PositionMapVisitor{})
    }
}


impl<'a> PositionMap<'a> {
    pub fn slice<'b: 'a>(&'b self, range: Range<usize>) -> PositionMap<'b> {
        Self::Slice(
            match self {
                PositionMap::Vec(v) => &v[range],
                PositionMap::Slice(s) => &s[range],
            }
        )
    }
    pub fn pos(&self, pixel_index: usize) -> Option<DVec3> {
        match self {
            PositionMap::Vec(v) => v.get(pixel_index),
            PositionMap::Slice(s) => s.get(pixel_index),
        }.unwrap_or(&None).clone()
    }
}