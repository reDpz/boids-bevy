const BOID_COUNT: usize = 10;
use bevy::{
    prelude::*, render::render_resource::encase::internal::ReadFrom, sprite::MaterialMesh2dBundle,
};

#[derive(Component)]
pub struct Boid;

#[derive(Component)]
pub struct Position(pub Vec2);
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Bundle)]
pub struct BoidBundle {
    marker: Boid,
    position: Position,
    velocity: Velocity,
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl Default for BoidBundle {
    fn default() -> Self {
        Self {
            marker: Boid,
            position: Position(Vec2::ZERO),
            velocity: Velocity(Vec2 { x: 0., y: -1. }),
            mesh_bundle: MaterialMesh2dBundle::default(),
        }
    }
}

fn spawn_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((BoidBundle {
        mesh_bundle: MaterialMesh2dBundle {
            mesh: meshes.add(Triangle2d::default()).into(),
            transform: Transform::default().with_scale(Vec3::new(15., 20., 0.)),
            material: materials.add(Color::WHITE),
            ..default()
        },
        ..default()
    },));
}

fn setup(mut commands: Commands) {
    commands.insert_resource(BoidsSettings {
        vision_radius: 40.,
        avoid_radius: 20.,
    });
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

fn tick_boids(mut query: Query<(&mut Position, &mut Velocity, &mut Transform), With<Boid>>) {
    for (mut position, mut velocity, mut transform) in &mut query {
        position.0 += velocity.0;
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;

        // update rotation to point towards direction velocity points to
        // transform.rotation.z = std::f32::consts::PI;
    }
}

fn print_boids(query: Query<(Entity, &Position, &Velocity), With<Boid>>) {
    for (e_id, position, velocity) in &query {
        info!(
            "{}\n-----------\nPos:{}\nVel:{}\n",
            e_id, position.0, velocity.0
        );
    }
}

#[derive(Resource)]
pub struct BoidsSettings {
    vision_radius: f32,
    avoid_radius: f32,
}

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, spawn_boid)
            .insert_resource(Time::<Fixed>::from_hz(60.)) //update 60 times per second
            .add_systems(FixedUpdate, (tick_boids, print_boids).chain());
        //                       seperate print boids later           ^^^^^^^^^^^
    }
}
