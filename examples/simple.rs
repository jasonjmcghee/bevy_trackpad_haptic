use bevy::app::{App, Update};
use bevy::MinimalPlugins;
use bevy::prelude::{EventWriter, Res};
use bevy::time::Time;

use bevy_trackpad_haptic::{FeedbackEvent, HapticFeedbackPlugin};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, HapticFeedbackPlugin))
        .add_systems(Update, trigger)
        .run();
}

fn trigger(time: Res<Time>, mut feedback_event_writer: EventWriter<FeedbackEvent>) {
    // Every second, trigger 100ms of feedback
    if time.elapsed_seconds() % 1.0 < 0.016 {
        feedback_event_writer.send(
            FeedbackEvent::new(100, 0)
        );
    }
}