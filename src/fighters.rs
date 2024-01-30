use bevy::{prelude::*, utils::{HashMap, BoxedFuture}, reflect::{TypePath, TypeUuid}, asset::{AssetLoader, LoadContext, LoadedAsset}};
use serde::{Deserialize, Serialize};

use crate::{actions::{Action, Effect, MovementEffect}, Inputs};

#[derive(Debug, Deserialize, Serialize, TypeUuid, TypePath, Clone)]
#[uuid = "0b336136-5f0c-491b-9d9a-2f7405b002c5"]
pub struct Fighter {
    pub name: String,
    pub moves: Vec<Move>
}

#[derive(Resource)]
pub struct FighterList(pub HashMap<String, Fighter>);

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Move {
    pub name: String,
    pub motion: Vec<u8>,
    pub input: Inputs,
    pub actions: Vec<Action>
}

pub fn get_fighter(character: String, fighter_list: &Res<FighterList>) -> Fighter {
    let fighter = &character.to_lowercase();
    if fighter_list.0.contains_key(fighter) {
        return fighter_list.0[fighter].to_owned();
    } else {
        return get_default_fighter(character);
    }
}

pub fn get_default_fighter(character: String) -> Fighter {
    Fighter {
        name: String::from(character),
        moves: vec![
            Move {
                name: String::from("Dash"),
                motion: vec![5, 6, 5, 6],
                input: Inputs::RIGHT,
                actions: vec![
                    Action {sprite: String::from("Idle"), effects: vec![
                        Effect::Move(
                            MovementEffect{
                                duration: 100, 
                                distance: 1.0, 
                                ease: 1.0, 
                                direction: Vec2::new(1.0, 0.0)
                            }
                        ), 
                        Effect::Wait(100)
                        ], ..Default::default()}
                    ]
            },
            Move {
                name: String::from("DP"),
                motion: vec![6, 2, 3],
                input: Inputs::H,
                actions: vec![]
            },
            Move {
                name: String::from("Spec DP"),
                motion: vec![6, 2, 3],
                input: Inputs::H | Inputs::S,
                actions: vec![]
            },
            Move {
                name: String::from("Super"),
                motion: vec![6, 2, 4, 6],
                input: Inputs::S,
                actions: vec![]
            }
        ]
    }
}

#[derive(Default)]
pub struct FighterLoader;

impl AssetLoader for FighterLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<Fighter>(bytes)?;

            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["fighter.ron"]
    }
}


pub fn fighters_setup(
    asset_server: Res<AssetServer>,
    fighters: Res<Assets<Fighter>>,
    mut fighter_list: ResMut<FighterList>
){
    let _handles: Vec<HandleUntyped> = asset_server.load_folder("./").unwrap();
    for _handle in _handles {
        let handle = _handle.typed_weak();
        if let Some(fighter) = fighters.get(&handle) {
            match asset_server.get_handle_path(handle.to_owned()) {
                Some(assetpath) => {
                    if let Some(name) = assetpath.path().file_name() {
                        if let Some((key, _extension)) = name.to_string_lossy().to_string().to_lowercase().split_once('.') {
                            //Add fighter to list of fighters for character selection
                            fighter_list.0.insert(key.to_owned(), fighter.to_owned());
                        }
                    }
                }
                _ => {}
            }
        }
    }
}