extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("safecli")
                          .version("0.1.0")
                          .author("hunterlester <guilfordhunter@gmail.com>")
                          .about("CLI for SAFE Network")
                          .arg(Arg::with_name("login")
                               .short("l")
                               .long("login")
                               .takes_value(true)
                               .multiple(true)
                               .number_of_values(2)
                               .value_names(&["username", "password"])
                               .help("Login to network with name and password"))
                          .arg(Arg::with_name("create_acc")
                               .short("c")
                               .long("create_acc")
                               .takes_value(true)
                               .multiple(true)
                               .number_of_values(3)
                               .value_names(&["username", "password", "invitation"])
                               .help("Create new account with SAFE network with name, password, and invitation key"))
                          .get_matches();

    match matches.values_of("login") {
      Some(value) => {
        let args: Vec<&str> = value.collect();
        println!("{:?}, {:?}", &args[0], &args[1]);
      },
      None => (),
    }

    match matches.values_of("create_acc") {
      Some(value) => {
        let args: Vec<&str> = value.collect();
        println!("{:?}, {:?}, {:?}", &args[0], &args[1], &args[2]);
      },
      None => (),
    }

}
