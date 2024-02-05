use std::{fs::{self, File}, io::Write};

use crate::{get_default_fighter, FighterList};

use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;


#[derive(Default, Resource)]
pub struct EditorUiState {
    opened_fighter: String,
    fighter_name: String,
    //TODO: "Saved!" popup timer
    //TODO: "Couldn't save, error" popup timer/message
}

pub fn editor_system(mut contexts: EguiContexts, mut ui_state: ResMut<EditorUiState>, fighter_list: Res<FighterList>) {
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
                                ui_state.opened_fighter = ui_state.fighter_name.to_owned();
                            }
                        }
                    }
                }
            }
        });
    } else {
        egui::Window::new(ui_state.opened_fighter.as_str()).show(contexts.ctx_mut(), |ui| {
            ui.label("Fighter Name: ");
            ui.text_edit_singleline(&mut ui_state.fighter_name);
            ui.label("(Fighter name cannot be empty or \"New Fighter\")");
            if ui.button("Save").clicked() {
                if ui_state.fighter_name != "" && ui_state.fighter_name != "New Fighter" {
                    ui_state.opened_fighter = ui_state.fighter_name.to_owned();
                }
            }
        });
    }
}