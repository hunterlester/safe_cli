use actix_web::{actix::*, ws, Error, HttpRequest, HttpResponse, Path};
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
use std::time::{Duration, Instant};

// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct AuthenticatorStruct {
    pub handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>>,
}

struct WebSocket {
    hb: Instant,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self, AuthenticatorStruct>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for WebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}

impl WebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping("");
        });
    }
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
            HttpResponse::BadRequest().body(response_string)
        }
    }
}

pub fn authorise(
    authenticator_req: Path<String>,
    http_req: HttpRequest<AuthenticatorStruct>,
) -> HttpResponse {
    let decoded_req = decode_msg(authenticator_req.as_ref()).unwrap();
    let auth_granted: bool = match http_req.query().get("auth_granted") {
        Some(is_granted) => {
            if is_granted.contains("true") {
                true
            } else {
                false
            }
        }
        None => false,
    };
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
                            }) => {
                                if auth_granted {
                                    authenticate(&c2, auth_req)
                                        .then(move |res| {
                                            match res {
                                                Ok(auth_granted) => {
                                                    let resp = IpcMsg::Resp {
                                                        req_id,
                                                        resp: IpcResp::Auth(Ok(auth_granted)),
                                                    };
                                                    let encoded_resp = encode_msg(&resp).unwrap();
                                                    let json_resp =
                                                        json!({ "authResp": encoded_resp });
                                                    *ipc_msg_clone.lock().unwrap() =
                                                        Some(json_resp);
                                                }
                                                Err(err) => {
                                                    println!("Authentication error: {:?}", err);
                                                }
                                            };
                                            Ok(())
                                        })
                                        .into_box()
                                } else {
                                    *ipc_msg_clone.lock().unwrap() =
                                        Some(json!({"error": "Auth not granted"}));
                                    ok!(())
                                }
                            }
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

pub fn web_socket(req: HttpRequest<AuthenticatorStruct>) -> Result<HttpResponse, Error> {
    ws::start(&req, WebSocket::new())
}
