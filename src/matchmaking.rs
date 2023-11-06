use crate::game::*;
use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_ggrs::ggrs::{Config, PlayerHandle, self};
use bevy_ggrs::PlayerInputs;


#[derive(Resource)]
pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
}

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = u16;
    type State = u8;
    type Address = PeerId;
}

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://162.195.243.83:25565/polyduel?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
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

pub fn network_input(
    _: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>
) -> u16 {
    return input(keyboard_input);
}

fn network_move_players(inputs: Res<PlayerInputs<GGRSConfig>>, mut players: Query<(&mut Transform, &Player)>) {
    move_players(inputs, players);
}