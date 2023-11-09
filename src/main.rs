mod game;
mod netcode;

use crate::game::*;
use crate::netcode::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::texture::ImageSampler;
use bevy_ggrs::{GgrsPlugin, GgrsSchedule, GgrsAppExtension};

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
        }))
        .add_ggrs_plugin(
            GgrsPlugin::<GGRSConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(network_input)
            .register_rollback_component::<Transform>()
            //.register_rollback_component::<Checksum>()
            //.register_rollback_resource::<FrameCount>()
        )
        .add_systems(Startup, setup)
        .add_systems(Update, spritemap_fix)
        .add_systems(OnEnter(GameState::Gameplay), spawn_players)
        .add_systems(Update, (offline_update).run_if(in_state(NetworkState::Offline).and_then(in_state(GameState::Gameplay))))
        .add_systems(OnEnter(NetworkState::Connecting), start_matchbox_socket)
        .add_systems(Update, (wait_for_players).run_if(in_state(NetworkState::Connecting)))

        // rollback schedule
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 432.0, height: 243.0 };
    commands.spawn(camera_bundle);
}

//thank you https://stackoverflow.com/questions/76292957/what-is-the-correct-way-to-implement-nearestneighbor-for-textureatlas-sprites-in
fn spritemap_fix(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if let Some(texture) = assets.get_mut(&handle) {
                    texture.sampler_descriptor = ImageSampler::nearest()
                }
            },
            _ => {}
        }
    }
}

fn offline_update(keyboard_input: Res<Input<KeyCode>>, players: Query<(&mut Transform, &Player)>){
    let mut inputs: Vec<u16> = vec![0; players.iter().len()];
    inputs[0] = input(keyboard_input);//TEMP, is currently only going to P1 slot
    move_players(inputs, players);
}