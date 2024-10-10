use horizon_data_types::Player;
use socketioxide::extract::SocketRef;
use std::sync::{Arc, Mutex};

//  pub mod auth; // Auth is not yet being used as it needs to be a more flexible
//  pub mod actors;


//  pub mod leaderboard;
//  pub mod player_data;
//  pub mod game_logic;
//  pub mod notifications;
//  pub mod logging;
//  pub mod level_data;
//  pub mod startup;



pub fn init_all(socket: SocketRef, players: Vec<Player>) {
    println!("Starting core modules...");

    // This is where the submodules are started on player join
    crate::core::chat::main::init(socket, players);
    // leaderboard::init(player);
    // player_data::init(player);
    // game_logic::init(player);
    // notifications::init(player);
    // logging::init(player);
    // level_data::init(player);
    // startup::init(player);
}