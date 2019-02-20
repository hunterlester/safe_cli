extern crate actix_web;
extern crate console;
extern crate safe_authenticator;

use actix_web::{http::Method, server, App, HttpRequest, Path, HttpResponse};
use console::style;
use safe_authenticator::{AuthError, Authenticator};
use std::sync::{Arc, Mutex};

struct AuthenticatorStruct {
    handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>>
}

fn create_acc(info: Path<(String, String, String)>, req: HttpRequest<AuthenticatorStruct>) -> HttpResponse {
    match Authenticator::create_acc(info.0.clone(), info.1.clone(), info.2.clone(), || {
        println!("{}", style("Disconnected from network").red().bold())
    }) {
        Ok(auth) => {
            *(req.state().handle.lock().unwrap()) = Some(Ok(auth));
            HttpResponse::Ok().body("Account created and logged in to SAFE network.")
        }
        Err(auth_error) => {
            let response_string = format!("Failed to create account: {} ", &auth_error);
            *(req.state().handle.lock().unwrap()) = Some(Err(auth_error));
            HttpResponse::Ok().body(response_string)
        }
    }
}

fn login(info: Path<(String, String)>, req: HttpRequest<AuthenticatorStruct>) -> HttpResponse {
    match Authenticator::login(info.0.clone(), info.1.clone(), || {
        println!("{}", style("Disconnected from network").red().bold())
    }) {
        Ok(auth) => {
            *(req.state().handle.lock().unwrap()) = Some(Ok(auth));
            HttpResponse::Ok().body("Logged in to SAFE network.")
        }
        Err(auth_error) => {
            let response_string = format!("Login failed: {} ", &auth_error);
            *(req.state().handle.lock().unwrap()) = Some(Err(auth_error));
            // format!("Login failed: {}", &auth_error)
            HttpResponse::Ok().body(response_string)
        }
    }
}

fn index(_req: HttpRequest<AuthenticatorStruct>) -> &'static str {
    "Hello, world!"
}

fn main() {
    let handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>> =
        Arc::new(Mutex::new(None));

    server::new(
        move || App::with_state(AuthenticatorStruct{handle: handle.clone()})
           .resource("/", |r| { r.method(Method::GET).with(index); })
           .resource("/login/{locator}/{password}", |r| { r.method(Method::POST).with(login); })
           .resource("/create_acc/{locator}/{password}/{invite}", |r| { r.method(Method::POST).with(create_acc); })
           .finish()
        )
        .bind("127.0.0.1:41805")
        .unwrap()
        .run();
}
