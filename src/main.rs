use std::env;
use std::io::{BufRead, BufReader};
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

  let process = match Command::new("cargo")
    .args(&args)
    .stderr(Stdio::piped())
    .spawn()
  {
    Ok(process) => process,
    Err(e) => panic!("cargo {}: {}", args[0], e),
  };

  let re = regex::Regex::new(r#"^[^\s]*error.*:"#).unwrap();

  let reader = BufReader::new(process.stderr.unwrap());
  let mut errors = 0;
  for line in reader.lines().filter_map(|line| line.ok()) {
    if re.is_match(&line) {
      errors += 1;
    }

    if errors >= 2 {
      process::exit(1);
    }

    eprintln!("{}", line);
  }
}
