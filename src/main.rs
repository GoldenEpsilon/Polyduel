mod game;
mod matchmaking;

use crate::game::*;
use crate::matchmaking::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::texture::ImageSampler;
use bevy_ggrs::{GgrsPlugin, GgrsSchedule, AddRollbackCommandExtension, GgrsAppExtension};

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
        .add_systems(Startup, (setup, start_matchbox_socket))
        .add_systems(Update, (spritemap_fix, wait_for_players))
        // rollback schedule
        .add_systems(
            GgrsSchedule,
            (
                //apply_inputs,
                move_players,
                //increase_frame_count,
                //checksum_players,
                global_update
            )
                .chain(),
        )
        //.add_systems(Update, button_system)
        //.add_systems(FixedUpdate, global_update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 432.0, height: 243.0 };
    commands.spawn(camera_bundle);
    commands.spawn((
        Player{ handle: 0 },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            texture: asset_server.load("IdIdle.png"),
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

fn global_update(mut interaction_query: Query<(&mut Transform, &mut Handle<Image>)>){
    for (_, _) in &mut interaction_query {
    }
}