use std::collections::VecDeque;

use bevy::{core::{Pod, Zeroable}, prelude::*};
use bevy_ggrs::AddRollbackCommandExtension;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::{actions::{Action, MovementEffect}, fighters::{get_fighter, Fighter, FighterList}, AnimationData, SpriteRes};

#[derive(Debug, Copy, Clone, Pod, Zeroable, PartialEq, Eq, Default, Deserialize, Serialize)]
#[repr(C)]
pub struct Inputs(u16);

impl Inputs {
    pub fn movement_input(&self) -> Inputs {
        *self & (Inputs::LEFT | Inputs::RIGHT | Inputs::DOWN | Inputs::UP)
    }
    pub fn has(&self, other: &Inputs) -> bool {
        *self & *other == *other
    }
    pub fn has_dir(&self, other: &u8) -> bool {
        self.movement_input() == match other {
            1 => {
                Inputs::LEFT | Inputs::DOWN
            }
            2 => {
                Inputs::DOWN
            }
            3 => {
                Inputs::RIGHT | Inputs::DOWN
            }
            4 => {
                Inputs::LEFT
            }
            5 => {
                Inputs::NONE
            }
            6 => {
                Inputs::RIGHT
            }
            7 => {
                Inputs::LEFT | Inputs::UP
            }
            8 => {
                Inputs::UP
            }
            9 => {
                Inputs::RIGHT | Inputs::UP
            }
            _ => {
                warn!("TRIED TO MATCH NON-NUMPAD DIR INPUT: {}", other);
                Inputs::NONE
            }
        }
    }

    pub fn has_dir_loose(&self, other: &u8) -> bool {
        self.movement_input() & match other {
            1 => {
                Inputs::LEFT | Inputs::DOWN
            }
            2 => {
                Inputs::DOWN
            }
            3 => {
                Inputs::RIGHT | Inputs::DOWN
            }
            4 => {
                Inputs::LEFT
            }
            5 => {
                Inputs::NONE
            }
            6 => {
                Inputs::RIGHT
            }
            7 => {
                Inputs::LEFT | Inputs::UP
            }
            8 => {
                Inputs::UP
            }
            9 => {
                Inputs::RIGHT | Inputs::UP
            }
            _ => {
                warn!("TRIED TO MATCH NON-NUMPAD DIR INPUT: {}", other);
                Inputs::NONE
            }
        } != Inputs::NONE
    }
}

bitflags! {
    impl Inputs: u16 {
        const NONE = 0;
        const UP = 1 << 0;
        const DOWN = 1 << 1;
        const LEFT = 1 << 2;
        const RIGHT = 1 << 3;
        const L = 1 << 4;
        const M = 1 << 5;
        const H = 1 << 6;
        const S = 1 << 7;
        const BUFFERCLEAR = 0xFF;
    }
}

#[derive(Component)]
pub struct Player {
    handle: usize,
    fighter: Fighter
}

#[derive(Component, Default)]
pub struct ActionComponent {
    pub actions: Vec<Action>
}

#[derive(Component, Default)]
pub struct Movable {
    pub input: GameInput,
    pub movements: Vec<MovementData>,
    pub grounded: bool,
    pub facing: FacingDirection,
    pub yspeed: f32,
    pub gravity: f32,
}

#[derive(Default)]
pub enum FacingDirection {
    #[default]
    Right,
    Left
}

#[derive(Default, Clone, Copy)]
pub struct MovementData {
    distance: f32,
    duration: i32,
    ease: f32,
    frame: i32,
    direction: Vec2
}

impl MovementData {
    pub fn new(distance: f32, duration: i32, ease: f32, frame: i32, direction: Vec2) -> MovementData {
        return MovementData { distance, duration, ease, frame: frame, direction }
    }
    pub fn from_movement_effect(movement_effect: MovementEffect) -> MovementData {
        return MovementData::new(movement_effect.distance, movement_effect.duration, movement_effect.ease, 0, movement_effect.direction);
    }
}

#[derive(Default)]
pub struct GameInput {
    input_log: VecDeque<Inputs>,
    smash_log: VecDeque<Inputs>,
}

pub fn spawn_players(mut commands: Commands, sprites: Res<SpriteRes>, fighter_list: Res<FighterList>){
    spawn_player(&mut commands, &sprites, &fighter_list, 0, Vec3::new(-50., 0., 0.), "Ky".to_owned(), "Idle".to_owned());
    spawn_player(&mut commands, &sprites, &fighter_list, 1, Vec3::new(50., 0., 0.), "Id".to_owned(), "Idle".to_owned());
}

pub fn spawn_player(commands: &mut Commands, sprites: &Res<SpriteRes>, fighter_list: &Res<FighterList>, handle: usize, position: Vec3, character: String, starting_animation: String){
    //TODO: make a default invisible "loading" sprite instead of grabbing the atlas manually
    if let Some(atlas) = sprites.atlases.get(&character.to_lowercase()) {
        commands.spawn((
            Player{ handle: handle, fighter: get_fighter(character.to_owned(), fighter_list) },
            Movable { ..default() },
            ActionComponent { ..default() },
            AnimationData::new(character, starting_animation, atlas),
            SpriteSheetBundle {
                transform: Transform::from_translation(position),
                texture_atlas: atlas.atlas.to_owned(),
                sprite: TextureAtlasSprite::new(0),
                ..default()
            }
        )).add_rollback();
    }
}

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>, 
) -> Inputs {
    let mut inp: Inputs = Inputs::NONE;
    
    //https://github.com/bevyengine/bevy/blob/release-0.11.3/examples/input/gamepad_input.rs
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::West)) {
            inp |= Inputs::L;
        }
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {
            inp |= Inputs::M;
        }
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::East)) {
            inp |= Inputs::H;
        }
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            inp |= Inputs::S;
        }
        
        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_x > 0.25 {
            inp |= Inputs::RIGHT;
        }
        if left_stick_x < -0.25 {
            inp |= Inputs::LEFT;
        }
        if left_stick_y > 0.25 {
            inp |= Inputs::UP;
        }
        if left_stick_y < -0.25 {
            inp |= Inputs::DOWN;
        }

        /*
        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
        */
    }
    
    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        inp |= Inputs::UP;
    }
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        inp |= Inputs::LEFT;
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        inp |= Inputs::DOWN;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        inp |= Inputs::RIGHT;
    }
    
    if keyboard_input.any_pressed([KeyCode::H, KeyCode::Z]) {
        inp |= Inputs::L;
    }
    if keyboard_input.any_pressed([KeyCode::J, KeyCode::X]) {
        inp |= Inputs::M;
    }
    if keyboard_input.any_pressed([KeyCode::K, KeyCode::C]) {
        inp |= Inputs::H;
    }
    if keyboard_input.any_pressed([KeyCode::L, KeyCode::V]) {
        inp |= Inputs::S;
    }

    inp
}

pub fn set_player_input(inputs: Vec<Inputs>, mut players: Query<(&mut Movable, &Player, &mut ActionComponent)>) {
	let log_length = 30; //how long the motion buffer should last
	let buffer_length = 1; //how long buffered moves should buffer for (for getups and cancels and such)
    
    for (mut movable, player, mut actions) in &mut players {

        let mut smash_input = inputs[player.handle];
        if let Some(last_input) = movable.input.input_log.back(){
            smash_input &= !*last_input;
        }
        movable.input.smash_log.push_back(smash_input);
        if movable.input.smash_log.len() > log_length {
            movable.input.smash_log.pop_front();
        }
        movable.input.input_log.push_back(inputs[player.handle]);
        if movable.input.input_log.len() > log_length {
            movable.input.input_log.pop_front();
        }
        
        for potentialmove in &player.fighter.moves {
            for buffered_input in movable.input.smash_log.to_owned().iter().rev().take(buffer_length) {
                if *buffered_input == Inputs::BUFFERCLEAR {
                    break;
                }
                if buffered_input.has(&potentialmove.input) {
                    let mut input_iter = potentialmove.motion.iter().rev().peekable();
                    let mut previous_motion_input = 5;
                    let mut previous_input = Inputs::NONE;


                    //make it so moves without a button to press take the last button press as the input
                    if !(potentialmove.input == Inputs::NONE && 
                        movable.input.input_log.back().is_some() && 
                        *movable.input.input_log.back().unwrap() == Inputs::NONE) {
                        for input in movable.input.input_log.iter().rev() {
                            if *input == Inputs::BUFFERCLEAR {
                                break;
                            }
                            if let Some(iter_input) = input_iter.next_if(
                                |&x| input.has_dir(x) || (*x != 5 && (input.movement_input() & previous_input).has_dir(x))
                            ) {
                                previous_motion_input = *iter_input;
                            } else {
                                if !(input.has_dir(&5) || 
                                (input_iter.peek().is_some() && input.has_dir_loose(input_iter.peek().unwrap())) || 
                                input.has_dir_loose(&previous_motion_input)) {
                                    break;
                                }
                            }
                            previous_input = input.movement_input();
                        }
                    }
                    
                    if input_iter.peek().is_none() { 
                        if actions.actions.len() == 0 {
                            println!("Move {} started!", potentialmove.name);
                            actions.actions = potentialmove.actions.to_owned();

                            if potentialmove.input != Inputs::NONE {
                                movable.input.smash_log.push_back(Inputs::BUFFERCLEAR);
                                movable.input.input_log.push_back(Inputs::BUFFERCLEAR);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn movable_system(mut movables: Query<(&mut Transform, &mut Movable)>) {
    for (mut transform, mut movable) in &mut movables {
        let mut move_delta = Vec2::ZERO;
        movable.movements.retain_mut(|movement_data| {
            let &mut MovementData { distance, duration, ease, frame, direction } = movement_data;
            if ease > 0.0 {
                move_delta += distance * ((1.0 - (frame as f32) / (duration as f32)).powf(ease) - (1.0 - (frame as f32 + 1.0) / (duration as f32)).powf(ease)) * direction;
            } else {
                move_delta += distance * ((1.0 - (frame as f32 + 1.0) / (duration as f32)).powf(-ease) - (1.0 - (frame as f32) / (duration as f32)).powf(-ease)) * direction;
            }
            movement_data.frame += 1;
            return movement_data.frame < duration;
        });
        movable.yspeed += movable.gravity;
        move_delta.y += movable.yspeed;

        //add better collision here
        transform.translation += move_delta.extend(0.0);
        if transform.translation.y <= -50.0 {
            transform.translation.y = -50.0;
            movable.grounded = true;
        } else {
            movable.grounded = false;
        }

        movable.gravity = -9.8/15.;
    }
}

pub fn soft_collision() {

}

pub fn hard_collision() {

}