# Injection

This repository contains an example of how to inject data into an executable after it has already been compiled. It's intended to be used in a "bundler", a program that creates a self-contained executable (eg. an interpreter bundled with the program files and dependencies).

## Usage

1. Edit `template/src/main.rs` to handle the result of `injected_data()` however you like.
2. Change `INJECTED_DATA` in `src/main.rs` to the data you'd like to inject.
3. Run `build.sh` to compile the program, and then run `cargo run > out` to produce the output binary `out`.
4. Make `out` executable by running `chmod +x out`.
5. Run `out` and you should see your injected data printed to the screen!

## How it works

The template program contains a "magic string", which is just a UUID that will almost surely appear in the binary only once. When you run the injector program, it appends your data to the end of the binary, and then replaces the magic string with the data's size and location relative to the magic string. When the template program is executed, it calculates a pointer to the data based on the location of the magic string in the binary and then reads the pointer. The result is a `&'static [u8]`.

The reason a magic string is needed instead of just reading eg. the last _n_ bytes of the binary is because the binary may be allocated anywhere in memory, so the actual address of the injected data is nondeterministic. In this design, the program just adds the distance from the magic string to the injected data in the binary to the address of the magic string. You could probably work around this by figuring out the "base address" of the program and then adding the binary's size to that address, but I couldn't find an API to do this in Rust.

This design is inspired by [Warp](https://github.com/dgiagio/warp), but I wanted a more generic solution that I could easily use from Rust code instead of a command-line tool. It was also a cool learning opportunity!
