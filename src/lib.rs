use base64::{engine::general_purpose, Engine};
use std::error::Error;

pub enum EncodingAlgorithm {
    Base64,
    Base64Url,
    Hex,
    Base58,
}

pub fn decode_input(input: &str, algorithm: EncodingAlgorithm) -> Result<String, Box<dyn Error>> {
    let decoded: Vec<u8> = match algorithm {
        EncodingAlgorithm::Base64 => general_purpose::STANDARD.decode(input)?,
        EncodingAlgorithm::Base64Url => base64_url::decode(input)?,
        EncodingAlgorithm::Hex => hex::decode(input)?,
        EncodingAlgorithm::Base58 => bs58::decode(input).into_vec()?,
    };
    Ok(String::from_utf8_lossy(&decoded).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_input_base64() {
        let input = "SGVsbG8gV29ybGQ=";
        let algorithm = EncodingAlgorithm::Base64;
        let result = decode_input(input, algorithm).unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_decode_input_base64url() {
        let input = "SGVsbG8gV29ybGQ";
        let algorithm = EncodingAlgorithm::Base64Url;
        let result = decode_input(input, algorithm).unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_decode_input_hex() {
        let input = "48656c6c6f20576f726c64";
        let algorithm = EncodingAlgorithm::Hex;
        let result = decode_input(input, algorithm).unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_decode_input_base58() {
        let input = "JxF12TrwUP45BMd";
        let algorithm = EncodingAlgorithm::Base58;
        let result = decode_input(input, algorithm).unwrap();
        assert_eq!(result, "Hello World");
    }
}
