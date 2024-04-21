use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("yamd1")
        .about("rust uniq")
        .arg(
            Arg::new("in_file")
                .help("")
                .value_name("IN_FILE")
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .help("")
                .value_name("OUT_FILE")
                .action(ArgAction::Append)
                .default_value(None)
                .last(true),
        )
        .arg(
            Arg::new("count")
                .help("")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .get_matches();

    let in_file = matches.get_one::<String>("in_file").unwrap().to_owned();

    let count = matches.get_flag("count");

    Ok(Config {
        in_file,
        out_file: Some("bar".to_string()),
        count,
    })
}
