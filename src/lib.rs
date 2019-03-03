use actix_web::{HttpRequest, HttpResponse, Path};
use console::style;
use futures::future::Future;
use safe_authenticator::app_auth::authenticate;
use safe_authenticator::ipc::decode_ipc_msg;
use safe_authenticator::{AuthError, Authenticator};
use safe_core::ipc::req::IpcReq;
use safe_core::ipc::resp::IpcResp;
use safe_core::ipc::{decode_msg, encode_msg, IpcMsg};
use safe_core::{ok, FutureExt};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub struct AuthenticatorStruct {
    pub handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>>,
}

pub fn create_acc(
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
            HttpResponse::BadRequest().body(response_string)
        }
    }
}

pub fn login(info: Path<(String, String)>, req: HttpRequest<AuthenticatorStruct>) -> HttpResponse {
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
            HttpResponse::BadRequest().body(response_string)
        }
    }
}

pub fn authorise(
    authenticator_req: Path<String>,
    http_req: HttpRequest<AuthenticatorStruct>,
) -> HttpResponse {
    let decoded_req = decode_msg(authenticator_req.as_ref()).unwrap();
    let authenticator: &Option<Result<Authenticator, AuthError>> =
        &*(http_req.state().handle.lock().unwrap());
    match authenticator {
        Some(Ok(auth_handle)) => {
            let ipc_msg: Arc<Mutex<Option<Value>>> = Arc::new(Mutex::new(None));
            let ipc_msg_clone = ipc_msg.clone();
            auth_handle
                .send(move |client| {
                    let c1 = client.clone();
                    let c2 = client.clone();
                    decode_ipc_msg(&c1, decoded_req)
                        .and_then(move |msg| match msg {
                            Ok(IpcMsg::Req {
                                req: IpcReq::Auth(auth_req),
                                req_id,
                            }) => authenticate(&c2, auth_req)
                                .then(move |res| {
                                    match res {
                                        Ok(auth_granted) => {
                                            let resp = IpcMsg::Resp {
                                                req_id,
                                                resp: IpcResp::Auth(Ok(auth_granted)),
                                            };
                                            let encoded_resp = encode_msg(&resp).unwrap();
                                            let json_resp = json!({ "authResp": encoded_resp });
                                            *ipc_msg_clone.lock().unwrap() = Some(json_resp);
                                        }
                                        Err(err) => {
                                            println!("Authentication error: {:?}", err);
                                        }
                                    };
                                    Ok(())
                                })
                                .into_box(),
                            Ok(IpcMsg::Req {
                                req: IpcReq::Containers(cont_req),
                                req_id,
                            }) => ok!(()),
                            Ok(IpcMsg::Req {
                                req: IpcReq::Unregistered(extra_data),
                                req_id,
                            }) => ok!(()),
                            Ok(IpcMsg::Req {
                                req: IpcReq::ShareMData(share_mdata_req),
                                req_id,
                            }) => ok!(()),
                            Err((error_code, description, error)) => ok!(()),
                            Ok(IpcMsg::Resp { .. })
                            | Ok(IpcMsg::Revoked { .. })
                            | Ok(IpcMsg::Err(..)) => ok!(()),
                        })
                        .map_err(move |err| {
                            println!("decode_ipc_msg error: {:?}", err);
                        })
                        .into_box()
                        .into()
                })
                .unwrap();
            while let None = *(ipc_msg.lock().unwrap()) {}
            let response_str = &*(ipc_msg.lock().unwrap());
            HttpResponse::Ok().json(response_str)
        }
        Some(Err(auth_error)) => HttpResponse::BadRequest().body(format!("{}", auth_error)),
        None => HttpResponse::BadRequest().body("Authenticator is not logged in."),
    }
}

pub fn index(_req: HttpRequest<AuthenticatorStruct>) -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}
