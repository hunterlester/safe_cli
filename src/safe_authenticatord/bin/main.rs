extern crate safe_authenticator;
extern crate console;
extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use safe_authenticator::{ Authenticator, AuthError };
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::env;
use console::style;

fn process_request(args: Vec<String>) -> Option<Authenticator> {
    let request_type = args[1].clone();
    match request_type.as_ref() {
     "create_acc" => {
        let locator = args[2].clone();
        let password = args[3].clone();
        let invite = args[4].clone();
        match Authenticator::create_acc(locator, password, invite, || println!("{}", style("Disconnected from network").red().bold())) {
          Ok(auth) =>  {
              println!("{}", style("Logged in to SAFE network.").green().bold());
              Some(auth)
          },
          Err(auth_error) => {
              println!("{}: {}", style("Failed to create account").red().bold(), style(&auth_error).red().bold());
              None
          },
        }
     },
     "login" => {
        let locator = args[2].clone();
        let password = args[3].clone();
        match Authenticator::login(locator, password, || println!("{}", style("Disconnected from network").red().bold())) {
          Ok(auth) =>  {
              println!("{}", style("Logged in to SAFE network.").green().bold());
              Some(auth)
          },
          Err(auth_error) => {
              println!("{}: {}", style("Login failed").red().bold(), style(&auth_error).red().bold());
              None
          },
        }
      },
      _ => {
          println!("Unrecognised operation: {}", request_type);
          None
      },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut auth_instance: Option<Authenticator> = None;
    auth_instance = process_request(args);
    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 41805);
    let listener = TcpListener::bind(&socket_address).unwrap(); 

    let server = listener.incoming().for_each(|socket| {
        println!("socket: {:?}", socket);
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        let connection = io::write_all(socket, "Connected\n")
            .then(|res| {
                println!("wrote message: {:?}", res);
                Ok(())
            });

        tokio::spawn(connection);

        Ok(())
    }).map_err(|err| {
        println!("accept error = {:?}", err);    
    });

    println!("server running on localhost:41805");
    tokio::run(server);
}
