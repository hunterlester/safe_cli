//extern crate safe_core;
//extern crate safe_app;
//
//use safe_core::ipc::req::{ AppExchangeInfo };
//use safe_core::ipc::resp::AuthGranted;
//use safe_app::{ App, AppError };
//
use std::env;

fn main() {
    println!("app bin");

    for argument in env::args() {
        println!("{}", argument);
    }
}
