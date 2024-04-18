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
        note = {
          url = "github:NewDawn0/rgpt";
          inputs.nixpkgs.follows = "nixpkgs";
          # Optional: If you use nix-systems or rust-overlay
          inputs.nix-systems.follows = "nix-systems";
          inputs.rust-overlay.follows = "rust-overlay";
        };
      };
    }
    ```
2. Add this to your overlays to expose note to your pkgs
    ```nix
    (final: prev: {
      note = inputs.rgpt.packages.${prev.system}.default;
    })
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
To use rgpt in your project, run
```bash
cargo add --git https://github.com/NewDawn0/rgpt
```

## Usage
```bash
# Simple usage
rgpt <your prompt>

# Use rgpt --help for the list of options
# output of rgpt --help
#                  _
#  _ __ __ _ _ __ | |_   An insane cli ChatGPT client
# | '__/ _` | '_ \| __|  Author: NewDawn0
# | | | (_| | |_) | |_   License: MIT
# |_|  \__, | .__/ \__|  Copyright: ©NewDawn0 2023
#      |___/|_|  https://github.com/NewDawn0/rgpt
# 
# OPTIONS
#     -h                          Print this help menu
#     --help                      Print this help menu
# 
#     -c                          Return code as answer only           Incompatible with: --no-fmt
#     --code                      Return code as answer only           Incompatible with: --no-fmt
# 
#     -s                          Returns a shell command
#     --shell                     Returns a shell command
#     -e                          Executes the shell command           Depends on: --shell
#     --execute                   Executes the shell command           Depends on: --shell
# 
#     -i                          Starts interactive mode
#     --interactive               Starts interactive mode
#     --no-parse                  Sets if arguments need to be parsed  Depends on: --interactive
# 
#     -r                          Roasts person
#     --roast                     Roasts person
# 
#     --no-timout                 Disables request timed out message
#     --no-spinner                Disables the waiting spinner
#     --no-fmt                    Disables answer string formatting    Incompatible with: --code   Depends on: --no-spinner
# 
#     -k                          By using this flag, you can specify an API key which will override the environment variable
#     --key                       By using this flag, you can specify an API key which will override the environment variable
# 
#     --config   <key=value>      Configures gpt itself:
#                model=<String>       davinci|ada|curie|babbage        Defalt: davinci
#                maxTokens=<int>      ada|curie|babbage: 5 - 2048      Defalt: 1024
#                                     davinci: 4000
#                temperature=<float>  0 - 2                            Defalt: 0.2
#                accuracy=<float>     0 - 1                            Defalt: 0.9
# 
# DESCRIPTION
#     Placeholder
#     For more info about the tool or its options visit the repo ttps://github.com/NewDawn0/rgpt
# 
# EXAMPLE
#     $ rgpt --shell Remove files containing tmp in filename
#     rm *tmp*
```
