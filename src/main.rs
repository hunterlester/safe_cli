mod app;
mod authenticator;
mod helpers;

extern crate actix_web;
extern crate clap;
extern crate console;
extern crate futures;
extern crate safe_core;
extern crate serde_json;
extern crate zxcvbn;

use app::initialise;
use authenticator::{create_acc, login};
use clap::{App, Arg, SubCommand};

fn main() {
    let args = App::new("safecli")
      .version("0.1.0")
      .author("hunterlester <guilfordhunter@gmail.com>")
      .about("CLI client for SAFE Network")
      .subcommand(
        SubCommand::with_name("login")
          .arg(Arg::with_name("config_file")
               .takes_value(true)
               .min_values(0)
               .help("Login to network with locator and password")))
      .subcommand(
        SubCommand::with_name("create_acc") 
          .arg(Arg::with_name("config_file")
               .takes_value(true)
               .min_values(0)
               .help("Create new account with SAFE network with locator, password, and invitation key")))
      .subcommand(
        SubCommand::with_name("create_acc") 
          .arg(Arg::with_name("config_file")
               .takes_value(true)
               .min_values(0)
               .help("Create new account with SAFE network with locator, password, and invitation key")))
      .subcommand(
        SubCommand::with_name("initialise") 
          .arg(Arg::with_name("config_file")
               .takes_value(true)
               .min_values(0)
               .help("Initialise new SAFE app instance")))
      .get_matches();

    match args.subcommand() {
        ("login", Some(login_matches)) => match login_matches.value_of("config_file") {
            Some(config_file) => login(Some(config_file)),
            None => login(None),
        },
        ("create_acc", Some(create_acc_matches)) => {
            match create_acc_matches.value_of("config_file") {
                Some(config_file) => create_acc(Some(config_file)),
                None => create_acc(None),
            }
        }
        ("initialise", Some(initialise_matches)) => {
            match initialise_matches.value_of("config_file") {
                Some(config_file) => initialise(Some(config_file)),
                None => initialise(None),
            }
        }
        ("", None) => (),
        _ => (),
    };
}
