mod station;

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::exit;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Mutex, RwLock};
use colored::Colorize;
use crate::{Command, Datapoint};
use crate::Datapoint::Status;

/// The connection manager of the application. This module is responsible for
/// managing the connection to the main PCB.
///
/// It offers a few interface functions:
/// - `launch` to start the connection
/// - `stop` to gracefully stop the connection
/// - `send(command)` to send a command to the main PCB
/// - `get_state()` to get the main PCB's FSM state
/// - ...
include!(concat!(env!("OUT_DIR"), "/config.rs"));

// pub enum Event {
//     ConnectionClosed,
//
// }

#[derive(Debug)]
pub struct Station {
    pub running : bool,
    pub connected: bool,
    pub gs_socket : SocketAddr,
    pub listener : TcpListener,
    pub tx: Sender<Datapoint>,
    pub rx: Mutex<Receiver<Command>>,
    // pub event_tx: Sender<Event>,
    // pub event_rx: Receiver<Event>
}

/// The main function of the connection handler,
/// it will open a socket and wait for the pod to connect,
/// then create a connection object and handle it
pub fn launch(tx : Sender<Datapoint>, rx : Receiver<Command>) {
    let server = TcpListener::bind(GS_SOCKET());

    if server.is_err() {
        println!("{}\n  IP={}\n  timestamp={}\n", "Server failed to start".on_bright_red(), GS_SOCKET().ip().to_string(),chrono::Utc::now().timestamp().to_string());
        println!("{}", "Try changing ../config/netconfig.toml".bright_yellow());
        exit(1);
    }

    let mut station: Station = Station::new(GS_SOCKET(), server.unwrap(), tx, rx);

    while station.running {
        match station.listener.accept() {
            Ok((stream, addr)) => {
                station.connected = true;
                station.run(stream, addr); // this will be run from the server main thread since a) we dont want it to die and b) we will only ever accept one connection: the main pcb
            },
            Err(e) => eprintln!("couldn't get client: {e:?}"),
        }

        if station.running {// if we get here the station disconnected from the main pcb.
            println!("Connection terminated");
            station.tx.send(Status(String::from("Connection was terminated! Trying to reconnect automatically."))).expect("Failed to send message to GUI!");
        }
    }
    println!("Shutting down ground station");
}

pub fn receive(buffer: [u8;IP_BUFFER_SIZE], n: usize, tx_channel : Sender<Datapoint> ) {
    // Process the received data.
    let data = String::from_utf8(Vec::from(&buffer[..n])).expect("failed to convert buffer to string");
    println!("received {}", data.trim().on_bright_blue());
}