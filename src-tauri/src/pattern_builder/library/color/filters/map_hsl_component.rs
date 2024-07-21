use itertools::Itertools;
use palette::encoding::Srgb;
use palette::{Alpha, Hsl, IntoColor, SetHue, Srgba};
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
enum HslComponent {
    Hue,
    Saturation,
    Lightness,
}

#[derive(Clone)]
pub struct MapHslComponent {
    map: Prop<LayerStack>,
    component: HslComponent,
}

impl MapHslComponent {
    pub fn new_hue() -> Self {
        Self {
            map: LayerStackPropCore::new(LayerStack::new()).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            component: HslComponent::Hue,
        }
    }
    pub fn new_saturation() -> Self {
        Self {
            map: LayerStackPropCore::new(LayerStack::new()).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            component: HslComponent::Saturation,
        }
    }
    pub fn new_lightness() -> Self {
        Self {
            map: LayerStackPropCore::new(LayerStack::new()).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            component: HslComponent::Lightness,
        }
    }

    pub fn map(&self) -> &Prop<LayerStack> {
        &self.map
    }

    pub fn into_layer(self) -> Layer {
        let component_name = match self.component {
            HslComponent::Hue => "Hue",
            HslComponent::Saturation => "Saturation",
            HslComponent::Lightness => "Value",
        };
        let info = LayerTypeInfo::new(format!("Map {}", component_name).as_str()).with_icon(LayerIcon::Filter);
        Layer::new(self, info)
    }
}

impl LayerCore for MapHslComponent {
    type Input = Frame<ColorPixel>;
    type Output = Frame<ColorPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let hsva_frame: Vec<Alpha<Hsl<Srgb, f64>, f64>> = input.into_iter()
            .map(|pixel| Srgba::from_linear(pixel).into_color())
            .collect();
        let component_frame: Frame<ScalarPixel> = hsva_frame.iter()
            .map(|pixel| match self.component {
                HslComponent::Hue => pixel.hue.into(),
                HslComponent::Saturation => pixel.saturation,
                HslComponent::Lightness => pixel.lightness,
            })
            .collect();
        let mapped_component_frame = self.map.write().next(component_frame, t, ctx)
            .unwrap_or_else(|_err| Frame::empty(ctx.num_pixels()));
        hsva_frame.into_iter()
            .zip_eq(mapped_component_frame)
            .map(|(mut pixel, new_value)| {
                match self.component {
                    HslComponent::Hue => pixel.set_hue(new_value),
                    HslComponent::Saturation => pixel.saturation = new_value,
                    HslComponent::Lightness => pixel.lightness = new_value,
                }
                pixel
            })
            .map(|hsva_pixel| IntoColor::<Srgba<f64>>::into_color(hsva_pixel).into_linear())
            .collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.map,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.map,
        );
    }
}