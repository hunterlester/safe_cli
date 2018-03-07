use safe_core::ipc::req::{ Permission, AppExchangeInfo, AuthReq };
use safe_authenticator::{ Authenticator, AuthError };
use safe_core::ipc::resp::AuthGranted;
use safe_authenticator::test_utils::register_app;
use safe_app::{ App, AppError };
use std::io;
use std::collections::{ BTreeSet, HashMap };
use console::style;

pub fn initialise() -> Option<AppExchangeInfo> {
    let mut id = String::new();
    let mut name = String::new();
    let mut vendor = String::new();
    let mut scope = String::new();
    println!("{}", style("Enter app ID:").cyan().bold());
    io::stdin().read_line(&mut id).expect("String");
    
    println!("{}", style("Enter app name:").cyan().bold());
    io::stdin().read_line(&mut name).expect("String");

    println!("{}", style("Enter app vendor name:").cyan().bold());
    io::stdin().read_line(&mut vendor).expect("String");

    println!("{}", style("Enter app scope (optional):").cyan().bold());
    io::stdin().read_line(&mut scope).expect("String");
    scope = scope.trim().to_string();

    Some(AppExchangeInfo {
      id: id.trim().to_string(),
      name: name.trim().to_string(),
      vendor: vendor.trim().to_string(),
      scope: match scope.len() {
          0 => None,
          _ => Some(scope),
      },
    })
}

pub fn authorise(app_info: AppExchangeInfo, auth: &Authenticator) -> Option<Result<AuthGranted, AuthError>> {
   let mut user_container_dec = String::new();
   println!("Creating permissions for {:?}", app_info.name);
   println!("{}", style("Create root container for app? y/n").cyan().bold());
   io::stdin().read_line(&mut user_container_dec).expect("String");

   let mut permissions = BTreeSet::new();
   permissions.insert(Permission::Read);
   permissions.insert(Permission::Insert);
   permissions.insert(Permission::Delete);
   permissions.insert(Permission::Update);
   permissions.insert(Permission::ManagePermissions);

   let mut containers = HashMap::new();
   containers.insert(String::from("_public"), permissions.clone());
   containers.insert(String::from("_publicNames"), permissions.clone());
   let own_container = match user_container_dec.trim() {
       "y" => true,
       "n" => false,
       _ => false,
   };
   let auth_req = AuthReq {
       app: app_info,
       app_container: own_container,
       containers: containers,
   };
   let mut auth_dec = String::new();
   println!("{}", style("Be aware that interactions with the network will occur on your behalf by this application. Do you grant authority? y/n").red().bold());
   io::stdin().read_line(&mut auth_dec).expect("String");
   let is_granted = match auth_dec.trim() {
     "y" => true,
      "n" => false,
      _ => false,
   };

   if is_granted {
       println!("{}", style("Auth granted.").green().bold());
       Some(register_app(auth, &auth_req))
       
   } else {
       println!("{}", style("Auth denied").red().bold());
       Some(Err(AuthError::Unexpected(String::from("User denied auth"))))
   }
}

pub fn registered(app_info: AppExchangeInfo, auth_granted: AuthGranted) -> Option<Result<App, AppError>> {
   let app = App::registered(app_info.id, auth_granted, || println!("{}", style("Disconnected from network.").red().bold()));
   Some(app)
}
