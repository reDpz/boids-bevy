use rand::{Rng, RngCore};

const BOID_COUNT: usize = 100;
use bevy::{
    prelude::*, render::render_resource::encase::internal::ReadFrom, sprite::MaterialMesh2dBundle,
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
                material: materials.add(Color::srgb(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                )),
                ..default()
            },
            ..default()
        },));
    }
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

fn tick_boids(
    mut query: Query<(&mut Velocity, &mut Transform), With<Boid>>,
    boids_settings: Res<BoidsSettings>,
) {
    // NOTE: different implementations for holding a copy:
    // - There isn't necessarily a need to copy although that would lead to non deterministic
    // behaviour.
    // Read https://discord.com/channels/691052431525675048/1270369289785839667/1270381052237709332

    let copy: Vec<(Velocity, Vec3)> = query
        .iter()
        .map(|r| {
            let velocity = r.0.clone();
            let transform = r.1.translation.clone();
            (velocity, transform)
        })
        .collect();

    for (mut vel_tuple, mut transform) in query.iter_mut() {
        // update positions
        // this is just to make the syntax a little cleaner
        let position = &mut transform.translation;
        let velocity = &mut vel_tuple.0;
        position.x += velocity.x;
        position.y += velocity.y;

        let mut avg_vel = Vec2::splat(0.0);
        let mut boids_invision = 0;

        // iterate over the copy and if in vision range then add to average vel
        for (copy_velocity, position) in copy.iter() {
            avg_vel += copy_velocity.0;
            boids_invision += 1;
        }

        if boids_invision != 0 {
            avg_vel /= boids_invision as f32;
        }

        // for some reason the qat is rotating Ccw so you must negate it
        // Calculate the angle using SOHCAHTOA. Since we want angle from the
        let mut angle = -(velocity.x / velocity.y).atan();
        // reasoning for this being done so is that when Y is negative we get an angle from the
        // nagative Y axis, when we are looking for the positive Y axis angle difference. In such
        // cases you should just flip it by adding 180 degrees (or PI in radians)
        if velocity.y < 0.0 {
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
    avoid_radius: f32,
}

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.)) //update 60 times per second
            .add_systems(Startup, setup)
            .add_systems(Startup, spawn_boid)
            .add_systems(FixedUpdate, tick_boids);
    }
}
