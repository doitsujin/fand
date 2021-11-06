pub mod app;
pub mod fan;
pub mod input;
pub mod output;
pub mod parser;
pub mod sensor;
pub mod util;

use crate::app::{App, Config};
use crate::parser::Parser;

use std::io::Write;
use std::path::Path;

fn global_config_path() -> Result<String, String> {
    std::env::args()
        .nth(1)
        .ok_or("No configuration file specified".to_string())
}

fn run_app() -> Result<(), String> {
    let config_path = global_config_path()?;
    let content = util::read_text_file(Path::new(&config_path))?;

    let config = Parser::parse_document(&content)?;
    let mut app = App::from_config(Config::parse(&config)?);
    app.run()
}

fn main() {
    match run_app() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            writeln!(&mut std::io::stderr(), "{}", &e).unwrap();
            std::process::exit(1);
        }
    }
}
