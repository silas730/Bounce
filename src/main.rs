use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision}
};

#[derive(Component)]
struct SpritePath(String);

#[derive(Component)]
struct Position {x: f32, y: f32}

#[derive(Component)]
struct BounceObject;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(add_objects)
        .add_system(movement_system)
        .run();
}

fn add_objects(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn()
        .insert(BounceObject)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(40.0, -60.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()

        })
        .insert(Velocity(Vec2::new(0.5, -0.5).normalize() * 2.0));
    commands
        .spawn()
        .insert(BounceObject)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(20.0, -50.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()

        })
        .insert(Velocity(Vec2::new(0.5, -0.5).normalize() * 5.0));
}


fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)> 
){
    let delta_seconds = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * delta_seconds;
        transform.translation.y += velocity.y * delta_seconds;
        println!("{:.32}", transform.translation.x);
        println!("{:.32}", transform.translation.y);
        println!();
        
    }
}