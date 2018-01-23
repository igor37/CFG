# CFG

CFG is a generator for phrases based on context-free grammars. Easy to use with calls
from the command line.

## Getting started

### Prerequisites

For compiling the project the Rust compiler and Cargo are required.  
See https://www.rust-lang.org/en-US/install.html for instructions.

### Running the example

After cloning/downloading the repository, simply switch into CFG's directory and compile with:

    cargo build --release

This generates an executable file in the *target/release* subdirectory. This program has to be run with a file containing the
rules of the context-free grammar. The repository already includes an example file for testing and getting familiar with the
syntax:

    target/release/cfg src/example_cfg -s

With the -s flag all phrases are output with an empty line inbetween as spacing.
If everything went right the output using the example file should look a bit like this:

    ...
    A sleepy white turtle dances around the sleepy raccoon

    The sleepy grey fox jumps around a crazy turtle

    A sleepy black raccoon jumps around the sleepy panda

    A quick grey dog jumps around the sleepy raccoon

    The sleepy white raccoon jumps over a lazy fox

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
