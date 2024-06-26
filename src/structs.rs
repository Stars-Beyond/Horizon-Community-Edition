use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use socketioxide::extract::SocketRef;
use tokio::net::UdpSocket;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    origin: (f64, f64, f64),
    data: String,
    propagation_distance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

pub struct ChildServer {
    id: u64,
    coordinate: Coordinate,
    parent_addr: SocketAddr,
    socket: UdpSocket,
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// * The `ChildServer` struct contains methods for:                                             //
// - Initializing the server with its ID, coordinate, parent server address, and local address. //
// - Receiving events from the master server.                                                   //
// - Determining which neighboring servers should receive an event.                             //
// - Sending events to the parent server for further multicast.                                 //
// - Running the server to continuously listen for and handle events.                           //
//////////////////////////////////////////////////////////////////////////////////////////////////

impl ChildServer {
    pub async fn new(
        id: u64,
        coordinate: Coordinate,
        parent_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> Self {
        let socket = UdpSocket::bind(local_addr).await.expect("Couldn't bind to address");
        ChildServer {
            id,
            coordinate,
            parent_addr,
            socket,
        }
    }
///////////////////////////////////////////////////////////////////////////////////////////////////////
// * Event Handling:                                                                                 //
// - The child server receives events from the master server. Each event contains its origin,        //
//   data, and a propagation distance, which determines how far the event should spread.             //
///////////////////////////////////////////////////////////////////////////////////////////////////////

    pub async fn receive_event(&self) -> Event {
        let mut buf = [0u8; 1024];
        let (amt, _src) = self
            .socket
            .recv_from(&mut buf)
            .await
            .expect("Didn't receive data");
        let event: Event = bincode::deserialize(&buf[..amt]).expect("Failed to deserialize event");
        event
    }

////////////////////////////////////////////////////////////////////////////////////////////////////////
// * Event Propagation:                                                                               //
// - Upon receiving an event, the child server calculates which neighboring child servers             //
//    should receive the event based on the event's origin and the specified propagation distance.    //
// - This calculation considers all adjacent coordinates within a 3x3x3 cube centered on the          //
//    server's coordinate, ensuring that all relevant neighbors are included.                         //
////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn determine_propagation(&self, event: &Event) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        let max_distance = event.propagation_distance;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let neighbor = Coordinate {
                        x: self.coordinate.x + x,
                        y: self.coordinate.y + y,
                        z: self.coordinate.z + z,
                    };

                    let distance = ((neighbor.x as f64 - event.origin.0).powi(2)
                        + (neighbor.y as f64 - event.origin.1).powi(2)
                        + (neighbor.z as f64 - event.origin.2).powi(2))
                    .sqrt();

                    if distance <= max_distance {
                        neighbors.push(neighbor);
                    }
                }
            }
        }
        neighbors
    }

//////////////////////////////////////////////////////////////////////////////////////////////////////
// * Event Transmission:                                                                            //
// - After determining the target neighbors, the child server sends the event to the master server. //
//   The master server then multicasts the event to the appropriate neighboring child servers.      //
//////////////////////////////////////////////////////////////////////////////////////////////////////

    pub async fn send_event(&self, event: &Event, target: &Coordinate) {
        let msg = bincode::serialize(event).expect("Failed to serialize event");
        let addr = self.calculate_addr(target);
        self.socket
            .send_to(&msg, addr)
            .await
            .expect("Failed to send event");
    }

    pub fn calculate_addr(&self, target: &Coordinate) -> SocketAddr {
        // Implement your logic to calculate the socket address of the target coordinate
        // This is a placeholder, you need to provide the actual mapping from coordinate to address
        SocketAddr::new("127.0.0.1".parse().unwrap(), 8080)
    }

    pub fn handle_event(&self, event: Event) {
        let neighbors = self.determine_propagation(&event);

        for neighbor in neighbors {
            self.send_event(&event, &neighbor);
        }
    }

    pub async fn run(&self) {
        loop {
            let event = self.receive_event().await;
            self.handle_event(event);
        }
    }
}

// Define a struct for Player
#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub socket: SocketRef,
    pub location: Option<Location>, // Optional to handle players who haven't sent location updates yet
}

////////////////////////////////////////////////////
//            World object structs:               //
// These Structs help store an object's location  //
// this server's coordanites in the instance grid //
// Define a struct for Rotation of objects        //
////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rotation {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Scale of objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Translation of objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    x: f64,
    y: f64,
    z: f64,
}

// Define a struct for Location of objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    rotation: Rotation,
    scale3D: Scale, // Update field name to match the JSON data
    translation: Translation,
}
