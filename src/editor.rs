use std::{fs::{self, File}, io::Write};

use crate::{get_default_fighter, AnimationData, FighterList, Inputs, SpriteRes};

use bevy_egui::{egui::{self, load::SizedTexture, Pos2, TextureId, TextureOptions, Vec2}, render_systems::EguiTextureId, EguiContexts};
use bevy::prelude::*;


#[derive(Default, Resource)]
pub struct EditorUiState {
    opened_fighter: String,
    fighter_name: String,
    open_moves: Vec<bool>//replace with MoveUiState that keeps track of if there's an image popup
    //TODO: "Saved!" popup timer
    //TODO: "Couldn't save, error" popup timer/message
}

pub fn editor_open_fighter(ui_state: &mut ResMut<EditorUiState>, name: String){
    ui_state.opened_fighter = name;
    ui_state.open_moves = vec![];
}

pub fn editor_system(mut contexts: EguiContexts, 
    mut ui_state: ResMut<EditorUiState>, 
    mut fighter_list: ResMut<FighterList>, 
    sprites: Res<SpriteRes>, 
    texture_atlases: Res<Assets<TextureAtlas>>, 
    images: Res<Assets<Image>>) {
    if ui_state.opened_fighter == "" {
        egui::Window::new("Fighters").show(contexts.ctx_mut(), |ui| {
            for (key, fighter) in fighter_list.0.iter() {
                if ui.button(key).clicked() {
                    ui_state.opened_fighter = key.to_owned();
                }
            }
            if ui.button("New Fighter").clicked() {
                ui_state.opened_fighter = String::from("New Fighter");
            }
        });
    } else if ui_state.opened_fighter == "New Fighter"{
        egui::Window::new(ui_state.opened_fighter.as_str()).show(contexts.ctx_mut(), |ui| {
            ui.label("Fighter Name: ");
            ui.text_edit_singleline(&mut ui_state.fighter_name);
            ui.label("(Fighter name cannot be empty or \"New Fighter\", and cannot already exist)");
            if ui.button("Save").clicked() {
                if ui_state.fighter_name != "" && ui_state.fighter_name != "New Fighter" {
                    if fs::create_dir(format!("./assets/{}", ui_state.fighter_name)).is_ok() {
                        if let Ok(mut file) = File::create(format!("./assets/{}/{}.fighter.ron", ui_state.fighter_name, ui_state.fighter_name)) {
                            if file.write_all(ron::ser::to_string_pretty(&get_default_fighter(ui_state.fighter_name.to_owned()), ron::ser::PrettyConfig::default()).unwrap().as_bytes()).is_ok() {
                                fighter_list.0.insert(ui_state.fighter_name.to_owned(), get_default_fighter(ui_state.fighter_name.to_owned()));
                                let name = ui_state.fighter_name.to_owned();
                                editor_open_fighter(&mut ui_state, name);
                            }
                        }
                    }
                }
            }
        });
    } else if let Some(fighter) = fighter_list.0.get_mut(&ui_state.opened_fighter) {
        if ui_state.open_moves.len() < fighter.moves.len() {
            ui_state.open_moves = vec![false; fighter.moves.len()];
        }
        if let Some(atlas) = sprites.atlases.get(&fighter.name.to_lowercase()) {
            // a system rendering widgets
            let mut texture_id_opt = contexts.image_id(&texture_atlases.get(&atlas.atlas).unwrap().texture);
            if texture_id_opt.is_none() {
                contexts.add_image(texture_atlases.get(&atlas.atlas).unwrap().texture.cast_weak());
                texture_id_opt = contexts.image_id(&texture_atlases.get(&atlas.atlas).unwrap().texture);
            }
            egui::Window::new(ui_state.opened_fighter.as_str()).show(contexts.ctx_mut(), |ui| {

                let texture_id = texture_id_opt.unwrap();
                let full_size = images.get(&texture_atlases.get(&atlas.atlas).unwrap().texture).unwrap().size();
                let size = texture_atlases.get(&atlas.atlas).unwrap().textures[AnimationData::new(String::from("Idle"), &atlas).get_atlas_index(&atlas, &texture_atlases).unwrap()].size();
                let rect: egui::Rect = egui::Rect::from_min_size(
                    Pos2::new(0.0, 0.0), 
                    Vec2::new(size.x, size.y));
                let uv: egui::Rect = egui::Rect::from_min_max(
                egui::pos2(rect.min.x / full_size.x, rect.min.y / full_size.y),
                egui::pos2(rect.max.x / full_size.x, rect.max.y / full_size.y),
                );
                let imagesource = egui::ImageSource::Texture(
                    SizedTexture::new(
                        texture_id,
                        Vec2::new(size.x, size.y)
                    )
                );
                ui.add(egui::Image::new(imagesource).uv(uv));

                ui.label(format!("Name: {}", fighter.name));
                ui.collapsing("Moves:", |ui| {
                    for (attack, mut state) in fighter.moves.iter().zip(ui_state.open_moves.iter_mut()) {
                        ui.toggle_value(&mut state, &attack.name);
                    }
                });
                if ui.button("Save").clicked() {
                    if ui_state.fighter_name != "" && ui_state.fighter_name != "New Fighter" {
                        ui_state.opened_fighter = ui_state.fighter_name.to_owned();
                    }
                }
            });
        } else {
            panic!("Oy, figure out a way to do default sprites, maybe an \"update sprites\" button while you're at it");
        }
        for (attack, state) in fighter.moves.iter_mut().zip(ui_state.open_moves.iter()) {
            if *state {
                egui::Window::new(format!("Move: {}", attack.name)).show(contexts.ctx_mut(), |ui| {

                    ui.horizontal(|ui| {
                        ui.label(format!("Name:"));
                        ui.text_edit_singleline(&mut attack.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("Input:"));
                        let mut motion = String::new();
                        for motion_part in &attack.motion {
                            motion += &motion_part.to_string();
                        }
                        ui.text_edit_singleline(&mut motion);
                    });
                    ui.horizontal(|ui| {
                        let mut i_L = attack.input.contains(Inputs::L);
                        ui.toggle_value(&mut i_L, "L");
                        let mut i_M = attack.input.contains(Inputs::M);
                        ui.toggle_value(&mut i_M, "M");
                        let mut i_H = attack.input.contains(Inputs::H);
                        ui.toggle_value(&mut i_H, "H");
                        let mut i_S = attack.input.contains(Inputs::S);
                        ui.toggle_value(&mut i_S, "S");
                        let mut i_UP = attack.input.contains(Inputs::UP);
                        ui.toggle_value(&mut i_UP, "UP");
                        let mut i_DOWN = attack.input.contains(Inputs::DOWN);
                        ui.toggle_value(&mut i_DOWN, "DOWN");
                        let mut i_LEFT = attack.input.contains(Inputs::LEFT);
                        ui.toggle_value(&mut i_LEFT, "LEFT");
                        let mut i_RIGHT = attack.input.contains(Inputs::RIGHT);
                        ui.toggle_value(&mut i_RIGHT, "RIGHT");

                        attack.input = Inputs::NONE;
                        if i_L {
                            attack.input |= Inputs::L;
                        }
                        if i_M {
                            attack.input |= Inputs::M;
                        }
                        if i_H {
                            attack.input |= Inputs::H;
                        }
                        if i_S {
                            attack.input |= Inputs::S;
                        }
                        if i_UP {
                            attack.input |= Inputs::UP;
                        }
                        if i_DOWN {
                            attack.input |= Inputs::DOWN;
                        }
                        if i_LEFT {
                            attack.input |= Inputs::LEFT;
                        }
                        if i_RIGHT {
                            attack.input |= Inputs::RIGHT;
                        }
                    });
                    ui.collapsing(format!("{} Actions:", attack.name), |ui| { 
                        for action in &attack.actions {
                            ui.label(format!("Action: {:#?}", action));
                        }
                    });
                });
            }
        }
    } else {
        ui_state.fighter_name = String::new();
    }
}