extern crate flate2;

use flate2::write::GzEncoder;
use flate2::read::GzDecoder; // Import GzDecoder for decompression
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::time::Instant;

fn main() {
    let args: Vec<String> = args().collect();

    // Check if the number of arguments is correct
    if args.len() != 3 {
        eprintln!("Usage: {} [-c|-d] <source>", args[0]);
        return;
    }

    // Get the operation type (compression or decompression)
    let operation = &args[1];
    let source_file = &args[2];

    // Choose compression or decompression based on the operation
    match operation.as_str() {
        "-c" => compress(source_file),
        "-d" => decompress(source_file),
        _ => {
            eprintln!("Invalid operation. Use -c for compression or -d for decompression.");
            return;
        }
    }
}

fn compress(source_file: &str) {
    let mut input = BufReader::new(File::open(source_file).unwrap());
    let target_file = format!("{}.gz", source_file);
    let output = File::create(&target_file).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    encoder.finish().unwrap();
    let end = Instant::now();

    println!("Compression completed successfully.");
    println!("Time elapsed: {:?}", end - start);
}

fn decompress(source_file: &str) {
    let input = BufReader::new(File::open(source_file).unwrap());
    let target_file = if source_file.ends_with(".gz") {
        source_file[..source_file.len() - 3].to_owned()
    } else {
        eprintln!("Invalid file format. Expected '.gz' extension.");
        return;
    };
    let output = File::create(&target_file).unwrap();
    let mut decoder = GzDecoder::new(input);

    let start = Instant::now();
    copy(&mut decoder, &mut BufWriter::new(output)).unwrap();
    let end = Instant::now();

    println!("Decompression completed successfully.");
    println!("Time elapsed: {:?}", end - start);
}
