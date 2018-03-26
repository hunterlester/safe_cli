use zxcvbn::zxcvbn;
use console::style;
use helpers::{ read_line };
use std::process::Command;

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
    // TODO: Understand security concerns for passing sensiste\
    // data to child propcesses
    let mut child = Command::new("C:\\Users\\guilf\\safe\\dev\\safe_cli\\target\\debug\\safe_authenticatord.exe")
                .arg("create_acc")
                .arg(locator)
                .arg(password)
                .arg(invite)
                .spawn()
                .expect("Authenticator process failed to start");
    child.wait().expect("Failed to wait on child");
    ()
}

pub fn login(config_file_option: Option<&str>) -> () {
    let mut locator: String;
    let mut password: String;
    match config_file_option {
        Some(config_file) => {
          println!("Handle config file passed to login, {:?}", config_file);
          locator = String::from("guilfordhunterlester");
          password = String::from("guilfordhunterlester");
        },
        None => {
          println!("{}", style("Please enter your locator:").yellow().bold());
          locator = String::new();
          locator = read_line(&mut locator);
          println!("{}", style("Please enter your password:").yellow().bold());
          password = String::new();
          password = read_line(&mut password);
        }
    }
    // TODO: Understand security concerns for passing sensiste\
    // data to child propcesses
    let mut child = Command::new("C:\\Users\\guilf\\safe\\dev\\safe_cli\\target\\debug\\safe_authenticatord.exe")
                .arg("login")
                .arg(locator)
                .arg(password)
                .spawn()
                .expect("App process failed to start");
    child.wait().expect("Failed to wait on child");
    ()
}
