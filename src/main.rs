mod backend;
mod game;
mod netcode;
mod menu;

use crate::game::*;
use crate::netcode::*;
use crate::menu::*;
use backend::animation_system;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::{GgrsAppExtension, GgrsPlugin, GgrsSchedule};

const FPS: usize = 60;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Gameplay,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteSheetBundle,
}

fn main() {
    let mut app = App::new();

    app
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
        .add_ggrs_plugin(
            GgrsPlugin::<GGRSConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(network_input)
            .register_rollback_component::<Transform>()
            //.register_rollback_component::<Checksum>()
            //.register_rollback_resource::<FrameCount>()
        )
        .add_systems(Startup, setup)

        //Backend Systems
        .add_systems(Update, animation_system)

        //Menus
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(Update, (button_system).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), menu_cleanup)

        //Connecting to online
        .add_systems(OnEnter(NetworkState::Connecting), start_matchbox_socket)
        .add_systems(Update, (wait_for_players).run_if(in_state(NetworkState::Connecting)))

        //Gameplay, both offline and online
        .add_systems(OnEnter(GameState::Gameplay), spawn_players)

        //Offline gameplay
        .add_systems(FixedUpdate, (offline_update).run_if(in_state(NetworkState::Offline).and_then(in_state(GameState::Gameplay))))

        //Online Gameplay (rollback schedule)
        .add_systems(
            GgrsSchedule,
            (
                //apply_inputs,
                network_move_players,
                //increase_frame_count,
                //checksum_players,
            )
                .chain().run_if(in_state(NetworkState::Online)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 432.0, height: 243.0 };
    commands.spawn(camera_bundle);
}

fn offline_update(keyboard_input: Res<Input<KeyCode>>, players: Query<(&mut Transform, &Player)>){
    let mut inputs: Vec<u16> = vec![0; players.iter().len()];
    inputs[0] = input(keyboard_input);//TEMP, is currently only going to P1 slot
    move_players(inputs, players);
}