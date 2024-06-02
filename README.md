# `bevy_trackpad_haptic`

[![Crates.io](https://img.shields.io/crates/v/bevy_trackpad_haptic.svg)](https://crates.io/crates/bevy_trackpad_haptic)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jasonjmcghee/bevy_trackpad_haptic#license)

A bevy plugin for triggering trackpad haptic feedback on a mac.

I also maintain its core dependency [`trackpad_feedback`](https://github.com/jasonjmcghee/trackpad_haptic).

Try out the example:

```bash
cargo run --example simple
```

## Usage

```rust
fn main() {
    App::new()
        .add_plugins((MinimalPlugins, TrackpadHapticPlugin))
        .add_systems(Update, important_system)
        .run();
}

// Everything important

fn important_system(mut feedback_event_writer: EventWriter<FeedbackEvent>) {
    // Something happens like damage to the player!
    feedback_event_writer.send(
        FeedbackEvent::new(
            // length in millis
            100,
            // minimum delay before additional feedback
            0
        )
    );
}
```

## Compatibility
| bevy | bevy_trackpad_haptic |
|------|----------------------|
| 0.13 | 0.1                  |

## License

* [MIT License](LICENSE)
