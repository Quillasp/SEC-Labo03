mod access_control;
/// This file is used to configure and start the TLS server.
/// On new connections, the `handle_client` function is called in a thread
///
/// Tasks todo: - Configure the TLS server properly.
///             - Log stuff whenever required
mod action;
mod connection;
mod database;
mod user;

use crate::access_control::AccessController;
use crate::action::{Action, ConnectedUser};
use crate::user::UserRole;
use connection::Connection;
use lazy_static::lazy_static;
use native_tls::{Identity, Protocol, TlsAcceptor};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

#[macro_use]
extern crate log;

use utils::init_logger;

const SERVER_IP: &str = "localhost:4444";
const KEY_PATH: &str = "keys/sec_lab3_private_pkcs8";
const CERT_PATH: &str = "keys/sec_lab3_cert.pem";

lazy_static! {
    static ref MOTIVATIONAL_QUOTES: Vec<&'static str> = vec![
        "Train people well enough so they can leave. Treat them well enough so they don’t want to.",
        "Human Resources isn’t a thing we do. It’s the thing that runs our business.",
        "When people go to work, they shouldn’t have to leave their hearts at home.",
        "Hire character. Train skill.",
        "Every problem is a gift - without problems we would not grow.",
        "Far and away the best prize that life offers is the chance to work hard at work worth doing.",
        "Believe you can and you’re halfway there."
    ];
}

// Handles client connection by sending a banner and then waiting for a client action
fn handle_client(u: &mut ConnectedUser) -> Result<(), Box<dyn Error>> {
    loop {
        let mut banner = "Welcome to RESIGN (hR onlinE uSer dIrectory manaGemeNt)!".to_string();
        if !u.is_anonymous() {
            banner.push_str(
                format!("\nCurrently logged in as {}", u.user_account()?.username()).as_str(),
            );

            if let UserRole::HR = u.user_account()?.role() {
                let quote =
                    MOTIVATIONAL_QUOTES[rand::thread_rng().gen_range(0..MOTIVATIONAL_QUOTES.len())];
                banner.push_str(format!("\nQuote of the day: {}\n", quote).as_str());
            }
        }

        // We send the banner to  the client and we expect to receive an Action
        u.conn().send(&banner)?;
        let action = u.conn().receive::<Action>()?;
        action.perform(u)?;
    }
}

fn accept(stream: TcpStream, access_control: Arc<AccessController>, acceptor: Arc<TlsAcceptor>) {
    // TLS handshake on top of the connection using the TlsAcceptor
    let stream = acceptor.accept(stream);
    if stream.is_err() {
        error!("TLS handshake failed with error: {}", stream.err().unwrap());
    } else {
        info!("TLS client connection accepted");
        let mut u = ConnectedUser::anonymous(access_control, Connection::new(stream.unwrap()));
        match handle_client(&mut u) {
            Ok(_) => info!("Client connection closed"),
            Err(e) => error!("Error while handling client connection: {}", e),
        }
    }
}

// Load the server certificate and private key from PKCS8 format
fn load_server_identity(cert_file: &str, key_file: &str) -> Identity {
    let mut cert = Vec::new();
    let mut key = Vec::new();

    let mut cert_file = File::open(cert_file).expect("Certificate file not found");
    let mut key_file = File::open(key_file).expect("Key file not found");

    cert_file.read_to_end(&mut cert).unwrap();
    key_file.read_to_end(&mut key).unwrap();

    Identity::from_pkcs8(&cert, &key).unwrap()
}

// Create a new TLS configuration
fn tls_config(cert_file: &str, key_file: &str) -> Arc<TlsAcceptor> {
    let identity = load_server_identity(cert_file, key_file);

    let acceptor = TlsAcceptor::builder(identity)
        .min_protocol_version(Some(Protocol::Tlsv12))
        .max_protocol_version(Some(Protocol::Tlsv12))
        .build()
        .expect("Could not build TlsAcceptor");

    Arc::new(acceptor)
}

#[tokio::main]
async fn main() {
    init_logger();
    trace!("Main server");
    // Start TLS server and wait for new connections
    let acceptor = tls_config(CERT_PATH, KEY_PATH);
    let listener = TcpListener::bind(SERVER_IP).unwrap();
    let access_control = Arc::new(AccessController::new().await.unwrap());
    info!("Server started");

    // Handles new connection, negotiate TLS and call handle_client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                let access_control = access_control.clone();
                thread::spawn(move || {
                    accept(stream, access_control, acceptor);
                });
            }
            Err(e) => {
                error!("Connection failed with error: {}", e);
            }
        }
    }
}
