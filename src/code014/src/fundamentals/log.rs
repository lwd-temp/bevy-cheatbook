use bevy::prelude::*;

fn main1() {
let mut app = App::new();

// ANCHOR: log-settings
use bevy::log::LogPlugin;

app.add_plugins(DefaultPlugins.set(LogPlugin {
    filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
    level: bevy::log::Level::DEBUG,
    ..default()
}));
// ANCHOR_END: log-settings
}

fn main2() {
let mut app = App::new();

// ANCHOR: log-settings-debugrelease
use bevy::log::LogPlugin;

// this code is compiled only if debug assertions are enabled (debug mode)
#[cfg(debug_assertions)]
app.add_plugins(DefaultPlugins.set(LogPlugin {
    level: bevy::log::Level::DEBUG,
    filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
    ..default()
}));

// this code is compiled only if debug assertions are disabled (release mode)
#[cfg(not(debug_assertions))]
app.add_plugins(DefaultPlugins.set(LogPlugin {
    level: bevy::log::Level::INFO,
    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    update_subscriber: None,
}));
// ANCHOR_END: log-settings-debugrelease
}
