mod authenticator;
mod crypto;
mod app;

extern crate safe_authenticator;
extern crate safe_core;
extern crate safe_app;
extern crate zxcvbn;
extern crate tiny_keccak;
extern crate console;

use console::style;
use std::io;
use authenticator::{ create_acc, login };
use crypto::sha3_hash;
use app::{ initialise, authorise, registered };
use safe_authenticator::{ Authenticator, AuthError };
use safe_core::ipc::req::{ AppExchangeInfo };
use safe_core::ipc::resp::AuthGranted;
use safe_app::{ App, AppError };

fn main() {
    let mut auth: Option<Result<Authenticator, AuthError>> = None;
    let mut hashed_data: Option<[u8; 32]> = None;
    let mut app_info: Option<AppExchangeInfo> = None;
    let mut auth_granted: Option<Result<AuthGranted, AuthError>> = None;
    let mut app: Option<Result<App, AppError>>;
    loop {
        println!("{}", style("SAFE CLI (enter command):").blue().bold());
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Please enter valid string");
        match command.trim() {
            "create_acc" => auth = create_acc(),
            "login" => auth = login(),
            "sha3_hash" => {
                hashed_data = sha3_hash();
                println!("SHA3 hash: {:?}", hashed_data);
            },
            "initialise" => {
                app_info = initialise();      
                println!("{}", style("App info created.").green().bold());
            },
            "authorise" => {
                match &app_info {
                    &Some(ref info) => {
                        match &auth {
                            &Some(ref auth_result) => {
                                match auth_result {
                                     &Ok(ref authenticator) => {
                                         auth_granted = authorise(info.clone(), authenticator);
                                         println!("{}", style("Auth granted.").green().bold());
                                         println!("{:?}", auth_granted);
                                     },
                                     &Err(ref err) => println!("Error occurred: {}", err),
                                }
                            },
                            &None => println!("{}", style("Use 'login' command to generate Authenticator").red().bold()),
                        }
                    },
                    &None => println!("{}", style("First use 'initialise' command to generate AppExchangeInfo").red().bold()),
                };
            },
            "registered" => {
                 match &app_info {
                     &Some(ref info) => {
                        match &auth_granted {
                            &Some(ref auth_granted_result) => {
                                match auth_granted_result {
                                     &Ok(ref granted) => {
                                         app = registered(info.clone(), granted.clone());
                                         println!("{}", style("Registered app session connected.").green().bold());
                                     },
                                     &Err(ref err) => println!("Error occurred: {}", err),
                                }
                            },
                            &None => println!("{}", style("Use 'login' command to generate Authenticator").red().bold()),
                        }
                     },
                     &None => println!("{}", style("First use 'initialise' command to generate AppExchangeInfo").red().bold()),
                 } 
            },
            _ => println!("{}", style("Command not yet implemented or recognised").red().bold()),
        }
    }
}


