use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::random;
use serde::Serialize;

use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::watch;
use tokio::time::{interval, MissedTickBehavior};

use crate::{fork_properties};
use crate::pattern_builder::component::{RandId};
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::{DisplayPane, LayerView};
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::layer::standard_types::{COLOR_FRAME, VOID};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::pattern_context::PatternContext;

struct PatternRunnerTask {
    layer: Prop<LayerStack<(), Frame<ColorPixel>>>,
    pattern_context: watch::Receiver<PatternContext<'static>>,
    update_sender: watch::Sender<Frame<ColorPixel>>,
    running: Prop<bool>,
    speed: Prop<f64>,
    t: watch::Sender<f64>,
    last_instant: watch::Sender<Instant>,
}

impl PatternRunnerTask {
    async fn run(self, fps: f32) {
        self.last_instant.send(Instant::now()).unwrap();
        let frame_duration = Duration::from_secs(1).div_f32(fps);
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

            let ctx = self.pattern_context.borrow();
            let pixel_data = self.layer.write().next((), *self.t.borrow(), &ctx)
                .unwrap_or_else(|err| {
                    eprintln!("Failed to evaluate stack: {:?}", err);
                    Frame::<ColorPixel>::empty(ctx.num_pixels())
                });
            self.update_sender.send(pixel_data).unwrap();
        }
    }
}

pub struct Pattern {
    id: RandId,
    name: String,
    animation_runner_handle: JoinHandle<()>,
    frame_receiver: watch::Receiver<Frame<ColorPixel>>,
    fps: f32,
    t: watch::Receiver<f64>,
    last_instant: watch::Receiver<Instant>,
    stack: Prop<LayerStack<(), Frame<ColorPixel>>>,
    pattern_context: watch::Receiver<PatternContext<'static>>,
    running: Prop<bool>,
    speed: Prop<f64>,
    property_view_map: HashMap<RandId, PropView>,
}

impl Pattern {
    pub fn new(name: &str, pattern_context: watch::Receiver<PatternContext<'static>>, fps: f32) -> Self {
        let (update_sender, update_receiver) = watch::channel(Frame::empty(pattern_context.borrow().num_pixels()));
        let (t_send, t_recv) = watch::channel(0.0);
        let (last_instant_send, last_instant_recv) = watch::channel(Instant::now());
        let animation_runner = PatternRunnerTask {
            layer: LayerStackPropCore::new(LayerStack::new(&VOID, &COLOR_FRAME)).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            pattern_context: pattern_context.clone(),
            update_sender,
            running: RawPropCore::new(true).into_prop(PropertyInfo::new("Running")),
            speed: NumPropCore::new_slider(1.0, 0.0..100.0, 0.05).into_prop(PropertyInfo::new("Speed")),
            t: t_send,
            last_instant: last_instant_send,
        };
        Pattern {
            id: random(),
            name: name.to_string(),
            frame_receiver: update_receiver,
            fps,
            t: t_recv,
            last_instant: last_instant_recv,
            stack: animation_runner.layer.clone(),
            pattern_context,
            running: animation_runner.running.clone(),
            speed: animation_runner.speed.clone(),
            animation_runner_handle: spawn(animation_runner.run(fps)),
            property_view_map: HashMap::new(),
        }
    }

    pub fn id(&self) -> RandId {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn view(&mut self) -> PatternView {
        let view = PatternView::new(self);
        self.property_view_map = view.generate_property_map();
        view
    }

    pub fn try_update_prop(&mut self, prop_id: RandId, value: String) -> Result<(), String> {
        let property = self.property_view_map.get_mut(&prop_id).ok_or("Unknown property id")?;
        property.try_update(value.as_str())
    }

    pub fn stack(&self) -> &Prop<LayerStack<(), Frame<ColorPixel>>> {
        &self.stack
    }

    pub fn running(&self) -> &Prop<bool> {
        &self.running
    }

    pub fn get_frame_receiver(&self) -> watch::Receiver<Frame<ColorPixel>> {
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

    fn detach(&mut self) {
        fork_properties!(
            self.stack,
            self.speed,
            self.running,
        );
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        self.animation_runner_handle.abort();
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        let (update_send, update_recv) = watch::channel(self.frame_receiver.borrow().clone());
        let (t_send, t_recv) = watch::channel(self.get_t());
        let (last_instant_send, last_instant_recv) = watch::channel(Instant::now());
        let animation_runner = PatternRunnerTask {
            layer: self.stack.fork(),
            pattern_context: self.pattern_context.clone(),
            update_sender: update_send,
            running: self.running.clone(),
            speed: self.speed.clone(),
            t: t_send,
            last_instant: last_instant_send,
        };
        Pattern {
            id: random(),
            name: self.name.clone(),
            frame_receiver: update_recv,
            fps: self.fps,
            t: t_recv,
            last_instant: last_instant_recv,
            stack: animation_runner.layer.clone(),
            pattern_context: self.pattern_context.clone(),
            running: animation_runner.running.clone(),
            speed: animation_runner.speed.clone(),
            animation_runner_handle: spawn(animation_runner.run(self.fps)),
            property_view_map: HashMap::new(),
        }
    }
}

#[derive(Serialize)]
pub struct PatternView {
    id: RandId,
    root_stack: PropView,
    components: HashMap<RandId, LayerView>,
}

impl PatternView {
    fn new(pattern: &Pattern) -> Self {
        let root_stack = pattern.stack().view();
        let mut layers = vec![];
        let mut current_layers = root_stack.child_layer_views();
        while !current_layers.is_empty() {
            let mut next_layers = vec![];
            for current_layer in current_layers {
                for property in current_layer.property_views() {
                    next_layers.append(&mut property.child_layer_views());
                }
                layers.push(current_layer);
            }
            current_layers = next_layers;
        }
        Self {
            id: pattern.id(),
            root_stack,
            components: layers.into_iter()
                .map(|layer_view| (layer_view.info().id(), layer_view))
                .collect(),
        }
    }
    pub fn generate_property_map(&self) -> HashMap<RandId, PropView> {
        self.components.values()
            .flat_map(|layer_config| layer_config.property_views())
            .map(|prop| (prop.info().id(), prop.clone()))
            .collect::<HashMap<RandId, PropView>>()
    }
}
