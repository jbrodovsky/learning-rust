use std::env;
use std::process;

use ch12_cli::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    //let config = Config::new(&args);
    let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = ch12_cli::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
