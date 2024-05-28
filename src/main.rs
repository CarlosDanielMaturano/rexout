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

fn print_words(words: Vec<u16>) {
    let mut offset = 0;
    words.chunks(8).for_each(|chunk| {
        print!("{:08x} ", offset);
        chunk.into_iter().for_each(|word| {
            print!("{:04x} ", word);
            let inc = word
                .to_be_bytes()
                .iter()
                .filter(|byte| **byte != 0)
                .collect::<Vec<_>>()
                .len();

            offset += inc;
        });
        println!("");
    });
    println!("{:08x}", offset);
}

fn main() -> Result<(), String> {
    let args = std::env::args();
    let Cli { file_path, options }: Cli = Cli::try_from(args).map_err(|err| {
        show_help();
        err
    })?;

    let words = order_bytes(read_file_content(file_path)?, options.little_endian)?;
    print_words(words);
    Ok(())
}
