// TODO: make 3D

use crate::util;
use crate::util::*;
use rand::{Rng, RngCore};

const BOID_COUNT: usize = 1000;
use bevy::{
    math::VectorSpace, prelude::*, render::render_resource::encase::internal::ReadFrom,
    sprite::MaterialMesh2dBundle,
};

#[derive(Component, Clone)]
pub struct Boid;

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Bundle)]
pub struct BoidBundle {
    marker: Boid,
    velocity: Velocity,
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl Default for BoidBundle {
    fn default() -> Self {
        Self {
            marker: Boid,
            velocity: Velocity(Vec2 { x: 0., y: 0. }),
            mesh_bundle: MaterialMesh2dBundle::default(),
        }
    }
}

fn spawn_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: this is a stupid way of doing it, do it with resources
    let mut rng = rand::thread_rng();

    for _ in 0..BOID_COUNT {
        let mut color = Vec3 {
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
        };
        color = color.normalize_or(Vec3::splat(1.0));
        commands.spawn((BoidBundle {
            velocity: Velocity(
                Vec2 {
                    x: rng.gen_range(-1.0..1.0),
                    y: rng.gen_range(-1.0..1.0),
                    /* x: -1.0,
                    y: -1.0, */
                }
                .normalize_or_zero(),
            ),

            mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes
                    .add(Triangle2d {
                        vertices: [
                            Vec2::new(-5.0, -7.5),
                            Vec2::new(5.0, -7.5),
                            Vec2::new(0.0, 7.5),
                        ],
                    })
                    .into(),
                transform: Transform::from_xyz(
                    rng.gen_range(-100.0..100.0),
                    rng.gen_range(-100.0..100.0),
                    rng.gen_range(-1.0..1.0),
                )
                .with_scale(Vec3::new(1., 1., 0.)),
                material: materials.add(Color::srgb(color.x, color.y, color.z)),
                ..default()
            },
            ..default()
        },));
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(BoidsSettings {
        vision_radius: 80.,
        avoid_radius: 10.,
        speed: 4.0,
        flock_strength: 0.1,
        cohesion_strength: 0.03,
        avoid_strength: 0.1,
        steer_away_strength: 0.1,
    });
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

// TODO: ensure entities are not the same
fn tick_boids(
    mut query: Query<(&mut Velocity, &mut Transform, Entity), With<Boid>>,
    boids_settings: Res<BoidsSettings>,
) {
    // NOTE: different implementations for holding a copy:
    // - There isn't necessarily a need to copy although that would lead to non deterministic
    // behaviour.
    // Read https://discord.com/channels/691052431525675048/1270369289785839667/1270381052237709332

    // make a copy of boids
    let copy: Vec<(Velocity, Vec3, Entity)> = query
        .iter()
        .map(|r| (r.0.clone(), r.1.translation, r.2))
        .collect();

    for (mut vel_tuple, mut transform, entity) in query.iter_mut() {
        // update positions
        // this is just to make the syntax a little cleaner

        let mut avg_vel = Vec2::splat(0.0);
        let mut avg_pos = Vec2::splat(0.0);
        let mut avg_avoid_pos = Vec2::splat(0.0);

        // might be advisable to use u32
        let mut boids_in_vision: u16 = 0;
        let mut boids_in_avoid: u16 = 0;

        let raycast_pos = Vec2 {
            x: transform.translation.x + vel_tuple.0.x * boids_settings.vision_radius,
            y: transform.translation.y + vel_tuple.0.y * boids_settings.vision_radius,
        };

        // iterate over the copy and if in vision range then add to average vel
        for (copy_velocity, copy_position, copy_entity) in copy.iter() {
            let distance = dist_vec3(&transform.translation, copy_position);

            // ensure we arent comparing the same entity
            if *copy_entity != entity && distance <= boids_settings.vision_radius {
                if distance <= boids_settings.avoid_radius {
                    avg_avoid_pos += copy_position.xy();
                    boids_in_avoid += 1;
                }
                avg_vel += copy_velocity.0;
                avg_pos += copy_position.xy();
                boids_in_vision += 1;
            }
        }

        if boids_in_vision != 0 {
            avg_vel /= boids_in_vision as f32;
            avg_pos /= boids_in_vision as f32;
            // go towards average velocity (FLOCKING)
            vel_tuple.0 = vel_tuple
                .0
                .lerp(avg_vel, boids_settings.flock_strength)
                .normalize_or_zero();

            // go towards flock center (COHESION)
            let offset = (avg_pos - transform.translation.xy()).normalize_or_zero();
            vel_tuple.0 = vel_tuple
                .0
                .lerp(offset, boids_settings.cohesion_strength)
                .normalize_or_zero();
        }

        if boids_in_avoid != 0 {
            avg_avoid_pos /= boids_in_avoid as f32;

            // calculate this as an inverse offset
            let inv_offset = transform.translation.xy() - avg_avoid_pos;

            vel_tuple.0 = vel_tuple
                .0
                .lerp(inv_offset, boids_settings.avoid_strength)
                .normalize_or_zero();
        }

        // TODO: calculate this live
        let camera_bounds = util::Rect {
            position: Vec2 { x: 0.0, y: 0.0 },
            width: 1920.0,
            height: 1080.0,
        };

        // raycast and check if new position is outside camera bounds
        if !camera_bounds.is_point_in(&raycast_pos) {
            /* info!(
                "looking out of bounds\n position:{:?}\n ray_castpos: {raycast_pos}",
                transform.translation
            ); */

            // steer away

            // just make it face towards center
            let towards_center = camera_bounds.position
                - Vec2 {
                    x: transform.translation.x,
                    y: transform.translation.y,
                };
            vel_tuple.0 = vel_tuple.0.lerp(
                towards_center.normalize_or_zero(),
                boids_settings.steer_away_strength,
            )
        }

        // move with velocity
        transform.translation.x += vel_tuple.0.x * boids_settings.speed;
        transform.translation.y += vel_tuple.0.y * boids_settings.speed;
    }
}

fn rotate_boids(mut query: Query<(&mut Transform, &Velocity), With<Boid>>) {
    for (mut transform, vel_tuple) in query.iter_mut() {
        // for some reason the qat is rotating Ccw so you must negate it
        // Calculate the angle using SOHCAHTOA. Since we want angle from the
        let mut angle = -(vel_tuple.0.x / vel_tuple.0.y).atan();
        // reasoning for this being done so is that when Y is negative we get an angle from the
        // nagative Y axis, when we are looking for the positive Y axis angle difference. In such
        // cases you should just flip it by adding 180 degrees (or PI in radians)
        if vel_tuple.0.y < 0.0 {
            angle += std::f32::consts::PI;
        }
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn print_boids(query: Query<(Entity, &Transform, &Velocity), With<Boid>>) {
    for (e_id, transform, velocity) in &query {
        info!(
            "{}\n-----------\nPos:{}\nVel:{}\n",
            e_id, transform.translation, velocity.0
        );
    }
}

#[derive(Resource)]
pub struct BoidsSettings {
    vision_radius: f32,
    /// Should be smaller than vision_radius
    avoid_radius: f32,
    speed: f32,

    flock_strength: f32,
    cohesion_strength: f32,
    avoid_strength: f32,
    steer_away_strength: f32,
}

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.)) //update 60 times per second
            .add_systems(Startup, setup)
            .add_systems(Startup, spawn_boid)
            .add_systems(FixedUpdate, tick_boids)
            .add_systems(Update, rotate_boids);
    }
}
