use actix_web::{http::Method, server, App, HttpRequest, HttpResponse, Json};
use std::sync::{Arc, Mutex};

//extern crate safe_app;

use safe_core::ipc::req::AppExchangeInfo;
//use safe_core::ipc::resp::AuthGranted;
//use safe_app::{ App, AppError };

struct AppExchangeInfoStruct {
    handle: Arc<Mutex<Option<AppExchangeInfo>>>,
}

fn initialise(
    info: Json<AppExchangeInfo>,
    req: HttpRequest<AppExchangeInfoStruct>,
) -> HttpResponse {
    let response_str = format!("{} initialised.", &info.name);
    let app_info: AppExchangeInfo = info.into_inner();
    *(req.state().handle.lock().unwrap()) = Some(app_info);
    HttpResponse::Ok().body(&response_str)
}

fn main() {
    let handle: Arc<Mutex<Option<AppExchangeInfo>>> = Arc::new(Mutex::new(None));

    server::new(move || {
        App::with_state(AppExchangeInfoStruct {
            handle: handle.clone(),
        })
        .resource("/initialise", |r| {
            r.method(Method::POST).with(initialise);
        })
        .finish()
    })
    .bind("127.0.0.1:41806")
    .unwrap()
    .run();
}
