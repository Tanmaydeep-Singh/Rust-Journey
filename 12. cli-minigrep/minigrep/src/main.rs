use std::env;
use std::process;
use minigrep::Config;

fn main() {
  // Get the search configuration
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  println!("Minigrep");
  println!("  Set any value to CASE_INSENSITIVE environment variable to use \
    case insensitive search.");
  // Perform the search
  if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {}.", e);
    process::exit(1);
  }
}