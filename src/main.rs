#[macro_use]
extern crate quicli;
use quicli::prelude::*;
use std::{
    fs,
    io::{self, Read, Seek, Write},
    num, result,
};

fn try_parse_number(num_str: &str) -> result::Result<usize, num::ParseIntError> {
    // TODO: reimplement
    match &num_str[..=1] {
        "0x" | "0X" => {
            usize::from_str_radix(&num_str[2..], 16)
        },

        _ => {
            usize::from_str_radix(num_str, 10)
        }
    }
}

#[derive(StructOpt)]
#[structopt(name = "binex", about = "Extract bytes from binary file.")]
struct Cli {
    #[structopt(
        name = "byte_count",
        long = "count",
        short = "c",
        default_value = "0",
        parse(try_from_str = "try_parse_number"),
    )]
    count: usize,

    #[structopt(
        name = "byte_skip",
        long = "skip",
        short = "s",
        default_value = "0",
        parse(try_from_str = "try_parse_number"),
    )]
    skip: usize,

    #[structopt(name = "input_file")]
    input: String,

    #[structopt(name = "output_file", long = "out", short = "o")]
    output: String,
}

main!(|args: Cli| {
    println!(
        "Extract {} bytes at offset 0x{:x} from: \"{}\".",
        args.count, args.skip, &args.input
    );

    const BUFFER_SIZE: usize = 4 * 1024;

    let mut input_file = {
        let mut input_file = fs::File::open(&args.input)?;
        input_file.seek(io::SeekFrom::Start(args.skip as u64))?;
        let input_file = input_file.take(args.count as u64);
        io::BufReader::with_capacity(BUFFER_SIZE, input_file)
    };

    let mut output_data = Vec::with_capacity(BUFFER_SIZE);
    let num_read_bytes = input_file.read_to_end(&mut output_data)?;
    if num_read_bytes > 0 {
        let mut output_file = {
            let output_file = fs::File::create(&args.output)?;
            io::BufWriter::with_capacity(BUFFER_SIZE, output_file)
        };
        output_file.write_all(&output_data[..])?;
    }
    

    print!("{} bytes extracted", num_read_bytes);
    if num_read_bytes > 0 {
        println!(", saved to: \"{}\".", &args.output);
    } else {
        println!(".");
    }
});
