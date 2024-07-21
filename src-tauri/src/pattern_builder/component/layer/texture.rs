use crate::fork_properties;
use crate::pattern_builder::component::frame::{Blend, BlendMode, Opacity};
use crate::pattern_builder::component::layer::{LayerCore};
use crate::pattern_builder::component::layer::io_type::DynType;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;

use crate::pattern_builder::pattern_context::PatternContext;

pub struct BlendingLayerCore<T> where T: DynType + Blend + Opacity {
    texture: Box<dyn LayerCore<Input=(), Output=T>>,
    blend_mode: Prop<BlendMode>,
    opacity: Prop<f64>,
}

impl<T> BlendingLayerCore<T> where T: DynType + Blend + Opacity {
    pub fn new(texture: impl LayerCore<Input=(), Output=T>) -> Self {
        Self {
            texture: Box::new(texture),
            blend_mode: RawPropCore::new(BlendMode::Normal).into_prop(PropertyInfo::new("Blend Mode")),
            opacity: NumPropCore::new_slider(1.0, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Opacity")),
        }
    }

    pub fn blend_mode(&self) -> &Prop<BlendMode> {
        &self.blend_mode
    }

    pub fn opacity(&self) -> &Prop<f64> {
        &self.opacity
    }
}

impl<T> Clone for BlendingLayerCore<T> where T: DynType + Blend + Opacity {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            blend_mode: self.blend_mode.clone(),
            opacity: self.opacity.clone(),
        }
    }
}

impl<T> LayerCore for BlendingLayerCore<T> where T: DynType + Blend + Opacity {
    type Input = Option<T>;
    type Output = T;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let mut frame = self.texture.next((), t, ctx);
        frame = frame.scale_opacity(*self.opacity.read());
        if let Some(active) = input {
            frame.blend(active, *self.blend_mode().read())
        } else {
            frame
        }
    }

    fn view_properties(&self) -> Vec<PropView> {
        let mut props = self.texture.view_properties();
        props.push(self.opacity.view());
        props.push(self.blend_mode.view());
        props
    }

    fn detach(&mut self) {
        self.texture.detach();
        fork_properties!(
            self.opacity,
            self.blend_mode,
        );
    }
}