use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{ActionComponent, Movable, MovementData};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Action {
    pub animation_speed: f32,
    pub sprite: String,
    pub duration: usize,
    pub start_effects: Vec<Effect>,
    pub effects: Vec<Effect>,
    pub end_effects: Vec<Effect>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Effect {
    Move(MovementEffect),
    Wait(i32),
    WaitForGround,
    SetYSpeed(f32),
    AddYSpeed(f32),
    //SetGravity,
    /*Hitbox,
    ModifyHurtbox,
    SetSprite,
    CallMoveIfDirectionHeld,
    CallMoveIfDirectionPressed,
    CallMoveIfButtonHeld,
    CallMoveIfButtonPressed,
    CallMoveIfButtonReleased,
    Knockdown,
    ResetHitstun*/
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct MovementEffect {
    pub distance: f32,
    pub duration: i32,
    pub ease: f32,
    pub direction: Vec2
}

pub fn parse_actions(mut entities: Query<(&mut ActionComponent, Option<&mut Movable>)>) {
    for (mut actions, mut moveable_opt) in &mut entities {
        let mut action_over = true;
        if let Some(action) = actions.actions.last_mut() {
            if action.start_effects.len() > 0 {
                for effect in &mut action.start_effects {
                    parse_effect(effect, &mut moveable_opt);
                }
                action_over = false;
                action.start_effects.clear();
            } else {
                for effect in &mut action.effects {
                    if parse_effect(effect, &mut moveable_opt) {
                        action_over = false;
                    }
                }
            }
        }
        if action_over {
            if let Some(action) = actions.actions.last_mut() {
                for effect in &mut action.end_effects {
                    parse_effect(effect, &mut moveable_opt);
                }
            }
            actions.actions.pop();
        }
    }
}

fn parse_effect(effect: &mut Effect, mut moveable_opt: &mut Option<Mut<Movable>>) -> bool {
    match effect {
        Effect::Wait(counter) => {
            *counter = *counter - 1;
            if *counter > 0 {
                return true;
            }
        }
        Effect::WaitForGround => {
            if let Some(moveable) = &mut moveable_opt {
                if !moveable.grounded {
                    return true;
                }
            }
        }
        Effect::Move(movement_effect) => {
            if let Some(moveable) = &mut moveable_opt {
                moveable.movements.push(MovementData::from_movement_effect(movement_effect.to_owned()));
            }
        }
        Effect::SetYSpeed(yspeed) => {
            if let Some(moveable) = &mut moveable_opt {
                moveable.yspeed = *yspeed;
            }
        }
        Effect::AddYSpeed(yspeed) => {
            if let Some(moveable) = &mut moveable_opt {
                moveable.yspeed += *yspeed;
            }
        }
        _ => {}
    }
    return false;
}