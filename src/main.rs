extern crate cargo;
extern crate rustc_serialize;
extern crate colored;

mod msg;
mod marvin;

use std::io::{ self, Write };

use cargo::{ human, CargoResult, CliResult, CliError, Config };
use cargo::util::process_builder::process;
use rustc_serialize::json;

use marvin::Marvinable;

const USAGE: &'static str = "
Usage:
    cargo marvin <args>...
";

#[derive(RustcDecodable)]
struct Options {
    arg_args: Vec<String>,
}

fn main() {
    cargo::execute_main_without_stdin(real_main, true, USAGE);
}

fn real_main(options: Options, config: &Config) -> CliResult<Option<()>> {
    real_real_main(options, config).map_err(|e| CliError::new(e, 1))
}

fn real_real_main(options: Options, _config: &Config) -> CargoResult<Option<()>> {
    let mut process = process("cargo");
    process.args(&options.arg_args);
    process.args(&[
        "--message-format", "json",
        "--color", "always",
    ]);
    let output = process.exec_with_streaming(
        &mut |line| {
            match json::decode::<msg::Line>(&line) {
                Ok(line) => print!("{}", line.marvined()),
                Err(_) => {
                    println!("{}", line);
                }
            };
            Ok(())
        },
        &mut |line| {
            write!(io::stderr(), "{}\n", line).unwrap();
            Ok(())
        }).map_err(|_| human("subcommand failed"))?;
    if output.status.success() {
        Ok(None)
    } else {
        Err(human("subcommand failed"))
    }
}
