use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::texture::{ImageSampler};
use bevy_ggrs::ggrs::{Config, PlayerHandle, self};
use bevy_ggrs::{GgrsPlugin, GgrsSchedule, AddRollbackCommandExtension, PlayerInputs, GgrsAppExtension};
use bevy_matchbox::prelude::*;

const FPS: usize = 60;

const INPUT_UP: u16 = 1 << 0;
const INPUT_DOWN: u16 = 1 << 1;
const INPUT_LEFT: u16 = 1 << 2;
const INPUT_RIGHT: u16 = 1 << 3;
const INPUT_L: u16 = 1 << 4;
const INPUT_M: u16 = 1 << 5;
const INPUT_H: u16 = 1 << 6;
const INPUT_S: u16 = 1 << 7;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Gameplay,
}

#[derive(Resource)]
pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
}

pub fn input(
    _: In<PlayerHandle>,
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

#[derive(Component)]
struct Player {
    handle: usize
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteSheetBundle,
}

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = u16;
    type State = u8;
    type Address = PeerId;
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
            .with_input_system(input)
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

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://162.195.243.83:25565/polyduel?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");
    
    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
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

fn move_players(inputs: Res<PlayerInputs<GGRSConfig>>, mut players: Query<(&mut Transform, &Player)>) {
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