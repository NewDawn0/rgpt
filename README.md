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

### Install using Cargo

```bash
cargo install --git https://github.com/NewDawn0/rgpt
```

### Install using Nix

#### Imperatively

```bash
git clone https://github.com/NewDawn0/rgpt
nix profile install .
```

#### Declaratively

1. Add it as an input to your system flake as follows
   ```nix
   {
     inputs = {
       # Your other inputs ...
       rgpt = {
         url = "github:NewDawn0/rgpt";
         inputs.nixpkgs.follows = "nixpkgs";
         # Optional: If you use nix-systems or rust-overlay
         inputs.nix-systems.follows = "nix-systems";
         inputs.rust-overlay.follows = "rust-overlay";
       };
     };
   }
   ```
2. Add the overlay to expose rgpt to your pkgs

   ```nix
   overlays = [ inputs.rgpt.overlays.default ];
   ```

3. Then you can either install it in your `environment.systemPackages` using
   ```nix
   environment.systemPackages = with pkgs; [ rgpt ];
   ```
   or install it to your `home.packages`
   ```nix
   home.packages = with pkgs; [ rgpt ];
   ```

### Using rgpt in your project

To use rgpt in your project, run:

```bash
cargo add --git https://github.com/NewDawn0/rgpt
```

Then use `cargo doc --open` to open the documentation

## Usage

```bash
# Simple usage
rgpt <your prompt>

# Use rgpt --help for a more options
```
