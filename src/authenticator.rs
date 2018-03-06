use safe_authenticator::{ Authenticator, AuthError };
use zxcvbn::zxcvbn;
use std::io;
use console::style;

fn validate_cred(cred: &'static str) -> String {
    println!("Please choose a {}:", &cred);
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).expect("Please enter valid string");
    secret = secret.trim().to_string();
    let secret_strength = zxcvbn(&mut secret, &[]).unwrap();
    println!("\nInteresting information about your {}:\n
      Estimated number of guesses needed to crack: {:?}\n
      Estimated number of seconds needed with fastest cracking method: {:?}\n
      In short, it would take with quickest cracking method: {:?}\n
      On a scale of 0-4, your score is: {:?}\n", &cred, &secret_strength.guesses, &secret_strength.crack_times_seconds.offline_fast_hashing_1e10_per_second, &secret_strength.crack_times_display.offline_fast_hashing_1e10_per_second, &secret_strength.score);

    if secret_strength.score <= 2 {
      println!("\nYour {} is not strong enough.\n Here is how to make it stronger: \n{:?}", &cred, &secret_strength.feedback.unwrap());
      validate_cred(cred)
    } else {
      println!("Please type in your {} again to confirm:", &cred);
      let mut secret_compare = String::new();
      io::stdin().read_line(&mut secret_compare).expect("Please enter valid string");
      secret_compare = secret_compare.trim().to_string();
      if &mut secret == &mut secret_compare {
        secret
      } else {
        println!("{} does not match. Starting over...", &cred);
        validate_cred(cred)
      }
    }
}

pub fn create_acc() -> Option<Result<Authenticator, AuthError>> {
    let secret = validate_cred("secret");
    println!("\u{2705}Valid secret");
    let password = validate_cred("password");
    println!("\u{2705}Valid password");
    println!("Please enter your invite code:");
    let mut invite = String::new();
    io::stdin().read_line(&mut invite).expect("Please enter valid string");
    invite = invite.trim().to_string();
    match Authenticator::create_acc(secret, password, invite, || println!("Disconnected from network")) {
      Ok(auth) =>  {
          println!("{}", style("Logged in to SAFE network.").green().bold());
          Some(Ok(auth))
      },
      Err(auth_error) => {
          println!("Failed to create account: {:?}", &auth_error);
          Some(Err(auth_error))
      },
    }
}

pub fn login() -> Option<Result<Authenticator, AuthError>> {
      println!("Please enter your secret:");
      let mut secret = String::new();
      io::stdin().read_line(&mut secret).expect("Please enter valid string");
      secret = secret.trim().to_string();
      println!("Please enter your password:");
      let mut password = String::new();
      io::stdin().read_line(&mut password).expect("Please enter valid string");
      password = password.trim().to_string();
      match Authenticator::login(secret, password, || println!("Disconnected from network")) {
      Ok(auth) =>  {
          println!("{}", style("Logged in to SAFE network.").green().bold());
          Some(Ok(auth))
      },
      Err(auth_error) => {
          println!("Login failed: {:?}", &auth_error);
          Some(Err(auth_error))
      },
    }
}
