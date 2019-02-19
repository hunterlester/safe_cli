extern crate actix_web;
extern crate console;
extern crate safe_authenticator;

use actix_web::{http::Method, server, App, HttpRequest, Path};
use console::style;
use safe_authenticator::{AuthError, Authenticator};
use std::sync::{Arc, Mutex};

fn create_acc(info: Path<(String, String, String)>, req: HttpRequest) -> &'static str {
    let mut handle = *(req.state().handle.lock().unwrap());
    match Authenticator::create_acc(info.0, info.1, info.2, || {
        println!("{}", style("Disconnected from network").red().bold())
    }) {
        Ok(auth) => {
            handle = Some(Ok(auth));
            "Account created and logged in to SAFE network."
        }
        Err(auth_error) => {
            handle = Some(Err(auth_error));
            format!("Failed to create account: {}", &auth_error).as_str()
        }
    }
}

fn login(info: Path<(String, String)>, req: HttpRequest) -> &'static str {
    let mut handle = *(req.state().handle.lock().unwrap());
    match Authenticator::login(info.0, info.1, || {
        println!("{}", style("Disconnected from network").red().bold())
    }) {
        Ok(auth) => {
            handle = Some(Ok(auth));
            "Logged in to SAFE network."
        }
        Err(auth_error) => {
            handle = Some(Err(auth_error));
            format!("Login failed: {}", &auth_error).as_str()
        }
    }
}

fn index(req: HttpRequest) -> &'static str {
    "Hello, world!"
}

struct Authenticator {
    handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>>
}

fn main() {
    let handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>> =
        Arc::new(Mutex::new(None));

    server::new(
        || App::with_state(Authenticator{handle: handle.clone()})
           .route("/", Method::GET, index)
           .route("/login/{locator}/{password}", Method::POST, login)
           .route("/create_acc/{locator}/{password}/{invite}", Method::POST, create_acc)
           .finish()
        )
        .bind("127.0.0.1:41805")
        .unwrap()
        .run();
}
