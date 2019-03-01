mod lib;

use crate::lib::{authorise, create_acc, index, login, AuthenticatorStruct};
use actix_web::{http::Method, server, App};
use safe_authenticator::{AuthError, Authenticator};
use std::sync::{Arc, Mutex};

fn main() {
    let handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>> = Arc::new(Mutex::new(None));

    server::new(move || {
        App::with_state(AuthenticatorStruct {
            handle: handle.clone(),
        })
        .resource("/", |r| {
            r.method(Method::GET).with(index);
        })
        .resource("/login/{locator}/{password}", |r| {
            r.method(Method::POST).with(login);
        })
        .resource("/create_acc/{locator}/{password}/{invite}", |r| {
            r.method(Method::POST).with(create_acc);
        })
        .resource("/authorise/{auth_req}", |r| {
            r.method(Method::POST).with(authorise);
        })
        .finish()
    })
    .bind("127.0.0.1:41805")
    .unwrap()
    .run();
}
