mod backend;
mod editor;
mod game;
mod netcode;
mod menu;
mod fighters;
mod actions;

use crate::game::*;
use crate::editor::*;
use crate::netcode::*;
use crate::menu::*;
use crate::fighters::*;
use crate::actions::*;
use backend::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy::asset::LoadState;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::{GgrsAppExtension, GgrsPlugin, GgrsSchedule};
use bevy_egui::EguiPlugin;

const FPS: usize = 60;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Editor,
    Gameplay,
}

#[derive(Resource, Default)]
struct FileHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteSheetBundle,
}

fn main() {
    let mut app = App::new();

    app
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_state::<NetworkState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_ggrs_plugin(
            GgrsPlugin::<GGRSConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(network_input)
            .register_rollback_component::<Transform>()
            //.register_rollback_component::<ActionComponent>()
            //.register_rollback_component::<Movable>()
            //.register_rollback_component::<Checksum>()
            //.register_rollback_resource::<FrameCount>()
        )

        .add_asset::<AnimationInfo>()
        .init_asset_loader::<AnimationInfoLoader>()

        .add_asset::<Fighter>()
        .init_asset_loader::<FighterLoader>()

        .init_resource::<FileHandles>()
        .init_resource::<EditorUiState>()
        .insert_resource(SpriteRes { atlases: HashMap::new() })
        .insert_resource(FighterList (HashMap::new()))

        .add_systems(OnEnter(AppState::Setup), load_files)
        .add_systems(Update, check_files.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), (setup, spriteset_setup, fighters_setup))

        //Backend Systems
        .add_systems(Update, animation_system)

        //Menus
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(Update, (button_system).run_if(in_state(GameState::Menu)))
        .add_systems(Update, (egui_menu_system).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), menu_cleanup)

        //Fighter Editor
        .add_systems(Update, (editor_system).run_if(in_state(GameState::Editor)))

        //Connecting to online
        .add_systems(OnEnter(NetworkState::Connecting), start_matchbox_socket)
        .add_systems(Update, (wait_for_players).run_if(in_state(NetworkState::Connecting)))

        //Gameplay, both offline and online
        .add_systems(OnEnter(GameState::Gameplay), spawn_players)
        .add_systems(FixedUpdate, movable_system)

        //Offline gameplay
        .add_systems(FixedUpdate, (offline_apply_inputs, parse_actions).run_if(in_state(NetworkState::Offline).and_then(in_state(GameState::Gameplay))))

        //Online Gameplay (rollback schedule)
        .add_systems(
            GgrsSchedule,
            (
                apply_inputs,
                parse_actions,
                //increase_frame_count,
                //checksum_players,
            )
                .chain().run_if(in_state(NetworkState::Online)),
        )
        .run();
}

fn load_files(mut file_handles: ResMut<FileHandles>, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    file_handles.handles = asset_server.load_folder("./").unwrap();
}

fn check_files(
    mut next_state: ResMut<NextState<AppState>>,
    file_handles: ResMut<FileHandles>,
    asset_server: Res<AssetServer>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    if let LoadState::Loaded = asset_server
        .get_group_load_state(file_handles.handles.iter().map(|handle| handle.id()))
    {
        next_state.set(AppState::Finished);
    }
}

fn setup(mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 432.0, height: 243.0 };
    commands.spawn(camera_bundle);

    next_state.set(GameState::Menu);
}