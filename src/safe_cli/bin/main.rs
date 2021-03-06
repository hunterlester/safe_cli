mod authenticator;
mod helpers;

use crate::authenticator::{authorise, create_acc, login, web_socket};
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
        SubCommand::with_name("authorise") 
          .arg(Arg::with_name("config_file")
               .takes_value(true)
               .min_values(0)
               .help("Returns encoded authorisation response to be used to connect to SAFE network")))
      .subcommand(
        SubCommand::with_name("ws") 
          .arg(Arg::with_name("config_file")
               .takes_value(true)
               .min_values(0)
               .help("Starts web socket connection")))
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
        ("authorise", Some(authorise_matches)) => match authorise_matches.value_of("config_file") {
            Some(config_file) => authorise(Some(config_file)),
            None => authorise(None),
        },
        ("ws", Some(ws_matches)) => match ws_matches.value_of("config_file") {
            Some(config_file) => web_socket(Some(config_file)),
            None => web_socket(None),
        },
        ("", None) => (),
        _ => (),
    };
}
