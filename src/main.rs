use bevy::{
    prelude::*, 
    render::pass::ClearColor, 
    sprite::collide_aabb::{collide, Collision},
};

mod systems;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_startup_system(setup.system())
    .add_system(player_controll_system.system())
    .add_system(physics_system.system())
    .add_system(collision_system.system())
    .add_plugin(systems::raycasting::RaycastingPlugin)
    .run();
}

struct Player {
    speed: Vec2,
}

struct Gravity(f32);
struct Velocity{
    x: f32,
    y: f32,
}
struct Grounded(bool);

#[derive(PartialEq)]
enum Collider {
    Solid,
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .insert(Player { 
            speed: Vec2::new(500.0, 200.0),
        })
        .insert(Velocity{x: 0.0, y: 0.0})
        .insert(Gravity(20.0))
        .insert(Grounded(false));

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
}

fn player_controll_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity)>,
) {
    if let Ok((player, mut velocity)) = query.single_mut() {
        let mut direction = Vec2::new(0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }

        velocity.x = time.delta_seconds() * direction.x * player.speed.x;
        velocity.y = time.delta_seconds() * direction.y * player.speed.y;
    }
}

fn collision_system(
    mut player_query: Query<(&Transform, &Sprite, &mut Grounded), With<Player>>,
    collider_query: Query<(&Collider, &Sprite, &Transform)>,
) {
    if let Ok((player_transform, player_sprite, mut player_grounded)) = player_query.single_mut() {
        for (collider, sprite, transform) in collider_query.iter() {
            if Collider::Solid == *collider {
                let collision = collide(
                    player_transform.translation,
                    player_sprite.size,
                    transform.translation,
                    sprite.size
                );
    
                if let Some(colission) = collision {
                    match colission {
                        Collision::Top => {
                            println!("Grounding the player");
                            player_grounded.0 = true;
                            break;
                        },
                        _ => (),
                    }
                } 
                player_grounded.0 = false;    
            }
        }
    }
}

fn physics_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &Gravity, &Grounded)>,
) {
    if let Ok((mut transform, velocity, gravity, grounded)) = query.single_mut() {
        let translation = &mut transform.translation;
        
        if !grounded.0 {
            translation.y -= time.delta_seconds() * gravity.0;
        }

        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.x = translation.x.min(380.0).max(-380.0);
        translation.y += velocity.y;
    }
}