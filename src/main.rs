use std::env;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::process::{self, Command, Stdio};

use atty;
use regex;

fn main() {
  let env_args: Vec<_> = env::args().collect();

  if env_args.len() < 3 {
    eprintln!("Must specify a cargo subcommand. E.g. `cargo first build`");
    process::exit(-1);
  }

  let mut args = vec![];
  if atty::is(atty::Stream::Stderr) {
    args.push("--color=always".to_owned());
  }

  args.extend(env::args().skip(2));

  let mut process = match Command::new("cargo")
    .args(&args)
    .stderr(Stdio::piped())
    .spawn()
  {
    Ok(process) => process,
    Err(e) => panic!("cargo {}: {}", args[0], e),
  };

  let re_error = regex::Regex::new(r#"^[^\s]*error.*:"#).unwrap();
  let re_warning = regex::Regex::new(r#"^[^\s]*warning.*:"#).unwrap();

  let reader = BufReader::new(process.stderr.take().unwrap());

  let mut found = false;
  let mut count = 0;
  for line in reader.lines().filter_map(|line| line.ok()) {
    if re_error.is_match(&line) {
      found = true;
      count += 1;
    } else if found && re_warning.is_match(&line) {
      count += 1;
    }

    if count <= 1 {
      eprintln!("{}", line);
      io::stderr().flush().ok();
    } else if found {
      process::exit(1);
    }
  }

  // if let Some(code) = process.wait().unwrap().code() {
  //   process::exit(code);
  // } else {
  //   process::exit(1);
  // }
}
