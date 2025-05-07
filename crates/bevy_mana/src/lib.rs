

use bevy::ecs::event::EventReader;
use bevy::ecs::system::Res;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::app::{Plugin, Update};
use bevy::log::info;
use bevy::time::Time;

pub struct ManaPlugin;

impl Plugin for ManaPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, mana_system);
    }
}


pub fn mana_system(time: Res<Time>, mut input: EventReader<KeyboardInput>) {
    for ev in input.read() {
        info!("{ev:?}")
    }
}
