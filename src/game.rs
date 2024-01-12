use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

const INPUT_UP: u16 = 1 << 0;
const INPUT_DOWN: u16 = 1 << 1;
const INPUT_LEFT: u16 = 1 << 2;
const INPUT_RIGHT: u16 = 1 << 3;
const INPUT_L: u16 = 1 << 4;
const INPUT_M: u16 = 1 << 5;
const INPUT_H: u16 = 1 << 6;
const INPUT_S: u16 = 1 << 7;

#[derive(Component)]
pub struct Player {
    handle: usize
}

#[derive(Component)]
pub struct Movable {
    
}

pub fn spawn_players(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn((
        Player{ handle: 0 },
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            texture_atlas: asset_server.load("Ky-Idle.png"),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        }
    )).add_rollback();
    commands.spawn((
        Player{ handle: 1 },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
            texture: asset_server.load("IdIdle.png"),
            ..default()
        }
    )).add_rollback();
}

pub fn input(
    keyboard_input: Res<Input<KeyCode>>
) -> u16 {
    let mut inp: u16 = 0;

    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        inp |= INPUT_UP;
    }
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        inp |= INPUT_LEFT;
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        inp |= INPUT_DOWN;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        inp |= INPUT_RIGHT;
    }
    
    if keyboard_input.any_pressed([KeyCode::H, KeyCode::Z]) {
        inp |= INPUT_L;
    }
    if keyboard_input.any_pressed([KeyCode::J, KeyCode::X]) {
        inp |= INPUT_M;
    }
    if keyboard_input.any_pressed([KeyCode::K, KeyCode::C]) {
        inp |= INPUT_H;
    }
    if keyboard_input.any_pressed([KeyCode::L, KeyCode::V]) {
        inp |= INPUT_S;
    }

    inp
}

pub fn move_players(inputs: Vec<u16>, mut players: Query<(&mut Transform, &Player)>) {
    for (mut transform, player) in &mut players {

        let input = inputs[player.handle];

        let mut direction = Vec2::ZERO;

        if input & INPUT_UP != 0 {
            direction.y += 1.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 1.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 1.;
        let move_delta = (direction * move_speed).extend(0.);

        transform.translation += move_delta;
    }
}

pub fn gravity() {

}