////////////////////////////////////////////////////////////////////////////////////////////////////
//                                      Horizon Game Server                                       //
//                                                                                                //
// This server software is part of a distributed system designed to facilitate communication      //
// and data transfer between multiple child servers and a master server. Each child server        //
// operates within a "Region map" managed by the master server, which keeps track of their        //
// coordinates in a relative cubic light-year space. The coordinates are stored in 64-bit floats  //
// to avoid coordinate overflow and to ensure high precision.                                     //
//                                                                                                //
////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////
// Import a few things to get us started //
///////////////////////////////////////////

use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, Mutex};
use tokio::main;
use tracing::info;
use viz::{serve, Result, Router};
use PebbleVault;
use TerraForge;

// WARNING
// Import all structs (when we have a ton of structs this will be very bad but should be fine for now)

use structs::*;

/////////////////////////////////////
// Import the modules we will need //
/////////////////////////////////////

mod events;
mod macros;
mod subsystems;
mod structs;

///////////////////////////////////////////////////////////////
//                         WARNING                           //
// on_connect runs every time a new player connects to the   //
// server avoid putting memory hungry code here if possible! //
///////////////////////////////////////////////////////////////

fn on_connect(socket: SocketRef, Data(data): Data<Value>, players: Arc<Mutex<Vec<Player>>>) {
    // Authenticate the user
    let player = Player {
        id: socket.id.to_string(),
        socket: socket.clone(),
        location: None, // Initialize with no location
    };
    
    players.lock().unwrap().push(player);
    
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("connected", true).ok();
    socket.emit("auth", data).ok();
        
    /////////////////////////////////////////////////////////
    // Setup external event listeners for the more complex //
    // systems                                             //
    /////////////////////////////////////////////////////////
    
    //subsystems::actors::main::main::main;
    subsystems::chat::init(socket.clone());
    subsystems::game_logic::init();
    subsystems::leaderboard::init();
    subsystems::level_data::init();
    subsystems::logging::init();
    subsystems::notifications::init();
    subsystems::player_data::init(socket.clone());
    
    ////////////////////////////////////////////////////////
    // Register some additional custom events with our    // 
    // socket server. Your custom events will be          //
    // registered here as well as in the ./events/mod.rs  //
    // file                                               //
    ////////////////////////////////////////////////////////
    
    define_event!(socket, "test", events::test::main());
    
}
#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //////////////////////////////////
    // Show branding during startup //
    //////////////////////////////////
    subsystems::startup::main();

    TerraForge::main();
    
    println!("{}", PebbleVault::greet("Rust"));
    let db = PebbleVault::create_db();
    PebbleVault::create_spatial_index(db, "SpaceBody", "1");
    PebbleVault::create_galaxy(db, "Galaxy", "Artermis");
    PebbleVault::create_galaxy(db, "Galaxy", "Athena");
    PebbleVault::create_galaxy(db, "Galaxy", "Hades");
    PebbleVault::get_k_nearest_galaxies(db, "Artermis");

    let app = Router::new()
        .get("/", |_| async { Ok("Welcome to Horizon Server V: 0.3.0-318974-C") });

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    if let Err(e) = serve(listener, app).await {
        println!("{}", e);
    }

    Ok(())
}