use bevy::prelude::*;

struct SnakeHead;

struct Materials {
    head_material: Handle<ColorMaterial>
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", spawn_snake.system())
        .add_system(snake_movement.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into())
    });
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteComponents {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(SnakeHead);
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<With<SnakeHead, &mut Transform>>
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            *transform.translation.x_mut() -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            *transform.translation.x_mut() += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            *transform.translation.y_mut() -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            *transform.translation.y_mut() += 2.;
        }
    }
}
