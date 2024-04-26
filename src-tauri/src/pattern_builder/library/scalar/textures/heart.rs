use nalgebra_glm::{DVec3, smoothstep};
use crate::pattern_builder::component::layer::generic::GenericLayer;
use crate::pattern_builder::component::layer::{LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::layer::standard_types::{SCALAR_FRAME, VOID};
use crate::pattern_builder::component::property::num_vec::NumVecPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{Frame, Pixel, ScalarPixel};
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Heart {
    center: Prop<DVec3>,
    scale: Prop<f64>,
    width: Prop<f64>,
}

impl Heart {
    pub fn new(center: DVec3, scale: f64) -> Self {
        Self {
            center: NumVecPropCore::new(center).into_prop(PropertyInfo::new("Center")),
            scale: NumPropCore::new_slider(scale, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Scale")),
            width: NumPropCore::new_slider(2.5, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Width")),
        }
    }

    pub fn into_layer(self) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Heart").with_icon(LayerIcon::Texture), &VOID, &SCALAR_FRAME)
    }

    pub fn center(&self) -> &Prop<DVec3> {
        &self.center
    }

    pub fn scale(&self) -> &Prop<f64> {
        &self.scale
    }

    pub fn width(&self) -> &Prop<f64> {
        &self.width
    }
}

impl LayerCore for Heart {
    type Input = ();
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        (0..ctx.num_pixels())
            .map(|x| {
                if let Some(pos) = ctx.position_map().pos(x) {
                    let mut vector = pos - *self.center.read();
                    let scale = *self.scale.read();
                    vector = vector.scale(5.0 / scale);
                    let c = vector.x.powf(2.0) + (vector.y - vector.x.powf(2.0).powf(1.0/3.0)).powf(2.0);
                    // x^2 + (y - x^(2/3))^2 = 5
                    let target_c = 5.0;
                    let w2 = *self.width.read() * 25.0 / scale;
                    smoothstep(target_c-w2, target_c, c) - smoothstep(target_c, target_c+w2, c)
                } else {
                    ScalarPixel::empty()
                }
            })
            .collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.center, self.scale, self.width)
    }

    fn detach(&mut self) {
        fork_properties!(self.center, self.scale, self.width);
    }
}

