use bevy::{core::FixedTimestep, prelude::*, render::texture::{self, Extent3d}};
use imager::render;
use particles::{movement, trail_map};
use rand::Rng;

mod imager;
mod particles;

pub fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(setup.system());

    app.add_system(render.system());
    app.add_system_set(
        SystemSet::new()
            .with_system(movement.system())
            .with_system(trail_map.system())
            .with_run_criteria(FixedTimestep::step(0.05))
    );

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // TODO: add all your other stuff to `app` as usual
    app.insert_resource(WindowDescriptor {
        title: "Slime Simulation".to_string(),
        width: 500.0,
        height: 500.0,
        ..Default::default()
    });

    app.run();
}

pub struct ParticleHandle {
    handle: Handle<Texture>,
}

#[derive(Debug)]
struct Sensor {
    pos: SensorPosition,
    distance: f32,
    angle: f32,
}

#[derive(Debug)]
pub struct SensorControll {
    sensors: [Sensor; 3],
}
impl SensorControll {
    pub fn new(angle: f32, distance: f32) -> Self {
        Self {
            sensors: [
                Sensor {
                    pos: SensorPosition::L,
                    distance,
                    angle: -angle,
                },
                Sensor {
                    pos: SensorPosition::F,
                    distance,
                    angle: 0.,
                },
                Sensor {
                    pos: SensorPosition::R,
                    distance,
                    angle,
                },
            ]
        }
    }
}
#[derive(Debug)]
pub enum SensorPosition {
    L,
    F,
    R,
}

pub struct WorldDimensions {pub width: i32, pub height: i32}
#[derive(Debug)]
pub struct Particle {
    x: i32,
    y: i32,
    heading: f32,
}

pub struct TrailMap {
    pub trail: Vec<f32>,
}

fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let dimensions = WorldDimensions {width: 1000, height: 1000};
    let texture = textures.add(Texture::new(
        Extent3d::new(dimensions.width as u32, dimensions.height as u32, 1),
        texture::TextureDimension::D2,
        vec![255; dimensions.height as usize * dimensions.width as usize * 4],
        texture::TextureFormat::Rgba8Unorm,
    ));

    let particles = ParticleHandle {
        handle: texture.clone(),
    };

    // spawn particles
    spawn_particles(&mut commands, 2000, &dimensions);

    commands.insert_resource(particles);
    let mut trail_map = TrailMap { trail: Vec::<f32>::new() };
    trail_map.trail.resize(dimensions.width as usize * dimensions.height as usize, 0.0);
    commands.insert_resource(trail_map);

    commands.insert_resource(dimensions);

    let material = materials.add(ColorMaterial::texture(texture));
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(500., 500.)),
        material,
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}

fn spawn_particles(
    commands: &mut Commands,
    count: i32,
    dim: &WorldDimensions,
) {
    for _ in 0..=count {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(0..dim.width);
        let y: i32 = rng.gen_range(0..dim.height);
        let heading: f32 = rng.gen_range(0.0..360.0);

        commands
            .spawn()
            .insert(Particle {x, y, heading})
            .insert(SensorControll::new(45., 100.));
    }

}