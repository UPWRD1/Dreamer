# Getting Started

Welcome! This guide will help you install and run Dreamer.

## Installation

Dreamer is available as standalone binaries (see releases), a Rust crate, a Dreamer dream, or you can compile from source.

### Rust Crate
Assuming you have a working `cargo` installation, simply run
`cargo install Dreamer` in a terminal to install Dreamer.

### Compiling From Source

Clone the repo with git:
`git clone https://github.com/UPWRD1/Dreamer.git`

Or download the compressed source and extract.

Cd to the directory:
`cd Dreamer`

Finally, compile the executable with `cargo build --release`

For convinience, add the binary to your $PATH.

-----------------

## First Steps

The core of how Dreamer works is the *Dreamfile*. A Dreamfile is a simple `*.zzz.yaml` file containing all the information on how to execute dreams.

Thankfully, you don't have to write these! To create a new dreamfile, run

`$ zzz new myfile`

This will create a brand new file named `myfile.zzz.yaml` in your current directory.