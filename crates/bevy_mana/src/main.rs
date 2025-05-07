use bevy::{
    a11y::AccessibilityPlugin, app::ScheduleRunnerPlugin, asset::{io::{AssetSourceBuilders, AssetSources}, AssetServerMode, UnapprovedPathMode}, input::InputPlugin, prelude::*, window::{self, PresentMode, WindowTheme}, winit::{WakeUp, WinitPlugin}
};

fn main() {
    App::new()
        .add_plugins(
            (
                AssetPlugin::default(),
                AccessibilityPlugin,
                InputPlugin,
                WinitPlugin::<WakeUp>::default(),
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy mana".into(),
                        name: Some("bevy.mana".into()),
                        resolution: (500., 300.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells Wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        ..default()
                    }),
                    ..default()
                },
            ),
        )
        .run();
}
