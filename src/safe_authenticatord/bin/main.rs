extern crate safe_authenticator;
extern crate console;
extern crate tokio;

use std::io::BufReader;
use tokio::io::{ lines, write_all };
use tokio::net::TcpListener;
use tokio::prelude::*;
use safe_authenticator::{ Authenticator, AuthError };
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::env;
use console::style;
use std::sync::{Arc, Mutex};

fn process_request(args: Vec<String>) -> Result<Authenticator, AuthError> {
    let request_type = args[1].clone();
    match request_type.as_ref() {
     "create_acc" => {
        let locator = args[2].clone();
        let password = args[3].clone();
        let invite = args[4].clone();
        match Authenticator::create_acc(locator, password, invite, || println!("{}", style("Disconnected from network").red().bold())) {
          Ok(auth) =>  {
              println!("{}", style("Logged in to SAFE network.").green().bold());
              Ok(auth)
          },
          Err(auth_error) => {
              println!("{}: {}", style("Failed to create account").red().bold(), style(&auth_error).red().bold());
              Err(auth_error)
          },
        }
     },
     "login" => {
        let locator = args[2].clone();
        let password = args[3].clone();
        match Authenticator::login(locator, password, || println!("{}", style("Disconnected from network").red().bold())) {
          Ok(auth) => {
              println!("{}", style("Logged in to SAFE network.").green().bold());
              Ok(auth)
          },
          Err(auth_error) => {
              println!("{}: {}", style("Login failed").red().bold(), style(&auth_error).red().bold());
              Err(auth_error)
          },
        }
      },
      _ => {
          println!("Unrecognised operation: {}", request_type);
          Err(AuthError::from(format!("Unrecognised operation: {}", request_type)))
      },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let auth_instance: Arc<Mutex<Result<Authenticator, AuthError>>> = Arc::new(Mutex::new(process_request(args)));
    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 41805);
    let listener = TcpListener::bind(&socket_address).unwrap(); 

    let server = listener.incoming().for_each(move |socket| {
        let (reader, writer) = socket.split();
        let lines = lines(BufReader::new(reader));
        let cloned_auth_instance = auth_instance.clone();
        let responses = lines.map(move |line| {
            let request: Vec<String> = line.split_whitespace().map(|v| v.to_string()).collect();
            let mut auth_instance_mutex = cloned_auth_instance.lock().unwrap();
            *auth_instance_mutex = process_request(request);
            match *auth_instance_mutex {
                Ok(ref _auth) => String::from("Logged in to SAFE network via IPC.").clone(),
                Err(ref auth_err) => format!("{}", &auth_err).clone(),
            }
        });

        let writes = responses.fold(writer, |writer, mut response| {
            response.push('\n');
            write_all(writer, response.into_bytes()).map(|(w, _)| w)
        });

        let msg = writes.then(move |_| Ok(()));

        tokio::spawn(msg);
        Ok(())
    }).map_err(|err| {
        println!("accept error = {:?}", err);    
    });

    println!("server running on localhost:41805");
    tokio::run(server);
}
