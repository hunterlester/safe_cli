mod authenticator;
//mod crypto;
//mod app;
mod helpers;

extern crate actix_web;
extern crate clap;
extern crate console;
extern crate tiny_keccak;
extern crate zxcvbn;

//use console::style;
use authenticator::{create_acc, login};
//use crypto::sha3_hash;
//use app::{ initialise, authorise, registered };
use clap::{App, Arg, SubCommand};

fn main() {
    // Goal to serve both human users and program consumption.
    // For example, for login, if no extra arguments are passed \
    // user is prompted to create secure credentials. Otherwise \
    // a program consuming the binary can pass a path to JSON or\
    // YAML file with encrypted credentials to complete login
    let args = App::new("safecli")
      .version("0.1.0")
      .author("hunterlester <guilfordhunter@gmail.com>")
      .about("CLI for SAFE Network")
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
        SubCommand::with_name("sha3_hash") 
          .arg(Arg::with_name("sha3_hash")
               .takes_value(true)
               .number_of_values(1)
               .value_names(&["string"])
               .help("FIPS-202-defined SHA-3 Hash using 32 bit words (SHA3-256)")))
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
        ("", None) => (),
        _ => (),
    }
}
