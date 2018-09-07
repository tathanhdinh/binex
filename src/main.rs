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
        "0x" | "0X" => usize::from_str_radix(&num_str[2..], 16),
        _ => usize::from_str_radix(num_str, 10),
    }
}

#[derive(StructOpt)]
#[structopt(name = "binex", about = "Extract bytes from binary file.")]
struct Cli {
    #[structopt(
        name = "byte_count",
        long = "count",
        short = "c",
        parse(try_from_str = "try_parse_number"),
        conflicts_with = "byte_stop",
        required_unless = "byte_stop",
    )]
    count: Option<usize>,

    #[structopt(
        name = "byte_skip",
        long = "skip",
        short = "k",
        default_value = "0",
        parse(try_from_str = "try_parse_number"),
    )]
    skip: usize,

    #[structopt(
        name = "byte_stop",
        long = "stop",
        short = "t",
        parse(try_from_str = "try_parse_number"),
        required_unless = "byte_count",
    )]
    stop: Option<usize>,

    #[structopt(name = "input_file")]
    input: String,

    #[structopt(name = "output_file", long = "out", short = "o")]
    output: String,
}

main!(|args: Cli| {
    let byte_count = {
        if let Some(byte_count) = args.count {
            byte_count
        } else {
            if let Some(byte_stop) = args.stop {
                if byte_stop < args.skip {
                    return Err(format_err!("number of extracted bytes is negative"));
                } else {
                    byte_stop - args.skip
                }
            } else {
                unreachable!()
            }
        }
    } as u64;

    println!(
        "Extract {} bytes at offset 0x{:x} from: \"{}\".",
        byte_count, args.skip, &args.input
    );

    const BUFFER_SIZE: usize = 4 * 1024;

    let mut input_file = {
        let skip_bytes = args.skip as u64;
        let filename = &args.input[..];
        
        let file_length = fs::metadata(filename)?.len();
        if skip_bytes > file_length {
            return Err(format_err!("number of skipped bytes is too large"))
        }

        let mut input_file = fs::File::open(filename)?;
        input_file.seek(io::SeekFrom::Start(skip_bytes))?;
        let input_file = input_file.take(byte_count);

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
