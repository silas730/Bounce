use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision}
};
use std::fs;


#[derive(Component)]
struct BounceObject;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

const WINDOW_HEIGHT: f32 = 576.0;
const WINDOW_WIDTH: f32 = 1024.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Bounce".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .add_startup_system(add_objects)
        .add_system(movement_system.before(collison_system))
        .add_system(collison_system)
        .run();
}

fn add_objects(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //commands.spawn_bundle(UiCameraBundle::default());
    let paths = fs::read_dir("assets/").unwrap();
    let mut x = 10.0;
    for path in paths {
        let path_buff = path.unwrap().path();
        let final_path = path_buff.file_name().unwrap().to_str().unwrap();
        commands
        .spawn()
        .insert(BounceObject)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 0.0),
                translation: Vec3::new(1.0 * x, -60.0, 1.0),
                ..default()
            },
            sprite: Sprite{
                custom_size: Some(Vec2::new(1.0,1.0)),
                ..default()
            },
            texture: asset_server.load(final_path),
            ..default()

        })
        .insert(Velocity(Vec2::new(0.5, -0.5).normalize() * 10.0 * x));
        x = x + 10.0;
    }
    //Walls
    commands
    .spawn()
    .insert(Wall)
    .insert_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(5.0, WINDOW_HEIGHT, 0.0),
            translation: Vec3::new(-WINDOW_WIDTH / 2.0, 0.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()

    })
    .insert(Collider);

    commands
    .spawn()
    .insert(Wall)
    .insert_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(5.0, WINDOW_HEIGHT, 0.0),
            translation: Vec3::new(WINDOW_WIDTH / 2.0, 0.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()

    })
    .insert(Collider);

    commands
    .spawn()
    .insert(Wall)
    .insert_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(WINDOW_WIDTH, 5.0, 0.0),
            translation: Vec3::new(0.0, WINDOW_HEIGHT / 2.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()

    })
    .insert(Collider);

    commands
    .spawn()
    .insert(Wall)
    .insert_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(WINDOW_WIDTH, 5.0, 0.0),
            translation: Vec3::new(0.0, -WINDOW_HEIGHT / 2.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()

    })
    .insert(Collider);
}


fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)> 
){
    let delta_seconds = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * delta_seconds;
        transform.translation.y += velocity.y * delta_seconds;
    }
}

fn collison_system(
    mut object_query: Query<(&mut Velocity, &Transform), With<BounceObject>>,
    collider_query: Query<&Transform, With<Collider>>,
){
    for object in object_query.iter_mut(){
        let (mut object_velocity, object_transform) = object;
        let object_size = object_transform.scale.truncate();

        for transform in collider_query.iter() {
            let collision = collide(
                object_transform.translation,
                object_size,
                transform.translation,
                transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => object_velocity.x = -object_velocity.x,
                    Collision::Right => object_velocity.x = -object_velocity.x,
                    Collision::Top => object_velocity.y = -object_velocity.y,
                    Collision::Bottom => object_velocity.y = -object_velocity.y,
                    Collision::Inside => {},
                }
            }
        }
    }

}