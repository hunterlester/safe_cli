use crate::helpers::read_line;
use actix_web::{actix, client, HttpMessage};
use console::style;
use futures::Future;
use safe_core::ipc::req::{AppExchangeInfo, AuthReq, IpcReq, Permission};
use safe_core::ipc::{encode_msg, gen_req_id, IpcMsg};
use std::collections::{BTreeSet, HashMap};
use zxcvbn::zxcvbn;

fn validate_cred(cred: &'static str) -> String {
    println!(
        "{} {}:",
        style("Please choose a").yellow().bold(),
        style(&cred).yellow().bold()
    );
    let mut secret = String::new();
    secret = read_line(&mut secret);
    let secret_strength = zxcvbn(&secret, &[]).unwrap();
    println!(
        "{} {}:\n 
              {}: {}\n
              {}: {}\n
              {}: {}\n
              {}: {}",
        style("Interesting information about your").cyan(),
        style(&cred).cyan(),
        style("Estimated number of guesses needed to crack").cyan(),
        &secret_strength.guesses,
        style("Estimated number of seconds needed with fastest cracking method").cyan(),
        &secret_strength
            .crack_times_seconds
            .offline_fast_hashing_1e10_per_second,
        style("In short, it would take with quickest cracking method").cyan(),
        &secret_strength
            .crack_times_display
            .offline_fast_hashing_1e10_per_second,
        style("On a scale of 0-4, your score is").cyan(),
        &secret_strength.score
    );

    if secret_strength.score <= 2 {
        let feedback = secret_strength.feedback.unwrap();
        let warning = match feedback.warning {
            Some(ref warn) => warn,
            None => "Entered data is too simple to be secure.",
        };
        println!(
            "\n{} {} {}\n
                {}: {}
                {}: {:?}",
            style("Your").red().bold(),
            style(&cred).red().bold(),
            style("is not strong enough.").red().bold(),
            style("Security feeback").red().bold(),
            style(warning).red().bold(),
            style("Suggestions").cyan().bold(),
            style(&feedback.suggestions).cyan().bold()
        );
        validate_cred(cred)
    } else {
        println!(
            "{} {} {}:",
            style("Please type in your").yellow().bold(),
            style(&cred).yellow().bold(),
            style("again to confirm").yellow().bold()
        );
        let mut secret_compare = String::new();
        secret_compare = read_line(&mut secret_compare);
        if secret == secret_compare {
            secret
        } else {
            println!(
                "{} {}",
                style(&cred).red().bold(),
                style("does not match.").red().bold()
            );
            validate_cred(cred)
        }
    }
}

pub fn create_acc(config_file_option: Option<&str>) {
    let locator: String;
    let password: String;
    let mut invite: String;
    match config_file_option {
        Some(config_file) => {
            println!("Handle Config file passed to create_acc, {:?}", config_file);
            locator = String::from("guilfordhunterlester");
            password = String::from("guilfordhunterlester");
            invite = String::from("guilfordhunterlester");
        }
        None => {
            locator = validate_cred("locator");
            println!("{}", style("Valid secret").green().bold());
            password = validate_cred("password");
            println!("{}", style("Valid password").green().bold());
            println!(
                "{}",
                style("Please enter your invite code:").yellow().bold()
            );
            invite = String::new();
            invite = read_line(&mut invite);
        }
    }
    let c_locator: String = locator.clone();
    let c_password: String = password.clone();
    let c_invite: String = invite.clone();
    actix::run(move || {
        client::post(format!(
            "http://localhost:41805/create_acc/{}/{}/{}",
            &c_locator, &c_password, &c_invite
        ))
        .finish()
        .unwrap()
        .send()
        .map_err(|err| {
            println!(
                "{}: {}",
                err,
                style("Authenticatord is not running, please execute it first, then run command again.").red().bold(),
            );
            actix::System::current().stop()
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
        }).map(|_| actix::System::current().stop())
    });
}

pub fn login(config_file_option: Option<&str>) {
    let locator: String;
    let password: String;
    match config_file_option {
        Some(config_file) => {
            println!("Handle config file passed to login, {:?}", config_file);
            locator = String::from("guilfordhunterlester");
            password = String::from("guilfordhunterlester");
        }
        None => {
            locator = validate_cred("locator");
            println!("{}", style("Valid secret").green().bold());
            password = validate_cred("password");
            println!("{}", style("Valid password").green().bold());
        }
    }
    let c_locator: String = locator.clone();
    let c_password: String = password.clone();
    actix::run(move || {
        client::post(format!(
            "http://localhost:41805/login/{}/{}",
            &c_locator, &c_password
        ))
        .finish()
        .unwrap()
        .send()
        .map_err(|err| {
            println!(
                "{}: {}",
                err,
                style("Authenticatord is not running, please execute it first, then run command again.").red().bold(),
            );
            actix::System::current().stop()
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
        }).map(|_| actix::System::current().stop())
    });
}

pub fn authorise(config_file_option: Option<&str>) {
    let mut id = String::new();
    let mut name = String::new();
    let mut vendor = String::new();
    let mut scope = String::new();
    let mut containers = HashMap::new();
    let mut own_container: bool;
    match config_file_option {
        Some(config_file) => {
            println!("Handle config file passed to login, {:?}", config_file);
            // Temporary default values for testing
            id = String::from("app.id.383");
            name = String::from("Test App");
            vendor = String::from("MaidSafe.net Ltd.");
            let mut permissions = BTreeSet::new();
            permissions.insert(Permission::Read);
            permissions.insert(Permission::Insert);
            permissions.insert(Permission::Delete);
            permissions.insert(Permission::Update);
            permissions.insert(Permission::ManagePermissions);

            containers.insert(String::from("_public"), permissions.clone());
            containers.insert(String::from("_publicNames"), permissions.clone());
            own_container = false;
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

            let mut user_container_dec = String::new();
            println!(
                "{} {:?}",
                style("Creating permissions for").cyan(),
                style(&name).cyan()
            );
            println!(
                "{}",
                style("Create root container for app? y/n").cyan().bold()
            );
            user_container_dec = read_line(&mut user_container_dec);

            let mut permissions = BTreeSet::new();
            permissions.insert(Permission::Read);
            permissions.insert(Permission::Insert);
            permissions.insert(Permission::Delete);
            permissions.insert(Permission::Update);
            permissions.insert(Permission::ManagePermissions);

            containers.insert(String::from("_public"), permissions.clone());
            containers.insert(String::from("_publicNames"), permissions.clone());
            own_container = match user_container_dec.trim() {
                "y" => true,
                "n" => false,
                _ => false,
            };
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

    let auth_req = AuthReq {
        app: app_info,
        app_container: own_container,
        containers: containers,
    };
    let req_id = gen_req_id();
    let encoded_auth_req = encode_msg(&IpcMsg::Req {
        req_id,
        req: IpcReq::Auth(auth_req),
    })
    .unwrap();
    println!("encoded_auth_req: {:?}", &encoded_auth_req);

    actix::run(move || {
        client::post(format!(
            "http://localhost:41805/authorise/{}",
            &encoded_auth_req
        ))
        .finish()
        .unwrap()
        .send()
        .map_err(|err| {
            println!(
                "{}: {}",
                err,
                style("Authenticatord is not running, please execute it first, then run command again.").red().bold(),
            );
            actix::System::current().stop()
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
        }).map(|_| actix::System::current().stop())
    });
}
