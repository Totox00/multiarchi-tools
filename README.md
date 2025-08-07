# Multiarchi Tools

Tools made to ease management of multiarchi-style asyncs.

# Installation

The tools are written in Rust, and prebuilt binaries are currently not available for any operating systems. As such, it will need to be compiled by the user.
To compile rust, you will need the rust toolchain, which can be installed using [rustup](https://rustup.rs/#). Simply follow the instructions to install it.
Additionally, you will need to clone this repository to a directory of your choice.
After it is installed, you can invoke `cargo` to build the tools using `cargo build --release` in the same directory as this file, this will generate an executable file for your operating system at `./target/release/multiarchi<extension>`.

# Usage

To use the tool, simply call the executable with the current working directory set to a directory with the following:
- A `process.tsv` file containing the bucket files to process and the names they will be set to. This file is formatted as two columns with the name in the first column and the id of the bucket file in the second.
- A `bucket` directory containing all yamls that can be used, named `bucket (<id>).yaml`.
- A `dist` directory that will contain the resultining yamls.

The tool will generate an `output.tsv` file containing the names, games, and notes of processed yamls, as well as write relevant warnings to the terminal.
