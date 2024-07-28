mod boids;
use boids::*;

use bevy::{prelude::*, utils::info};

fn main() {
    App::new()
        // why is it not black by default?
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(BoidsPlugin)
        .run();
    println!("Hello, world!");
}
