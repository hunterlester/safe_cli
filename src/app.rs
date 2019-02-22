use actix_web::{actix, client, HttpMessage};
use futures::Future;
use safe_core::ipc::req::{AppExchangeInfo, AuthReq, Permission};
use serde_json;
// use safe_authenticator::{ Authenticator, AuthError };
// use safe_core::ipc::resp::AuthGranted;
// use safe_authenticator::test_utils::register_app;
// use safe_app::{ App, AppError };
// use std::collections::{ BTreeSet, HashMap };
use console::style;
use helpers::read_line;
use std::str;

// TODO: Use maidsafe_utilities to serialise and deserialise AuthReq, etc, (which uses bincode)

pub fn initialise(config_file_option: Option<&str>) {
    let mut id = String::new();
    let mut name = String::new();
    let mut vendor = String::new();
    let mut scope = String::new();
    match config_file_option {
        Some(config_file) => {
            println!("Handle config file passed to login, {:?}", config_file);
            id = String::from("app.id.383");
            name = String::from("Test App");
            vendor = String::from("hunterlester");
        }
        None => {
            println!("{}", style("Enter app ID:").cyan().bold());
            id = read_line(&mut id);

            println!("{}", style("Enter app name:").cyan().bold());
            name = read_line(&mut name);

            println!("{}", style("Enter app vendor name:").cyan().bold());
            vendor = read_line(&mut vendor);

            println!("{}", style("Enter app scope (optional):").cyan().bold());
            scope = read_line(&mut scope);
        }
    }

    let app_info = AppExchangeInfo {
        id: id,
        name: name,
        vendor: vendor,
        scope: match scope.len() {
            0 => None,
            _ => Some(scope),
        },
    };
    let app_info_serial: String = serde_json::to_string(&app_info).unwrap();
    println!("AppExhangeInfo as JSON: {:?}", app_info_serial);

    actix::run(move || {
        client::post(format!("http://localhost:41806/initialise"))
            .json(app_info)
            .unwrap()
            .send()
            .map_err(|err| {
                println!("App client error: {:?}", err);
            })
            .and_then(|response| {
                response
                    .body()
                    .map(move |body| (response, body))
                    .map_err(|e| println!("Error: {:?}", e))
                    .and_then(|(response, body)| {
                        println!("Response: {:?}, Body: {:?}", response, body);
                        Ok(())
                    })
                    .map(|_| actix::System::current().stop())
            })
    });
}

//pub fn authorise(app_info: AppExchangeInfo, auth: &Authenticator) -> Option<Result<AuthGranted, AuthError>> {
//   let mut user_container_dec = String::new();
//   println!("{} {:?}", style("Creating permissions for").cyan(), style(&app_info.name).cyan());
//   println!("{}", style("Create root container for app? y/n").cyan().bold());
//   user_container_dec = read_line(&mut user_container_dec);
//
//   let mut permissions = BTreeSet::new();
//   permissions.insert(Permission::Read);
//   permissions.insert(Permission::Insert);
//   permissions.insert(Permission::Delete);
//   permissions.insert(Permission::Update);
//   permissions.insert(Permission::ManagePermissions);
//
//   let mut containers = HashMap::new();
//   containers.insert(String::from("_public"), permissions.clone());
//   containers.insert(String::from("_publicNames"), permissions.clone());
//   let own_container = match user_container_dec.trim() {
//       "y" => true,
//       "n" => false,
//       _ => false,
//   };
//   let auth_req = AuthReq {
//       app: app_info,
//       app_container: own_container,
//       containers: containers,
//   };
//   let mut auth_dec = String::new();
//   println!("{}", style("Be aware that interactions with the network will occur on your behalf by this application. Do you grant authority? y/n").red().bold());
//   auth_dec = read_line(&mut auth_dec);
//   let is_granted = match auth_dec.trim() {
//     "y" => true,
//      "n" => false,
//      _ => false,
//   };
//
//   if is_granted {
//       println!("{}", style("Auth granted.").green().bold());
//       Some(register_app(auth, &auth_req))
//
//   } else {
//       println!("{}", style("Auth denied").red().bold());
//       Some(Err(AuthError::Unexpected(String::from("User denied auth"))))
//   }
//}
//
//pub fn registered(app_info: AppExchangeInfo, auth_granted: AuthGranted) -> Option<Result<App, AppError>> {
//   let app = App::registered(app_info.id, auth_granted, || println!("{}", style("Disconnected from network.").red().bold()));
//   Some(app)
//}
