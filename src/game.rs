use bevy::prelude::*;

//TEMP "use" STATEMENTS, SHOULD BE REMOVED WHEN POSSIBLE
use bevy_ggrs::PlayerInputs;
use crate::matchmaking::GGRSConfig;

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
    pub handle: usize //TODO: REMOVE THIS PUB
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

pub fn move_players(inputs: Res<PlayerInputs<GGRSConfig>>, mut players: Query<(&mut Transform, &Player)>) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

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