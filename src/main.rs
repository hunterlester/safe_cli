extern crate clap;
extern crate safe_authenticator;
extern crate zxcvbn;
extern crate tiny_keccak;

mod crypto;

use clap::{ Arg, App };
use safe_authenticator::{ Authenticator };
use zxcvbn::zxcvbn;
use std::io;
use crypto::sha3_hash;

fn connected(auth: Authenticator) -> () {
  println!("Logged in to SAFE network.")
}

fn validate_cred(cred: &'static str) -> String {
    println!("Please choose a {}:", &cred);
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).expect("Please enter valid string");
    secret = secret.trim().to_string();
    let secret_strength = zxcvbn(&mut secret, &[]).unwrap();
    println!("\nInteresting information about your {}:\n
      Estimated number of guesses needed to crack: {:?}\n
      Estimated number of seconds needed with fastest cracking method: {:?}\n
      In short, it would take with quickest cracking method: {:?}\n
      On a scale of 0-4, your score is: {:?}\n", &cred, &secret_strength.guesses, &secret_strength.crack_times_seconds.offline_fast_hashing_1e10_per_second, &secret_strength.crack_times_display.offline_fast_hashing_1e10_per_second, &secret_strength.score);

    if secret_strength.score <= 2 {
      println!("\nYour {} is not strong enough.\n Here is how to make it stronger: \n{:?}", &cred, &secret_strength.feedback.unwrap());
      validate_cred(cred)
    } else {
      println!("Please type in your {} again to confirm:", &cred);
      let mut secret_compare = String::new();
      io::stdin().read_line(&mut secret_compare).expect("Please enter valid string");
      secret_compare = secret_compare.trim().to_string();
      if &mut secret == &mut secret_compare {
        secret
      } else {
        println!("{} does not match. Starting over...", &cred);
        validate_cred(cred)
      }
    }
}

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
                               .number_of_values(1)
                               .value_names(&["invitation"])
                               .help("Create new account with SAFE network with name, password, and invitation key"))
                          .arg(Arg::with_name("sha3_hash")
                               .short("s")
                               .long("sha3_hash")
                               .takes_value(true)
                               .number_of_values(1)
                               .value_names(&["string"])
                               .help("FIPS-202-defined SHA-3 Hash using 32 bit words (SHA3-256)"))
                          .get_matches();

    match matches.values_of("login") {
      Some(value) => {
        let args: Vec<&str> = value.collect();
        match Authenticator::login(args[0], args[1], || println!("Disconnected from network")) {
          Ok(auth) =>  connected(auth),
          Err(auth_error) => println!("Login failed: {:?}", &auth_error),
        }
      },
      None => (),
    }

    match matches.value_of("create_acc") {
      Some(invite) => {
          let secret = validate_cred("secret");
          println!("\u{2705}Valid secret");
          let password = validate_cred("password");
          println!("\u{2705}Valid password");
          match Authenticator::create_acc(secret, password, String::from(invite), || println!("Disconnected from network")) {
            Ok(auth) =>  connected(auth),
            Err(auth_error) => println!("Failed to create account: {:?}", &auth_error),
          }
      },
      None => (),
    }

    match matches.value_of("sha3_hash") {
      Some(data) => {
        let hash = sha3_hash(String::from(data));
        println!("sha3 256 hash: {:?}", hash);
      },
      None => (),
    }
}


