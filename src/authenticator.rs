use safe_authenticator::{ Authenticator, AuthError };
use zxcvbn::zxcvbn;
use console::style;
use helpers::{ read_line };

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

pub fn create_acc() -> Option<Result<Authenticator, AuthError>> {
    let locator = validate_cred("locator");
    println!("{}", style("Valid secret").green().bold());
    let password = validate_cred("password");
    println!("{}", style("Valid password").green().bold());
    println!("{}", style("Please enter your invite code:").yellow().bold());
    let mut invite = String::new();
    invite = read_line(&mut invite);
    match Authenticator::create_acc(locator, password, invite, || println!("{}", style("Disconnected from network").red().bold())) {
      Ok(auth) =>  {
          println!("{}", style("Logged in to SAFE network.").green().bold());
          Some(Ok(auth))
      },
      Err(auth_error) => {
          println!("{}: {}", style("Failed to create account").red().bold(), style(&auth_error).red().bold());
          Some(Err(auth_error))
      },
    }
}

pub fn login() -> Option<Result<Authenticator, AuthError>> {
      println!("{}", style("Please enter your locator:").yellow().bold());
      let mut locator = String::new();
      locator = read_line(&mut locator);
      println!("{}", style("Please enter your password:").yellow().bold());
      let mut password = String::new();
      password = read_line(&mut password);
      match Authenticator::login(locator, password, || println!("{}", style("Disconnected from network").red().bold())) {
      Ok(auth) =>  {
          println!("{}", style("Logged in to SAFE network.").green().bold());
          Some(Ok(auth))
      },
      Err(auth_error) => {
          println!("{}: {}", style("Login failed").red().bold(), style(&auth_error).red().bold());
          Some(Err(auth_error))
      },
    }
}
