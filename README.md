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