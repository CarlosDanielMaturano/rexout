pub mod byte_ordering;
pub mod cli_parser;
pub mod file_operations;

use byte_ordering::order_bytes;
use cli_parser::Cli;
use file_operations::read_file_content;

fn show_help() {
    let help = String::from("Help");
    eprintln!("{}", help);
}

fn main() -> Result<(), String> {
    let args = std::env::args();
    let Cli { file_path, options }: Cli = Cli::try_from(args).map_err(|err| {
        show_help();
        err
    })?;

    let bytes = order_bytes(read_file_content(file_path)?, options.little_endian);
    Ok(())
}
