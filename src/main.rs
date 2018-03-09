mod authenticator;
mod crypto;
mod app;
mod helpers;

extern crate safe_authenticator;
extern crate safe_core;
extern crate safe_app;
extern crate zxcvbn;
extern crate tiny_keccak;
extern crate console;

use console::style;
use authenticator::{ create_acc, login };
use crypto::sha3_hash;
use app::{ initialise, authorise, registered };
use safe_authenticator::{ Authenticator, AuthError };
use safe_core::ipc::req::{ AppExchangeInfo };
use safe_core::ipc::resp::AuthGranted;
use safe_app::{ App, AppError };
use helpers::{ read_line };

fn main() {
    let mut auth: Option<Result<Authenticator, AuthError>> = None;
    let mut hashed_data: Option<[u8; 32]> = None;
    let mut app_info: Option<AppExchangeInfo> = None;
    let mut auth_granted: Option<Result<AuthGranted, AuthError>> = None;
    let mut app: Option<Result<App, AppError>>;
    println!("{}", style("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~").blue().bold());
    println!("{}", style("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~").blue().bold());
    println!("{}", style("~~~~~~~~~~~~~~~Welcome to SAFE CLI~~~~~~~~~~~~~~~~~~~").blue().bold());
    println!("{}", style("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~").blue().bold());
    println!("{}", style("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~").blue().bold());
    loop {
        println!("{}", style("SAFE CLI (enter command):").yellow().bold());
        let mut command = String::new();
        command = read_line(&mut command);

        match command.as_str() {
            "create_acc" => auth = create_acc(),
            "login" => auth = login(),
            "sha3_hash" => {
                hashed_data = sha3_hash();
                println!("{} {:?}", style("SHA3 hash:").green(), style(hashed_data).cyan());
            },
            "initialise" => {
                app_info = initialise();      
                println!("{}", style("App info created.").green());
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
                                         println!("{:?}", style(&auth_granted).cyan());
                                     },
                                     &Err(ref err) => println!("{} {}", style("Error occurred:").red().bold(), style(err).red()),
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
                                     &Err(ref err) => println!("{} {}", style("Error occurred:").red().bold(), style(err).red()),
                                }
                            },
                            &None => println!("{}", style("Use 'login' command to generate Authenticator").red().bold()),
                        }
                     },
                     &None => println!("{}", style("First use 'initialise' command to generate AppExchangeInfo").red().bold()),
                 } 
            },
            "commands" => {
                println!("{}", style("create_acc").magenta().bold());    
                println!("{}", style("login").magenta().bold());    
                println!("{}", style("initialise").magenta().bold());    
                println!("{}", style("authorise").magenta().bold());    
                println!("{}", style("registered").magenta().bold());    
                println!("{}", style("sha3_hash").magenta().bold());    
            },
            _ => println!("{}", style("Command not yet implemented or recognised").red().bold()),
        }
    }
}


