use std::ops::Range;
use crate::pattern_builder::pattern_context::position_map::PositionMap;

pub mod position_map;

#[derive(Clone)]
pub struct PatternContext<'a> {
    num_pixels: usize,
    position_map: PositionMap<'a>,
}

impl PatternContext<'static> {
    pub fn new(num_pixels: usize) -> Self {
        Self {
            num_pixels,
            position_map: PositionMap::new_linear(num_pixels),
        }
    }
}

impl<'a> PatternContext<'a> {
    pub fn slice<'b: 'a>(&'b self, mut range: Range<usize>) -> PatternContext<'b> {
        range.end = usize::min(range.end, self.num_pixels);
        Self {
            num_pixels: range.end - range.start,
            position_map: self.position_map.slice(range),
        }
    }
    pub fn set_num_pixels(&mut self, num_pixels: usize) {
        self.num_pixels = num_pixels;
    }
    pub fn num_pixels(&self) -> usize {
        self.num_pixels
    }
    pub fn set_position_map(&mut self, map: PositionMap<'a>) {
        self.position_map = map;
    }
    pub fn position_map(&self) -> &PositionMap {
        &self.position_map
    }
}

