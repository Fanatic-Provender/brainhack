# `brainhack`

Authors: Altan Mehmet Ünver and Shiqiao Zhang

## Overview

The `brainhack` assembler compiles Hack assembly as defined in the [Nand2Tetris] project into the [brainfuck] programming language, partially to demonstrate the Turing completeness of brainfuck.  The `brainhack` optimizing interpreter executes brainfuck code, rendering graphics and receiving keyboard input according to the specification of the Hack assembly language.

## Installing Git

The `brainhack` repository is managed using Git.
Please run the command
```
git -v
```
to make sure that you have Git installed.
Otherwise, please refer to the [Git] website
for instructions on installing Git.

## Installing Rust

`brainhack` is implemented in the [Rust] programming language.  We recommended installing Rust via the official installer [`rustup`].  If you already have Rust installed, please run the command
```
cargo -V
```
to check if your installed version is at least `1.66.1`.  If this is not the case, please run the command
```
rustup update
```
to update your Rust installation.

## Installing SDL2

`brainhack` uses the SDL2 library for screen rendering and keyboard interaction.
If you are on macOS, please use
```
brew install sdl2
```
to install the SDL2 library.
Otherwise, please refer to the [Rust-SDL2] project
for instructions on installing the SDL2 library.

## Installing `brainhack`

Clone the source of `brainhack` with Git:
```
git clone https://github.com/Fanatic-Provender/brainhack.git
cd brainhack
```

## Build `brainhack`

Use Cargo to build `brainhack`:
```
cargo build --release
```

## Using `brainhack`

**Assembler**: in the `brainhack` directory, run the command
```
cargo run --release --bin assembler <FILE>
```
to convert the assembly file `<FILE>` to brainfuck code.
`<FILE>` should have the extension `asm`,
and `brainhack` will generate an output file with the extension `bf`.
The output file will rewrite any existing file with the same name.

**Interpreter**: in the `brainhack` directory, run the command
```
cargo run --release --bin interpreter <FILE>
```
to execute the brainfuck program `<FILE>`.
`<FILE>` should have the extension `bf`.

Alternatively, the `assembler` and `interpreter` executables
can be found in the directory `./target/release`
after building `brainhack` in release mode.


[Git]: https://git-scm.com/
[nand2tetris]: https://www.nand2tetris.org/
[Rust]: https://www.rust-lang.org/
[Rust-SDL2]: https://github.com/Rust-SDL2/rust-sdl2
[`rustup`]: https://rustup.rs/
