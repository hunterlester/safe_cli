use actix_web::{http::Method, test, App, HttpMessage};
use rand::Rng;
use safe_authenticator::{AuthError, Authenticator};
use safe_cli::{authorise, create_acc, index, login, AuthenticatorStruct};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

fn create_test_service() -> App<AuthenticatorStruct> {
    let handle: Arc<Mutex<Option<Result<Authenticator, AuthError>>>> = Arc::new(Mutex::new(None));
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
}

#[test]
fn get_index() {
    let mut srv = test::TestServer::with_factory(create_test_service);
    let request = srv.client(Method::GET, "/").finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());
}

#[test]
fn post_create_acc() {
    let mut rng = rand::thread_rng();
    let locator: u32 = rng.gen();
    let password: u32 = rng.gen();
    let invite: u16 = rng.gen();
    let mut srv = test::TestServer::with_factory(create_test_service);
    let endpoint = format!("/create_acc/{}/{}/{}", locator, password, invite);
    let request = srv.client(Method::POST, &endpoint).finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());
}

#[test]
fn post_login() {
    let mut rng = rand::thread_rng();
    let locator: u32 = rng.gen();
    let password: u32 = rng.gen();
    let invite: u16 = rng.gen();
    let mut srv = test::TestServer::with_factory(create_test_service);

    let create_acc_endpoint = format!("/create_acc/{}/{}/{}", locator, password, invite);
    let create_acc_request = srv
        .client(Method::POST, &create_acc_endpoint)
        .finish()
        .unwrap();
    let create_acc_response = srv.execute(create_acc_request.send()).unwrap();

    assert!(create_acc_response.status().is_success());

    let login_endpoint = format!("/login/{}/{}", locator, password);
    let login_request = srv.client(Method::POST, &login_endpoint).finish().unwrap();
    let login_response = srv.execute(login_request.send()).unwrap();

    assert!(login_response.status().is_success());
}

#[test]
#[ignore]
fn post_authorise() {
    let mut rng = rand::thread_rng();
    let locator: u32 = rng.gen();
    let password: u32 = rng.gen();
    let invite: u16 = rng.gen();
    let mut srv = test::TestServer::with_factory(create_test_service);

    let create_acc_endpoint = format!("/create_acc/{}/{}/{}", locator, password, invite);
    let create_acc_request = srv
        .client(Method::POST, &create_acc_endpoint)
        .finish()
        .unwrap();
    let create_acc_response = srv.execute(create_acc_request.send()).unwrap();

    assert!(create_acc_response.status().is_success());

    let auth_req = "bAAAAAACTBZGGMAAAAAABGAAAAAAAAAAANB2W45DFOIXGYZLTORSXELRUHAXDGOAACYAAAAAAAAAAAR3VNFWGM33SMQQEQ5LOORSXEICMMVZXIZLSCEAAAAAAAAAAATLBNFSFGYLGMUXG4ZLUEBGHIZBOAEBAAAAAAAAAAAAHAAAAAAAAAAAF64DVMJWGSYYFAAAAAAAAAAAAAAAAAAAQAAAAAIAAAAADAAAAABAAAAAAYAAAAAAAAAAAL5YHKYTMNFRU4YLNMVZQKAAAAAAAAAAAAAAAAAABAAAAAAQAAAAAGAAAAACAAAAAAE";
    let endpoint = format!("/authorise/{}", auth_req);
    let request = srv.client(Method::POST, &endpoint).finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
    let bytes = srv.execute(response.body()).unwrap();
    let body = from_utf8(&bytes).unwrap();
    assert_eq!(body, "Hello world!");
}
