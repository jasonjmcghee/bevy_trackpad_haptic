use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::ecs::schedule::MainThreadExecutor;
use bevy::prelude::{Commands, Event, EventReader, Res, ResMut, Resource};
use bevy::tasks::IoTaskPool;
use once_cell::unsync::Lazy;
use trackpad_haptic::FeedbackManager;

pub use trackpad_haptic::Feedback;

thread_local! {
    static FEEDBACK_MANAGER: Lazy<FeedbackManager> = Lazy::new(||
        FeedbackManager::new()
    );
}

pub struct HapticFeedbackPlugin;

impl Plugin for HapticFeedbackPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the main thread marker and NSApplication on startup
        app
            .add_event::<FeedbackEvent>()
            .add_systems(Update, process_feedback);
    }
}

#[derive(Event)]
pub struct FeedbackEvent {
    length_millis: u64,
    delay_millis: u64
}

impl FeedbackEvent {
    pub fn new(length_millis: u64, delay_millis: u64) -> Self {
        Self {
            length_millis,
            delay_millis
        }
    }
}

fn process_feedback(
    mut feedback_event_reader: EventReader<FeedbackEvent>,
) {
    for event in feedback_event_reader.read() {
        let task_pool = IoTaskPool::get();
        let feedback = Feedback::new(
            event.length_millis,
            event.delay_millis
        );
        task_pool.spawn(async move{
            FEEDBACK_MANAGER.with(|feedback_manager| {
                feedback_manager.trigger_with_feedback(feedback);
            });
        }).detach();
    }
}