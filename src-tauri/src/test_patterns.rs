use tokio::sync::watch;
use nalgebra_glm::DVec3;
use palette::rgb::Rgb;
use palette::{Alpha, Hsl, Hsla, IntoColor, Srgba, WithHue};
use palette::encoding::Srgb;
use std::str::FromStr;
use crate::pattern_builder::component::frame::{Blend, BlendMode, ColorPixel, Frame, Opacity, Pixel, ScalarPixel};
use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::layer::generic::GenericLayer;
use crate::pattern_builder::component::layer::standard_types::{COLOR_FRAME, SCALAR_FRAME, VOID};
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::pattern_builder::component::property::color::ColorPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::num_vec::NumVecPropCore;
use crate::pattern_builder::library::color::filters::alpha_mask::AlphaMask;
use crate::pattern_builder::library::color::filters::map_hsl_component::MapHslComponent;
use crate::pattern_builder::library::color::textures::color_range::ColorRange;
use crate::pattern_builder::library::color::textures::solid_color::SolidColor;
use crate::pattern_builder::library::core::group::Group;
use crate::pattern_builder::library::generic::filters::persistence::Persistence;
use crate::pattern_builder::library::generic::filters::stutter::Stutter;
use crate::pattern_builder::library::scalar::textures::dual_waves::DualWaves;
use crate::pattern_builder::library::scalar::textures::heart::Heart;
use crate::pattern_builder::library::scalar::textures::pulse::Pulse;
use crate::pattern_builder::library::scalar::textures::simplex_noise::SimplexNoise;
use crate::pattern_builder::library::scalar::textures::sparkles::Sparkles;
use crate::pattern_builder::library::transformers::scalar_to_dual_texture::ScalarToDualTexture;
use crate::pattern_builder::library::transformers::scalar_to_texture::ScalarToTexture;
use crate::pattern_builder::math_functions::square_wave;
use crate::pattern_builder::pattern::Pattern;
use crate::pattern_builder::pattern_context::PatternContext;
use crate::{fork_properties, view_properties};

pub fn test_patterns(pattern_context: watch::Receiver<PatternContext<'static>>) -> Vec<Pattern> {
    vec![
        test_pattern(pattern_context.clone()),
        stutter_pulse_pattern(pattern_context.clone()),
        blocks_pattern(pattern_context.clone()),
        simple_wave_pattern(pattern_context.clone()),
        growing_hearts_pattern(pattern_context.clone()),
        solid_color_pattern(pattern_context.clone()),
        single_pixel_pattern(pattern_context.clone()),
        pretty_light_pattern(pattern_context.clone()),
    ]
}

#[derive(Clone)]
pub struct AddValue {
    speed: Prop<f64>,
}

impl AddValue {
    pub fn new(multiplier: f64) -> Self {
        AddValue {
            speed: NumPropCore::new(multiplier).into_prop(PropertyInfo::new("Rate of Change")),
        }
    }

    pub fn into_layer(self) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Add value"), &SCALAR_FRAME, &SCALAR_FRAME)
    }
}

impl LayerCore for AddValue {
    type Input = Frame<ScalarPixel>;
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        input.into_iter()
            .map(|value| value + t * *self.speed.read())
            .collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.speed)
    }

    fn detach(&mut self) {
        fork_properties!(self.speed);
    }
}

#[derive(Clone)]
struct SinglePixel {
    position: Prop<u64>,
}

impl SinglePixel {
    pub fn new(num_pixels: u64) -> Self {
        SinglePixel{
            position: NumPropCore::new_slider(0, 0..num_pixels, 1).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn into_layer(self) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Single Pixel").with_icon(LayerIcon::Texture), &VOID, &SCALAR_FRAME)
    }
}

impl LayerCore for SinglePixel {
    type Input = ();
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let pos = *self.position.read();
        (0..ctx.num_pixels()).map(|x| if x as u64 == pos {1.0} else {0.0})
            .collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.position,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.position
        )
    }
}

fn solid_color_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Solid Color", pattern_context, 60.0);

    pattern.stack().write().push(SolidColor::new(Rgb::from_str("#0000FF").unwrap().into()).into_layer());

    pattern
}

fn single_pixel_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Single Pixel", pattern_context, 60.0);

    pattern.stack().write().push(SinglePixel::new(150).into_layer());


    let transformer = ScalarToTexture::new();
    transformer.texture().write().push(SolidColor::new(Rgb::from_str("#0000FF").unwrap().into()).into_layer());
    pattern.stack().write().push(transformer.into_layer());

    pattern
}

fn test_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Test Pattern", pattern_context, 60.0);

    let waves = DualWaves::new();
    *waves.wave1_scale().write() = 36.0;
    *waves.wave2_scale().write() = 45.0;
    pattern.stack().write().push(waves.into_layer());

    let transformer = ScalarToDualTexture::new();
    transformer.texture_a().write().push(ColorRange::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer());
    transformer.texture_b().write().push(ColorRange::new(Rgb::from_str("#0433FF").unwrap().into()).into_layer());
    pattern.stack().write().push(transformer.into_layer());

    let mask = AlphaMask::new();
    mask.stack().write().push(Pulse::new(4.0, 10.0, 3.0).into_layer());
    mask.stack().write().push(Persistence::new(5.0).into_layer(&SCALAR_FRAME));
    mask.stack().write().push(Sparkles::new(7.0, 5.0).into_layer());
    pattern.stack().write().push(mask.into_layer());

    pattern
}

fn stutter_pulse_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Stutter Pulse", pattern_context, 60.0);

    let green_group = Group::new();
    let noise = SimplexNoise::new(0.5);
    noise.scale().write().x = 1.0/40.0;
    noise.scale().write().y = 1.0/40.0;
    noise.scale().write().z = 1.0/40.0;
    green_group.stack().write().push(noise.into_layer());
    let into_texture = ScalarToTexture::new();
    into_texture.texture().write().push(SolidColor::new(Rgb::from_str("#FEFF66").unwrap().into()).into_layer());
    *into_texture.lower_bound().write() = 0.3;
    *into_texture.upper_bound().write() = 0.35;
    green_group.stack().write().push(into_texture.into_layer());
    pattern.stack().write().push(green_group.into_layer().with_name("Pulses"));

    let sparkles_group = Group::new();

    let range_a = ColorRange::new(Rgb::from_str("#B55748").unwrap().into());
    *range_a.variance().write() = 10.0;
    sparkles_group.stack().write().push(range_a.into_layer());
    let mask = AlphaMask::new();
    mask.stack().write().push(Sparkles::new(7.0, 3.0).into_layer());
    sparkles_group.stack().write().push(mask.into_layer());
    pattern.stack().write().push(sparkles_group.into_layer().with_name("BG Sparkle"));

    let hue_shift = MapHslComponent::new_hue();
    hue_shift.map().write().push(AddValue::new(60.0).into_layer());
    pattern.stack().write().push(hue_shift.into_layer().with_name("Hue Shift"));

    let pulse_group = Group::new();
    pulse_group.stack().write().push(Pulse::new(3.5, 3.0, 8.0).into_layer());
    pulse_group.stack().write().push(Stutter::new_partially_empty(0.026, 0.0, |ctx| Frame::empty(ctx.num_pixels())).into_layer(&SCALAR_FRAME));
    pulse_group.stack().write().push(Persistence::new(2.7).into_layer(&SCALAR_FRAME));
    let texture = ScalarToTexture::new();
    texture.texture().write().push(SolidColor::new(Rgb::from_str("#FFB846").unwrap().into()).into_layer().with_name("Gold"));
    pulse_group.stack().write().push(texture.into_layer());

    pattern.stack().write().push(pulse_group.into_layer().with_name("Stuttering Pulse"));

    pattern
}

pub fn blocks_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Blocks", pattern_context, 60.0);

    let color: Alpha<Hsl<Srgb, f64>, f64> = Hsla::new(0.0, 1.0, 0.5, 1.0);
    let num_colors = 10;

    for x in 0..num_colors {
        let layer = Group::new();
        let noise = SimplexNoise::new(0.8 - (0.3 * (x + 1) as f64 / num_colors as f64));
        noise.scale().write().x = 1.0/30.0;
        noise.scale().write().y = 1.0/30.0;
        noise.scale().write().z = 1.0/30.0;
        layer.stack().write().push(noise.into_layer());
        let into_texture = ScalarToTexture::new();
        let c2 = color.clone().with_hue(color.clone().hue.into_degrees() + (x as f64 * 360.0 / num_colors as f64));
        into_texture.texture().write().push(SolidColor::new(IntoColor::<Srgba<f64>>::into_color(c2).into_linear()).into_layer());
        *into_texture.lower_bound().write() = 0.3;
        *into_texture.upper_bound().write() = 0.35;
        layer.stack().write().push(into_texture.into_layer());
        pattern.stack().write().push(layer.into_layer().with_name(format!("Layer {}", x).as_str()));
    }

    let hue_shift = MapHslComponent::new_hue();
    hue_shift.map().write().push(AddValue::new(20.0).into_layer());
    pattern.stack().write().push(hue_shift.into_layer().with_name("Hue Rotate"));

    pattern
}

fn simple_wave_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Simple Wave", pattern_context, 60.0);

    #[derive(Clone)]
    struct Wave {
        color: Prop<ColorPixel>,
        wavelength: Prop<DVec3>,
        wave_speed: Prop<f64>,
        ratio: Prop<f64>,
        smoothness: Prop<f64>,
    }

    impl Wave {
        pub fn new(color: ColorPixel, wavelength: DVec3, wave_speed: f64) -> Self {
            Wave {
                color: ColorPropCore::new(color).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
                wavelength: NumVecPropCore::new_slider(wavelength, -50.0..50.0, 1.0).into_prop(PropertyInfo::new("Wavelength")),
                wave_speed: NumPropCore::new_slider(wave_speed, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Wave Speed")),
                ratio: NumPropCore::new_slider(0.2, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Wave Ratio")),
                smoothness: NumPropCore::new_slider(0.05, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Wave Smoothness")),
            }
        }

        pub fn into_layer(self) -> TextureLayer {
            TextureLayer::new(self, LayerTypeInfo::new("Wave"))
        }
    }

    impl LayerCore for Wave {
        type Input = ();
        type Output = Frame<ColorPixel>;

        fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
            (0..ctx.num_pixels())
                .map(|x| ctx.position_map().pos(x))
                .map(|o_pos| {
                    if let Some(pos) = o_pos {
                        let magnitude = pos.dot(&self.wavelength.read().scale(1.0 / self.wavelength.read().magnitude()));
                        let mut amount = square_wave(*self.ratio.read(), *self.smoothness.read(), self.wavelength.read().magnitude(), magnitude - t * *self.wave_speed.read());
                        amount = (amount).powf(3.0);
                        self.color.read().scale_opacity(amount)
                    } else {
                        ColorPixel::empty()
                    }
                })
                .collect()
        }

        fn view_properties(&self) -> Vec<PropView> {
            view_properties!(self.color, self.wavelength, self.wave_speed, self.ratio, self.smoothness)
        }

        fn detach(&mut self) {
            fork_properties!(self.color, self.wavelength, self.wave_speed, self.ratio, self.smoothness);
        }
    }

    let color = Rgb::from_str("#FF00E1").unwrap().into();

    pattern.stack().write().push(Wave::new(color, DVec3::new(10.0, 20.0, 0.0), 2.0).into_layer());
    pattern
}

fn growing_hearts_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Heart", pattern_context, 60.0);

    #[derive(Clone)]
    struct GrowingHearts {
        center: Prop<DVec3>,
        width: Prop<f64>,
        grow_speed: Prop<f64>,
        heart_period: Prop<f64>,
        max_size: Prop<f64>,
        hearts: Vec<Heart>,
        last_t: Option<f64>,
        last_heart: Option<f64>,
    }

    impl GrowingHearts {
        pub fn new(center: DVec3) -> Self {
            Self {
                center: NumVecPropCore::new(center).into_prop(PropertyInfo::new("Center")),
                width: NumPropCore::new_slider(2.5, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Width")),
                grow_speed: NumPropCore::new_slider(10.0, 0.0..50.0, 0.5).into_prop(PropertyInfo::new("Grow Speed")),
                heart_period: NumPropCore::new_slider(2.5, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Heart Period")),
                max_size: NumPropCore::new_slider(50.0, 0.0..100.0, 1.0).into_prop(PropertyInfo::new("Max Size")),
                hearts: vec![],
                last_t: None,
                last_heart: None,
            }
        }

        pub fn into_layer(self) -> GenericLayer<Self> {
            GenericLayer::new(self, LayerTypeInfo::new("Growing Hearts").with_icon(LayerIcon::Texture), &VOID, &SCALAR_FRAME)
        }
    }

    impl LayerCore for GrowingHearts {
        type Input = ();
        type Output = Frame<ScalarPixel>;

        fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
            if self.last_heart.unwrap_or(0.0) + *self.heart_period.read() <= t {
                let heart = Heart::new(self.center.read().clone(), 0.0);
                *heart.width().write() = *self.width.read();
                self.hearts.push(heart);

                self.last_heart = Some(t);
            }

            let delta_t = self.last_t.map_or(0.0, |last_t| t - last_t);
            self.last_t = Some(t);

            let grow_speed = *self.grow_speed.read();
            // let width = *self.width.read();
            let max_size = *self.max_size.read();
            self.hearts.retain_mut(|heart| {
                *heart.scale().write() += delta_t * grow_speed;
                let scale = *heart.scale().read();
                // *heart.width().write() = width * 25.0 / scale;
                scale <= max_size
            });

            self.hearts.iter_mut()
                .map(|heart| heart.next((), t, ctx))
                .reduce(|acc, e| e.blend(acc, BlendMode::Normal))
                .unwrap_or(Frame::empty(ctx.num_pixels()))
        }

        fn view_properties(&self) -> Vec<PropView> {
            view_properties!(
                self.center,
                self.width,
                self.grow_speed,
                self.heart_period,
                self.max_size,
            )
        }

        fn detach(&mut self) {
            fork_properties!(
                self.center,
                self.width,
                self.grow_speed,
                self.heart_period,
                self.max_size,
            );
        }
    }

    // pattern.stack().write().push(Heart::new(DVec3::new(7.5, 7.5, 0.0), 15.0).into_layer());
    pattern.stack().write().push(GrowingHearts::new(DVec3::new(7.5, 7.5, 0.0)).into_layer());
    let texture = ScalarToTexture::new();
    texture.texture().write().push(SolidColor::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer());
    pattern.stack().write().push(texture.into_layer());

    pattern
}

pub fn pretty_light_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Pretty Lights", pattern_context, 30.0);

    let white_group = Group::new();
    white_group.stack().write().push(Sparkles::new(0.1, 10.0).into_layer());
    let white_texture = ScalarToTexture::new();
    white_texture.texture().write().push(SolidColor::new(Rgb::from_str("#FFFFFF").unwrap().into()).into_layer());
    white_group.stack().write().push(white_texture.into_layer());
    pattern.stack().write().push(white_group.into_layer());

    let pink_group = Group::new();
    pink_group.stack().write().push(Sparkles::new(0.1, 10.0).into_layer());
    let pink_texture = ScalarToTexture::new();
    pink_texture.texture().write().push(SolidColor::new(Rgb::from_str("#D357FE").unwrap().into()).into_layer());
    pink_group.stack().write().push(pink_texture.into_layer());
    pattern.stack().write().push(pink_group.into_layer());

    pattern.stack().write().push(Persistence::new(0.0).into_layer(&COLOR_FRAME));

    pattern
}
