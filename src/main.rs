pub mod byte_ordering;
pub mod cli_parser;
pub mod file_operations;

use byte_ordering::order_bytes;
use cli_parser::Cli;
use file_operations::read_file_content;

fn show_help() {
    println!(
        "Usage:\n\
        rexout <file path> <options flags>\n\n\
        file path -> a valid path to a system file.\n\n\
        flags:\n\
            \t--big -> the program with print the words on big-endian style\n\
            \t--no-count -> don't display the byte count of the left\n"
    );
}

fn print_words(words: Vec<u16>, print_count: bool) {
    let mut offset = 0;
    let mut chunks = words.chunks(8);
    loop {
        if print_count {
            print!("{:08x} ", offset);
        }
        if let Some(chunk) = chunks.next() {
            chunk.into_iter().for_each(|word| {
                print!("{:04x} ", word);
                offset += if *word > 0b11111111 { 2 } else { 1 }
            });
            println!("");
            continue;
        }
        break;
    }
    println!("")
}

fn main() -> Result<(), String> {
    let args = std::env::args();
    let Cli { file_path, options }: Cli = Cli::try_from(args).map_err(|err| {
        show_help();
        err
    })?;

    let words = order_bytes(read_file_content(file_path)?, options.little_endian)?;
    print_words(words, options.count);
    Ok(())
}
