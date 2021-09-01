use crate::Player;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct RaycastingPlugin;

#[derive(Default)]
struct RaycastConfig {
    horizontal_raycount: i8,
    vertical_raycount: i8,
    skin_width: f32,
    horizontal_ray_spacing: f32,
    vertical_ray_spacing: f32,
}

impl Plugin for RaycastingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(raycasting_system.system())
            .insert_resource(RaycastConfig {
                horizontal_raycount: 4,
                vertical_raycount: 4,
                ..Default::default()
            })
            .add_plugin(ShapePlugin)
            .add_system(debug_system.system());
    }
}

fn raycasting_system(collider_query: Query<(&Sprite, &Transform), With<Player>>) {
    if let Ok((sprite, transform)) = collider_query.single() {
        let translation = transform.translation;
        let bottom_left = Vec2::new(translation.x, translation.y);
        let bottom_right = Vec2::new(translation.x, translation.y + sprite.size.x);
        let top_left = Vec2::new(translation.x + sprite.size.y, translation.y);
        let top_right = Vec2::new(translation.x + sprite.size.y, translation.y + sprite.size.x);

        println!(
            "BL: {} BR: {} TL: {} TR: {}",
            bottom_left, bottom_right, top_left, top_right
        );
    }
}

fn debug_system(mut commands: Commands) {
    let line = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(300.0, 300.0));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &line, 
        ShapeColors::new(Color::GREEN), 
        DrawMode::Fill(FillOptions::default()), 
        Transform::default()
    ));
}
