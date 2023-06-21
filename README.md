# Local Decoder CLI Tool

The Local Decoder CLI tool is a Rust-based command-line utility that allows you to decode encoded strings using different encoding algorithms. You can install it locally on your machine and use it to decode strings in Base64, Base64URL, Hex, and Base58 formats.

## Installation

To install the Local Decoder CLI tool locally on your machine, follow these steps:

- Ensure you have Rust and Cargo installed. You can check by running the following commands in your terminal:

```
rustc --version
cargo --version
```

- Clone the repository or download the source code to your local machine.

- Open your terminal and navigate to the project directory.
  Run the following command to build the project:

```
cd local-decoder && cargo install --path .
```

This will compile the code and create an executable binary in your path.
With the Local Decoder CLI tool installed, you're ready to start using it!
<br/>

## Usage

To use the Local Decoder CLI tool, follow these steps:

Open your terminal.

Run the following command to decode an encoded string:

```
local-decoder <encoding_algorithm> <input>
```

Replace `<encoding_algorithm>` with one of the supported algorithms: base64, base64url, hex, or base58.

Replace `<input>` with the encoded string you want to decode.

Example:

```
local-decoder base64 SGVsbG8gV29ybGQ=
```

This command will decode the provided Base64-encoded string and display the decoded output.

The CLI tool will print the original input and the decoded output to the terminal.

Example output:

```
Input:
SGVsbG8gV29ybGQ=

Decoded:
Hello World
```

You can use the Local Decoder CLI tool to decode strings with different encoding algorithms by specifying the corresponding `<encoding_algorithm>` and `<input>` values in the command.
