use std::ops::Range;
use nalgebra_glm::DVec3;

pub enum PositionMap<'a> {
    Vec(Vec<DVec3>),
    Slice(&'a[DVec3]),
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
    pub fn new(positions: Vec<DVec3>) -> Self {
        Self::Vec(positions)
    }

    pub fn new_linear(num_pixels: usize) -> Self {
        Self::Vec(
            (0..num_pixels)
                .map(|i| DVec3::new(i as f64,0.0, 0.0 ))
                .collect(),
        )
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
        }.map(|pos| *pos)
    }
}