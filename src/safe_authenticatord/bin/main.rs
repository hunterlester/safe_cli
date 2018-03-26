extern crate safe_authenticator;
extern crate console;
use safe_authenticator::{ Authenticator, AuthError };
use console::style;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let request_type = args[1].clone();
    let mut auth_instance: Option<Authenticator> = None;

    // TODO: Implement single instance management, file descriptor polling, daemonise with loop
    // using mio

    match request_type.as_ref() {
     "create_acc" => {
        let locator = args[2].clone();
        let password = args[3].clone();
        let invite = args[4].clone();
        match Authenticator::create_acc(locator, password, invite, || println!("{}", style("Disconnected from network").red().bold())) {
          Ok(auth) =>  {
              println!("{}", style("Logged in to SAFE network.").green().bold());
              auth_instance = Some(auth)
          },
          Err(auth_error) => {
              println!("{}: {}", style("Failed to create account").red().bold(), style(&auth_error).red().bold());
              auth_instance = None;
          },
        }
     },
     "login" => {
        let locator = args[2].clone();
        let password = args[3].clone();
        match Authenticator::login(locator, password, || println!("{}", style("Disconnected from network").red().bold())) {
          Ok(auth) =>  {
              println!("{}", style("Logged in to SAFE network.").green().bold());
              auth_instance = Some(auth)
          },
          Err(auth_error) => {
              println!("{}: {}", style("Login failed").red().bold(), style(&auth_error).red().bold());
              auth_instance = None;
          },
        }
      },
      _ => println!("Unrecognised operation: {}", request_type),
    }
}
