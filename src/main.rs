use std::{env, process};

use local_decoder::{decode_input, EncodingAlgorithm};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <encoding_algorithm> <input>", args[0]);
        process::exit(1);
    }

    let algorithm = match args[1].as_str() {
        "base64" => EncodingAlgorithm::Base64,
        "base64url" => EncodingAlgorithm::Base64Url,
        "hex" => EncodingAlgorithm::Hex,
        "base58" => EncodingAlgorithm::Base58,
        _ => {
            eprintln!("Unsupported encoding: {}", args[1]);
            process::exit(1);
        }
    };

    let input = &args[2];

    match decode_input(input, algorithm) {
        Ok(decoded) => println!("\nInput:\n {}\n\nDecoded:\n {}", input, decoded),
        Err(err) => {
            eprintln!("Error decoding input: {}", err);
            process::exit(1);
        }
    }
}
