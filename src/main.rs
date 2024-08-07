mod boids;
use boids::*;

use bevy::{prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, utils::info};

fn main() {
    App::new()
        // why is it not black by default?
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(
            // use VULKAN instead of DX12/DX11
            RenderPlugin{
                render_creation: WgpuSettings{
                    backends: Some(Backends::VULKAN),..default()
                }.into(),..default()
        }))
        .add_plugins(BoidsPlugin)
        .run();
    println!("Hello, world!");
}
