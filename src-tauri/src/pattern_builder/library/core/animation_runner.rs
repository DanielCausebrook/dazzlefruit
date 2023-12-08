use std::time::{Duration, Instant};

use palette::WithAlpha;
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::watch;
use tokio::time::{interval, MissedTickBehavior};

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, FrameSize, PixelFrame};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::component::TexturePropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};

const FPS: f32 = 30.0;

#[derive(Clone)]
pub struct AnimationRunnerConfig {
    layer: Prop<TextureLayer>,
    num_pixels: Prop<FrameSize>,
    speed: Prop<f64>,
    running: Prop<bool>,
}

impl AnimationRunnerConfig {
    pub fn new(layer: TextureLayer, num_pixels: FrameSize) -> Self {
        Self {
            layer: TexturePropCore::new(layer).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            num_pixels: NumPropCore::new_slider(num_pixels, 0..500, 10).into_prop(PropertyInfo::new("Number of Pixels")),
            speed: NumPropCore::new_slider(1.0, 0.0..100.0, 0.05).into_prop(PropertyInfo::new("Speed")),
            running: RawPropCore::new(true).into_prop(PropertyInfo::new("Running")),
        }
    }
    
    pub fn into_texture(self) -> AnimationRunner {
        AnimationRunner::new(self)
    }
    
    pub fn layer(&self) -> &Prop<TextureLayer> {
        &self.layer
    }
    
    pub fn running(&self) -> &Prop<bool> {
        &self.running
    }
    
}

struct AnimationRunnerTask {
    layer: Prop<TextureLayer>,
    num_pixels: Prop<FrameSize>,
    update_sender: watch::Sender<PixelFrame>,
    running: Prop<bool>,
    speed: Prop<f64>,
    t: watch::Sender<f64>,
    last_instant: watch::Sender<Instant>,
}

impl AnimationRunnerTask {
    async fn run(self) {
        self.last_instant.send(Instant::now()).unwrap();
        let frame_duration = Duration::from_secs(1).div_f32(FPS);
        let mut interval = interval(frame_duration);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
        // enum WaitResult {
        //     Elapsed,
        //     LayerChange,
        //     NotRunning,
        // }
        loop {
            interval.tick().await;
            while !*self.running.read() {
                self.last_instant.send(Instant::now()).unwrap();
                interval.tick().await;
            }
            // 'interval: loop {
            //     let wait_result = select! {
            //         _ = interval.tick() => WaitResult::Elapsed,
            //         _ = self.layer.changed() => WaitResult::LayerChange,
            //         _ = self.running.wait_for(|&r| !r) => WaitResult::NotRunning,
            //     };
            //     match wait_result {
            //         WaitResult::Elapsed => break 'interval,
            //         WaitResult::LayerChange => {
            //             interval.reset();
            //             break 'interval;
            //         },
            //         WaitResult::NotRunning => {
            //             self.running.wait_for(|&r| r).await.unwrap();
            //             interval.reset();
            //             self.last_instant.send(Instant::now()).unwrap();
            //         }
            //     }
            // }

            let now = Instant::now();
            self.t.send_modify(
                |t|
                    *t += now.duration_since(*self.last_instant.borrow()).as_secs_f64()
                        * *self.speed.read()
            );
            self.last_instant.send(now).unwrap();

            let pixel_data = self.layer.write().next_frame(*self.t.borrow(), *self.num_pixels.read());
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
            config.layer.write().next_frame(0.0, *config.num_pixels.read())
        );
        let (t_send, t_recv) = watch::channel(0.0);
        let (last_instant_send, last_instant_recv) = watch::channel(Instant::now());
        let animation_runner = AnimationRunnerTask {
            layer: config.layer.clone(),
            num_pixels: config.num_pixels.clone(),
            update_sender,
            running: config.running.clone(),
            speed: config.speed.clone(),
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
        if *self.config.running.read() {
            *self.t.borrow()
                + Instant::now().duration_since(*self.last_instant.borrow()).as_secs_f64()
                * *self.config.speed.read()
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
        let config = self.config.clone();
        let animation_runner = AnimationRunnerTask {
            layer: config.layer.clone(),
            num_pixels: config.num_pixels.clone(),
            update_sender: update_send,
            running: config.running.clone(),
            speed: config.speed.clone(),
            t: t_send,
            last_instant: last_instant_send,
        };
        AnimationRunner {
            config,
            update_receiver: update_recv,
            t: t_recv,
            last_instant: last_instant_recv,
            animation_runner_handle: Some(spawn(animation_runner.run())),
        }
    }
}

impl Component for AnimationRunner {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.config.layer,
            self.config.num_pixels,
            self.config.speed,
            self.config.running,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.config.layer,
            self.config.num_pixels,
            self.config.speed,
            self.config.running,
        );
    }

}

impl Texture for AnimationRunner {
    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        let mut pixel_data = self.update_receiver.borrow().clone();
        pixel_data.resize_with(num_pixels as usize, || palette::named::BLACK.with_alpha(0.0).into_linear());
        pixel_data
    }
}
