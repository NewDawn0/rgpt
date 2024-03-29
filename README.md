# RGPT
**An insane cli ChatGpt client written in Rust**</br>
Bring the Power of ChatGPT to the Command Line with RGPT - A Rust-Based CLI ChatGPT Client
Inspired by [shell_gpt](https://github.com/TheR1D/shell_gpt)

## Description
RGPT is a powerful CLI chatbot client that combines the speed of the command line with the flexibility of chatbots.
Built with the Rust programming language, RGPT offers faster performance, more reliable code, fewer dependency issues, easier maintenance, more secure code,
easier debugging, easier extension, easier optimization, and more options than its Python counterpart.
RGPT has a variety of configuration options that can be invoked using the command line.
These include interactive mode, a descriptive help menu, syntax highlighting in code snippets,
code mode, shell mode, and executable mode. Additionally, RGPT has the ability to configure the model (Davinci, Curie, Ada, or Babbage),
the max tokens, the temperature, and the accuracy. RGPT is available through the Cargo install command or via a Docker container.

## Installation
### Install binary
**Cargo**
```bash
cargo install --git https://github.com/NewDawn0/rgpt
```
**Install from source**
```bash
git clone https://github.com/NewDawn0/rgpt
cd rgpt
cargo build --release
sudo mv target/release/rgpt /usr/local/bin/
```
### Using rgpt in your project
To use rgpt in your project, run
```bash
cargo add --git https://github.com/NewDawn0/rgpt
```

## Usage
Too lazy to write documentation right now; Just run `rgpt --help`
