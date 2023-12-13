use std::time::{Duration, Instant};

use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::watch;
use tokio::time::{interval, MissedTickBehavior};

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, PixelFrame};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::component::TexturePropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};
use crate::pattern_builder::pattern_context::PatternContext;
use crate::pattern_builder::pattern_context::position_map::PositionMap;

const FPS: f32 = 30.0;

struct AnimationRunnerTask {
    layer: Prop<TextureLayer>,
    num_pixels: Prop<usize>,
    position_map: Prop<PositionMap<'static>>,
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

            let num_pixels = *self.num_pixels.read();
            let mut ctx = PatternContext::new(num_pixels);
            let guard = self.position_map.read();
            ctx.set_position_map(guard.slice(0..num_pixels));
            let pixel_data = self.layer.write().next_frame(*self.t.borrow(), &ctx);
            self.update_sender.send(pixel_data).unwrap();
        }
    }
}

pub struct Pattern {
    animation_runner_handle: Option<JoinHandle<()>>,
    frame_receiver: watch::Receiver<PixelFrame>,
    t: watch::Receiver<f64>,
    last_instant: watch::Receiver<Instant>,
    layer: Prop<TextureLayer>,
    num_pixels: Prop<usize>,
    position_map: Prop<PositionMap<'static>>,
    running: Prop<bool>,
    speed: Prop<f64>,
}

impl Pattern {
    pub fn new(mut layer: TextureLayer, num_pixels: usize) -> Self {
        let (update_sender, update_receiver) = watch::channel(
            layer.next_frame(0.0, &PatternContext::new(num_pixels))
        );
        let (t_send, t_recv) = watch::channel(0.0);
        let (last_instant_send, last_instant_recv) = watch::channel(Instant::now());
        let animation_runner = AnimationRunnerTask {
            layer: TexturePropCore::new(layer).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            num_pixels: NumPropCore::new_slider(num_pixels, 0..500, 10).into_prop(PropertyInfo::new("Number of Pixels")),
            position_map: RawPropCore::new(PositionMap::new_linear(num_pixels)).into_prop(PropertyInfo::new("Position Map")),
            update_sender,
            running: RawPropCore::new(true).into_prop(PropertyInfo::new("Running")),
            speed: NumPropCore::new_slider(1.0, 0.0..100.0, 0.05).into_prop(PropertyInfo::new("Speed")),
            t: t_send,
            last_instant: last_instant_send,
        };
        Pattern {
            frame_receiver: update_receiver,
            t: t_recv,
            last_instant: last_instant_recv,
            layer: animation_runner.layer.clone(),
            num_pixels: animation_runner.num_pixels.clone(),
            position_map: animation_runner.position_map.clone(),
            running: animation_runner.running.clone(),
            speed: animation_runner.speed.clone(),
            animation_runner_handle: Some(spawn(animation_runner.run())),
        }
    }

    pub fn layer(&self) -> &Prop<TextureLayer> {
        &self.layer
    }

    pub fn running(&self) -> &Prop<bool> {
        &self.running
    }

    pub fn get_frame_receiver(&self) -> watch::Receiver<PixelFrame> {
        self.frame_receiver.clone()
    }

    pub fn get_t(&self) -> f64 {
        if *self.running.read() {
            *self.t.borrow()
                + Instant::now().duration_since(*self.last_instant.borrow()).as_secs_f64()
                * *self.speed.read()
        } else {
            *self.t.borrow()
        }
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        self.animation_runner_handle.take().unwrap().abort();
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        let (update_send, update_recv) = watch::channel(self.frame_receiver.borrow().clone());
        let (t_send, t_recv) = watch::channel(self.get_t());
        let (last_instant_send, last_instant_recv) = watch::channel(Instant::now());
        let animation_runner = AnimationRunnerTask {
            layer: self.layer.clone(),
            num_pixels: self.num_pixels.clone(),
            position_map: self.position_map.clone(),
            update_sender: update_send,
            running: self.running.clone(),
            speed: self.speed.clone(),
            t: t_send,
            last_instant: last_instant_send,
        };
        Pattern {
            frame_receiver: update_recv,
            t: t_recv,
            last_instant: last_instant_recv,
            layer: animation_runner.layer.clone(),
            num_pixels: animation_runner.num_pixels.clone(),
            position_map: animation_runner.position_map.clone(),
            running: animation_runner.running.clone(),
            speed: animation_runner.speed.clone(),
            animation_runner_handle: Some(spawn(animation_runner.run())),
        }
    }
}

impl Component for Pattern {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.layer,
            self.num_pixels,
            self.speed,
            self.running,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.layer,
            self.num_pixels,
            self.speed,
            self.running,
        );
    }

}
