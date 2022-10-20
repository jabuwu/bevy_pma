pub use bevy::prelude::*;
use bevy::{render::texture::ImageSettings, sprite::Material2dPlugin};

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<vanilla::Material>::default())
        .add_plugin(Material2dPlugin::<corrected::Material>::default())
        .add_startup_system(setup)
        .add_startup_system(vanilla::setup)
        .add_startup_system(corrected::setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub mod corrected;
pub mod image_gen;
pub mod vanilla;
