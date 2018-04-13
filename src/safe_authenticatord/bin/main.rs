extern crate safe_authenticator;
extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use safe_authenticator::{ Authenticator, AuthError };
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::env;

fn main() {
    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 41805);
    let listener = TcpListener::bind(&socket_address).unwrap(); 

    let server = listener.incoming().for_each(|socket| {
        println!("socket: {:?}", socket);
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());
        let buf: Vec<u8> = Vec::new();

        let connection = io::read_to_end(socket, buf)
            .and_then(|(socket, buf)| {
                let resp = String::from("Connected");
                io::write_all(socket, resp.into_bytes())
            })
            .then(|_| Ok(()));
        //https://tokio.rs/docs/getting-started/io/
        tokio::spawn(connection);
        Ok(())
    }).map_err(|err| {
        println!("accept error = {:?}", err);    
    });

    println!("server running on localhost:41805");
    tokio::run(server);

    // let args: Vec<String> = env::args().collect();
    // let request_type = args[1].clone();
    // let mut auth_instance: Option<Authenticator> = None;

    // // TODO: Implement single instance management, file descriptor polling, daemonise with loop
    // // using mio

    // match request_type.as_ref() {
    //  "create_acc" => {
    //     let locator = args[2].clone();
    //     let password = args[3].clone();
    //     let invite = args[4].clone();
    //     match Authenticator::create_acc(locator, password, invite, || println!("{}", style("Disconnected from network").red().bold())) {
    //       Ok(auth) =>  {
    //           println!("{}", style("Logged in to SAFE network.").green().bold());
    //           auth_instance = Some(auth)
    //       },
    //       Err(auth_error) => {
    //           println!("{}: {}", style("Failed to create account").red().bold(), style(&auth_error).red().bold());
    //           auth_instance = None;
    //       },
    //     }
    //  },
    //  "login" => {
    //     let locator = args[2].clone();
    //     let password = args[3].clone();
    //     match Authenticator::login(locator, password, || println!("{}", style("Disconnected from network").red().bold())) {
    //       Ok(auth) =>  {
    //           println!("{}", style("Logged in to SAFE network.").green().bold());
    //           auth_instance = Some(auth)
    //       },
    //       Err(auth_error) => {
    //           println!("{}: {}", style("Login failed").red().bold(), style(&auth_error).red().bold());
    //           auth_instance = None;
    //       },
    //     }
    //   },
    //   _ => println!("Unrecognised operation: {}", request_type),
    // }
}
