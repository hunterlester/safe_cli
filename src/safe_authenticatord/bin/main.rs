extern crate actix_web;
extern crate console;
extern crate futures;
extern crate safe_authenticator;
#[macro_use]
extern crate safe_core;
extern crate serde_json;

use actix_web::{http::Method, server, App, HttpRequest, HttpResponse, Path};
use console::style;
use safe_authenticator::{AuthError, Authenticator};
use safe_authenticator::ipc::decode_ipc_msg;
use safe_core::FutureExt;
use safe_core::ipc::{decode_msg, IpcMsg};
use safe_core::ipc::req::{IpcReq};
use std::sync::{Arc, Mutex};
use futures::future::Future;

struct AuthenticatorStruct {
    handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>>,
}

fn create_acc(
    info: Path<(String, String, String)>,
    req: HttpRequest<AuthenticatorStruct>,
) -> HttpResponse {
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

fn authorise(auth_req: Path<String>, req: HttpRequest<AuthenticatorStruct>) -> HttpResponse {
    let decoded_req = decode_msg(auth_req.as_ref()).unwrap();
    let auth = &*(req.state().handle.lock().unwrap());
    match auth {
        Some(Ok(auth_handle)) => {
            let ipc_msg : Arc<Mutex<Option<IpcMsg>>> = Arc::new(Mutex::new(None));
            let ipc_msg_clone = ipc_msg.clone();
            auth_handle.send(move |client| {
                decode_ipc_msg(client, decoded_req)
                    .and_then(move |msg| {
                        *ipc_msg_clone.lock().unwrap() = Some(msg.unwrap());
                        ok!(())
                    }).map_err(move |err| {
                        println!("decode_ipc_msg error: {:?}", err);
                    }).into_box()
                    .into()
            }).unwrap();
            while let None = *(ipc_msg.lock().unwrap()) {};
            let response_str = &*(ipc_msg.lock().unwrap());
            HttpResponse::Ok().json(response_str)
        },
        _ => {
            HttpResponse::Ok().body("Some kind of authorise error.")
        },
    }
}

fn index(_req: HttpRequest<AuthenticatorStruct>) -> &'static str {
    "Hello, world!"
}

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
                // decode_ipc_msg(client, decoded_req)
                //     .and_then(move |msg| match msg {
                //         Ok(IpcMsg::Req {
                //             req: IpcReq::Auth(auth_req),
                //             req_id,
                //         }) => {
                //             ok!(())
                //         }
                //         Ok(IpcMsg::Req {
                //             req: IpcReq::Containers(cont_req),
                //             req_id,
                //         }) => {
                //             ok!(())
                //         }
                //         Ok(IpcMsg::Req {
                //             req: IpcReq::Unregistered(extra_data),
                //             req_id,
                //         }) => {
                //             ok!(())
                //         }
                //         Ok(IpcMsg::Req {
                //             req: IpcReq::ShareMData(share_mdata_req),
                //             req_id,
                //         }) => {
                //             ok!(())     
                //         },
                //         Err((error_code, description, err)) => {
                //             ok!(())
                //         }
                //         Ok(IpcMsg::Resp { .. }) | Ok(IpcMsg::Revoked { .. }) | Ok(IpcMsg::Err(..)) => {
                //             ok!(())
                //         }
                //     }).map_err(move |err| {
                //         // call_result_cb!(Err::<(), _>(err), user_data, o_err);
                //     }).into_box()
                //     .into()
