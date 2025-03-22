![Commit Activity](https://img.shields.io/github/commit-activity/m/ggkkaa/iff_rs)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/iff_rs)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/ggkkaa/iff_rust)



iff_rs is a simple IFF reader library made in Rust. It reads IFF files, skips the FORM at the beginning, and returns:
- The amount of chunks
- A vector with all of the chunks

This is an example on how to use iff_rs, the iff file in this example has 2 chunks.

- The FORM chunk, which is 8 bytes long, and
- The DATA chunk, which is 4 bytes long and contains the string 'abcd'

The data in the chunks is stored as a Vec<u8>

```rust
use iff_rs::parse_iff;
use std::string::String;

fn main() {
    let file: File = File::open("sample.iff").expect("Failed to open sample IFF file");
    let iff = parse_iff(file).expect("Failed to parse IFF file");
    let chunk = &iff.chunks[0];

    println!("{:?}", from_utf8(chunk.data));
}
```
This code will output 'abcd'.

## Installation
Add the following to your Cargo.toml file:
```toml
[dependencies]
iff_rs = "0.1.1"
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing
If you would like to contribute to this project, please fork the repository and create a pull request. If you have any questions, feel free to create an issue.

## Features
- [x] Read Basic IFF files
- [ ] Write to IFF files
- [ ] Detect IFF type

## Disclaimer

THIS LIBRARY IS VERY EARLY ALPHA!!! IT CANNOT WRITE OR DETECT THE TYPE OF IFF FILES AND ONLY GIVES DATA!!!