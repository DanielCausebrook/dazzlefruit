use std::time::{Duration, Instant};

use palette::WithAlpha;
use parking_lot::RwLock;
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::select;
use tokio::sync::watch;
use tokio::time::{interval, MissedTickBehavior};
use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::{ComponentInfo, ComponentConfig, Component};

use crate::pattern_builder::component::texture::{Texture};
use crate::pattern_builder::component::data::{DisplayPane, FrameSize, PixelFrame};
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::{BlendModeProperty, BoolProperty};
use crate::pattern_builder::component::property::locked::TextureProperty;
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};
use crate::watch_guard::RWLockWatchReceiver;

const FPS: f32 = 30.0;

#[derive(Clone)]
pub struct AnimationRunnerConfig {
    info: ComponentInfo,
    blend_mode: BlendModeProperty,
    layer: TextureProperty,
    num_pixels: NumProperty<FrameSize>,
    speed: NumProperty<f64>,
    running: BoolProperty,
}

impl AnimationRunnerConfig {
    pub fn new(layer: impl Texture, num_pixels: FrameSize) -> Self {
        Self {
            info: ComponentInfo::new("Animation Runner"),
            blend_mode: BlendModeProperty::default(),
            layer: TextureProperty::new(Box::new(layer), PropertyInfo::unnamed().display_pane(DisplayPane::Tree)),
            num_pixels: NumProperty::new(num_pixels, PropertyInfo::new("Number of Pixels"))
                .set_slider(Some(NumSlider::new(0..500, 10))),
            speed: NumProperty::new(1.0, PropertyInfo::new("Speed"))
                .set_slider(Some(NumSlider::new(0.0..100.0, 0.05))),
            running: BoolProperty::new(true, PropertyInfo::new("Running"))
        }
    }
    
    pub fn into_texture(self) -> AnimationRunner {
        AnimationRunner::new(self)
    }
    
    pub fn get_layer_property(&self) -> &TextureProperty {
        &self.layer
    }
    
    pub fn get_running_property(&self) -> &BoolProperty {
        &self.running
    }
    
}

impl_component_config!(self: AnimationRunnerConfig, self.info, [
    self.layer,
    self.speed,
]);

struct AnimationRunnerTask {
    layer: watch::Receiver<RwLock<Box<dyn Texture>>>,
    num_pixels: watch::Receiver<FrameSize>,
    update_sender: watch::Sender<PixelFrame>,
    running: watch::Receiver<bool>,
    speed: watch::Receiver<f64>,
    t: watch::Sender<f64>,
    last_instant: watch::Sender<Instant>,
}

impl AnimationRunnerTask {
    async fn run(mut self) {
        self.last_instant.send(Instant::now()).unwrap();
        let frame_duration = Duration::from_secs(1).div_f32(FPS);
        let mut interval = interval(frame_duration);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
        enum WaitResult {
            Elapsed,
            LayerChange,
            NotRunning,
        }
        loop {
            'interval: loop {
                let wait_result = select! {
                    _ = interval.tick() => WaitResult::Elapsed,
                    _ = self.layer.changed() => WaitResult::LayerChange,
                    _ = self.running.wait_for(|&r| !r) => WaitResult::NotRunning,
                };
                match wait_result {
                    WaitResult::Elapsed => break 'interval,
                    WaitResult::LayerChange => {
                        interval.reset();
                        break 'interval;
                    },
                    WaitResult::NotRunning => {
                        self.running.wait_for(|&r| r).await.unwrap();
                        interval.reset();
                        self.last_instant.send(Instant::now()).unwrap();
                    }
                }
            }

            let now = Instant::now();
            self.t.send_modify(
                |t|
                    *t += now.duration_since(*self.last_instant.borrow()).as_secs_f64()
                        * *self.speed.borrow()
            );
            self.last_instant.send(now).unwrap();

            let pixel_data = self.layer.write().next_frame(*self.t.borrow(), *self.num_pixels.borrow());
            self.update_sender.send(pixel_data).unwrap();
        }
    }
}

pub struct AnimationRunner {
    config: AnimationRunnerConfig,
    animation_runner_handle: Option<JoinHandle<()>>,
    update_receiver: watch::Receiver<PixelFrame>,
    t: watch::Receiver<f64>,
    last_instant: watch::Receiver<Instant>,
}

impl AnimationRunner {
    pub fn new(config: AnimationRunnerConfig) -> Self {
        let (update_sender, update_receiver) = watch::channel(
            config.layer.write().next_frame(0.0, config.num_pixels.get())
        );
        let (t_send, t_recv) = watch::channel(0.0);
        let (last_instant_send, last_instant_recv) = watch::channel(Instant::now());
        let animation_runner = AnimationRunnerTask {
            layer: config.layer.subscribe(),
            num_pixels: config.num_pixels.subscribe(),
            update_sender,
            running: config.running.subscribe(),
            speed: config.speed.subscribe(),
            t: t_send,
            last_instant: last_instant_send,
        };
        AnimationRunner {
            config,
            update_receiver,
            t: t_recv,
            last_instant: last_instant_recv,
            animation_runner_handle: Some(spawn(animation_runner.run())),
        }
    }

    pub fn config(&self) -> &AnimationRunnerConfig {
        &self.config
    }

    pub fn get_update_receiver(&self) -> watch::Receiver<PixelFrame> {
        self.update_receiver.clone()
    }

    pub fn get_t(&self) -> f64 {
        if self.config.running.get() {
            *self.t.borrow()
                + Instant::now().duration_since(*self.last_instant.borrow()).as_secs_f64()
                * self.config.speed.get()
        } else {
            *self.t.borrow()
        }
    }
}

impl Drop for AnimationRunner {
    fn drop(&mut self) {
        self.animation_runner_handle.take().unwrap().abort();
    }
}

impl Clone for AnimationRunner {
    fn clone(&self) -> Self {
        let (update_send, update_recv) = watch::channel(self.update_receiver.borrow().clone());
        let (t_send, t_recv) = watch::channel(self.get_t());
        let (last_instant_send, last_instant_recv) = watch::channel(Instant::now());
        let animation_runner = AnimationRunnerTask {
            layer: self.config.layer.subscribe(),
            num_pixels: self.config.num_pixels.subscribe(),
            update_sender: update_send,
            running: self.config.running.subscribe(),
            speed: self.config.speed.subscribe(),
            t: t_send,
            last_instant: last_instant_send,
        };
        AnimationRunner {
            config: self.config.clone(),
            update_receiver: update_recv,
            t: t_recv,
            last_instant: last_instant_recv,
            animation_runner_handle: Some(spawn(animation_runner.run())),
        }
    }
}

impl_component!(self: AnimationRunner, self.config, "pixel");

impl Texture for AnimationRunner {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.config.blend_mode
    }

    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        let mut pixel_data = self.update_receiver.borrow().clone();
        pixel_data.resize_with(num_pixels as usize, || palette::named::BLACK.with_alpha(0.0).into_linear());
        pixel_data
    }
}
