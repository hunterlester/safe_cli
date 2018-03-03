mod authenticator;
mod crypto;

extern crate safe_authenticator;
extern crate zxcvbn;
extern crate tiny_keccak;
extern crate console;

use console::style;
use std::io;
use authenticator::{ create_acc, login };
use crypto::sha3_hash;

fn main() {
    let mut auth;
    let mut hashed_data;
    loop {
        println!("SAFE CLI (enter command):");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Please enter valid string");
        match command.trim() {
            "create_acc" => auth = create_acc().unwrap(),
            "login" => auth = login().unwrap(),
            "sha3_hash" => {
                hashed_data = sha3_hash();
                println!("SHA3 hash: {:?}", hashed_data);
            },
            _ => println!("{}", style("Command not yet implemented or recognised").red().bold()),
        }
    }
}


