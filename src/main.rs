use std::io::{self, Write};
use template::MAGIC_STRING;

#[cfg(debug_assertions)]
static PRINTER_PROGRAM_DATA: &[u8] = include_bytes!("../target/debug/template");
#[cfg(not(debug_assertions))]
static PRINTER_PROGRAM_DATA: &[u8] = include_bytes!("../target/release/template");

const INJECTED_DATA: &[u8] = b"Hello, world!";

fn main() {
    let mut program = PRINTER_PROGRAM_DATA.to_vec();
    inject(INJECTED_DATA.to_vec(), &mut program);
    io::stdout().write_all(&program).unwrap();
}

fn inject(mut injected_data: Vec<u8>, program: &mut Vec<u8>) {
    // Find the location of the magic string
    let magic_string_offset =
        find_subsequence(&program, MAGIC_STRING).expect("Could not find magic string in binary");

    // We have to use a relative distance here because the program's base
    // address is nondeterministic -- the printer will calculate the actual
    // pointer to the data by adding this distance to the pointer to the magic
    // string
    let distance_to_injected_data_bytes = (program.len() - magic_string_offset).to_ne_bytes();

    // Add the injected data to the end of the binary
    let injected_data_len_bytes = injected_data.len().to_ne_bytes();
    program.append(&mut injected_data);

    // Replace the magic string with the size of the injected data and the
    // distance from the magic string to the injected data in the binary
    program.splice(
        magic_string_offset..(magic_string_offset + MAGIC_STRING.len()),
        injected_data_len_bytes
            .iter()
            .chain(distance_to_injected_data_bytes.iter())
            .cloned()
            // zero out the rest of the magic string
            .chain(std::iter::repeat(0).take(
                MAGIC_STRING.len()
                    - injected_data_len_bytes.len()
                    - distance_to_injected_data_bytes.len(),
            )),
    );
}

/// https://stackoverflow.com/a/35907071/5569234
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
