use zxcvbn::zxcvbn;
use console::style;
use helpers::{ read_line };
use std::process::{ Command, Stdio };
use tokio;
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, Shutdown};
use std::io::BufReader;
use std::env::current_dir;

fn validate_cred(cred: &'static str) -> String {
    println!("{} {}:", style("Please choose a").yellow().bold(), style(&cred).yellow().bold());
    let mut secret = String::new();
    secret = read_line(&mut secret);
    let secret_strength = zxcvbn(&mut secret, &[]).unwrap();
    println!("{} {}:\n 
              {}: {}\n
              {}: {}\n
              {}: {}\n
              {}: {}", 
        style("Interesting information about your").cyan(), style(&cred).cyan(),  
        style("Estimated number of guesses needed to crack").cyan(), &secret_strength.guesses,
        style("Estimated number of seconds needed with fastest cracking method").cyan(), &secret_strength.crack_times_seconds.offline_fast_hashing_1e10_per_second,
        style("In short, it would take with quickest cracking method").cyan(), &secret_strength.crack_times_display.offline_fast_hashing_1e10_per_second,
        style("On a scale of 0-4, your score is").cyan(), &secret_strength.score
    );

    if secret_strength.score <= 2 {
      let feedback = &secret_strength.feedback.unwrap();
      let warning = match &feedback.warning {
          &Some(ref warn) => warn,
          &None => "Entered data is too simple to be secure.",
      };
      println!("\n{} {} {}\n
                {}: {}
                {}: {:?}", 
          style("Your").red().bold(), style(&cred).red().bold(), style("is not strong enough.").red().bold(),
          style("Security feeback").red().bold(), style(warning).red().bold(),
          style("Suggestions").cyan().bold(), style(&feedback.suggestions).cyan().bold()
      );
      validate_cred(cred)
    } else {
      println!("{} {} {}:", style("Please type in your").yellow().bold(), style(&cred).yellow().bold(), style("again to confirm").yellow().bold());
      let mut secret_compare = String::new();
      secret_compare = read_line(&mut secret_compare);
      if &mut secret == &mut secret_compare {
        secret
      } else {
        println!("{} {}", style(&cred).red().bold(), style("does not match.").red().bold());
        validate_cred(cred)
      }
    }
}

pub fn create_acc(config_file_option: Option<&str>) -> () {
    let locator: String;
    let password: String;
    let mut invite: String; 
    match config_file_option {
        Some(config_file) => {
          println!("Handle Config file passed to create_acc, {:?}", config_file);
          locator = String::from("guilfordhunterlester");
          password = String::from("guilfordhunterlester");
          invite = String::from("guilfordhunterlester");
        },
        None => {
          locator = validate_cred("locator");
          println!("{}", style("Valid secret").green().bold());
          password = validate_cred("password");
          println!("{}", style("Valid password").green().bold());
          println!("{}", style("Please enter your invite code:").yellow().bold());
          invite = String::new();
          invite = read_line(&mut invite);
        }
    }
    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 41805);
    let c_locator: String = locator.clone();
    let c_password: String = password.clone();
    let c_invite: String = invite.clone(); 
    let client = TcpStream::connect(&socket_address).and_then(move |stream| {
        println!("stream connected: {}", stream.local_addr().unwrap());
        io::write_all(stream, format!("IPC create_acc {} {} {}\n", c_locator, c_password, c_invite).into_bytes())
    })
    .and_then(|(stream, _)| {
        let socket = BufReader::new(stream);
        io::lines(socket).for_each(|line| {
            println!("Authenticator response: {}", line);
            // TODO: Needs to shutdown stream and end process once response has been received
            Ok(()) 
        }).then(|_| Ok(()))
    })
    .map_err(|err| {
        println!("{:?}: No running instance, executing authenticatord...", err);
        let mut path = current_dir().unwrap();
        path.push("target");
        path.push("debug");
        if cfg!(windows) {
            path.push("safe_authenticatord.exe");
        } else {
            path.push("safe_authenticatord");
        }
        let mut child = Command::new(path.to_str().unwrap())
            // This is me playing with std IO
            //.stdin(Stdio::null())
            //.stdout(Stdio::null())
            //.stderr(Stdio::null())
            .arg("create_acc")
            .arg(locator)
            .arg(password)
            .arg(invite)
            .spawn()
            .expect("Authenticator process failed to start");
       child.wait().expect("Failed to wait on child");
    });
    tokio::run(client);
    ()
}

pub fn login(config_file_option: Option<&str>) -> () {
    let locator: String;
    let password: String;
    match config_file_option {
        Some(config_file) => {
          println!("Handle config file passed to login, {:?}", config_file);
          locator = String::from("guilfordhunterlester");
          password = String::from("guilfordhunterlester");
        },
        None => {
          locator = validate_cred("locator");
          println!("{}", style("Valid secret").green().bold());
          password = validate_cred("password");
          println!("{}", style("Valid password").green().bold());
        }
    }
    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 41805);
    let c_locator: String = locator.clone();
    let c_password: String = password.clone();
    let client = TcpStream::connect(&socket_address).and_then(move |stream| {
        println!("stream connected: {}", stream.local_addr().unwrap());
        io::write_all(stream, format!("IPC login {} {}\n", c_locator, c_password).into_bytes())
    })
    .and_then(|(stream, _)| {
        let socket = BufReader::new(stream);
        io::lines(socket).for_each(|line| {
            println!("Authenticator response: {}", line);
            // TODO: Needs to shutdown stream and end process once response has been received
            Ok(()) 
        }).then(|_| Ok(()))
    })
    .map_err(|err| {
        println!("{:?}: No running instance, executing authenticatord...", err);
        let mut path = current_dir().unwrap();
        path.push("target");
        path.push("debug");
        if cfg!(windows) {
            path.push("safe_authenticatord.exe");
        } else {
            path.push("safe_authenticatord");
        }
        let mut child = Command::new(path.to_str().unwrap())
            // This is me playing with std IO
            //.stdin(Stdio::null())
            //.stdout(Stdio::null())
            //.stderr(Stdio::null())
            .arg("create_acc")
            .arg(locator)
            .arg(password)
            .spawn()
            .expect("Authenticator process failed to start");
       child.wait().expect("Failed to wait on child");
    });
    tokio::run(client);
    ()
}
