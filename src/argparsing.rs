/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: argparse.rs
 *  desc: Agument parsing functions
 *  date: 22.02.2023
 *  lang: rust
*/

/* Imports */
use std::{env, process::exit};
use crate::error_exit;
use crate::common::*;
use crate::util::{parse_float, set_mode};

/* fn parse_args: Parsing command line args
 * @RVAL prmpt: String
 * @RVAL params: crate::common::Params
 * @RVAL settigns: crate::common::Settings */
pub fn parse_args() -> (String, Params, Settings) {
    /* vars */
    let args: Vec<String> = env::args().skip(1).collect();
    let mut settings = Settings::new(); // store the setings
    let mut temp_tokens = settings.max_tokens; // storing the max tokens until the model is set
    let mut prompt = String::new(); // prompt variable
    let mut params = Params::new(); // param variable
    let mut index: usize = 0; // index for looping over the args

    /* looping over args */
    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {
            "-h" | "--help" => {help(); exit(0)},
            "-e" | "--execute" => params.execute = true,
            "-i" | "--interactive" => params.interactive = true,
            "--no-parse" => params.no_parse = true,
            "--no-spinner" => params.spinner = false,
            "--no-timeout" => params.no_timout = true,
            "-c" | "--code" => set_mode(Modes::Code, &mut params, &mut settings),
            "-r" | "--roast" => set_mode(Modes::Roast, &mut params, &mut settings),
            "-s" | "--shell" => set_mode(Modes::Shell, &mut params, &mut settings),
            "--no-fmt" => params.fmt = false,
            "-k" | "--key" => {
                if index+1 < args.len() {
                    params.key = Some(args[index+1].clone());
                } else {
                    error_exit!("Provide a key");
                }
            },
            "--history" => params.history = true,
            "--config" => {
                if index+1 < args.len() {
                    let opts: Vec<&str> = args[index+1].split("=").collect();
                    if opts.len() != 2 {
                        error_exit!("Config option '{}' is erroneously formatted", args[index+1]);
                    }
                    match opts[0] {
                        "model" => {
                            match opts[1] {
                                "davinci" => settings.model = String::from("text-davinci-003"),
                                "ada" => settings.model = String::from("text-ada-001"),
                                "curie" => settings.model = String::from("text-curie-001"),
                                "babbage" => settings.model = String::from("text-babbage-001"),
                                // "gpt-3.5-turbo" => settings.model = String::from("gpt-3.5-turbo"),
                                model => error_exit!("Invalid model '{}'", model)
                            }
                        },
                        "temperature" => {
                            match parse_float(opts[1], 2.0, ParseFloatType::Temperature) {
                                Some(val) => settings.temperature = val,
                                None => {},
                            }
                        },
                        "maxTokens" => {
                            match opts[1].parse::<i32>() {
                                Ok(val) => temp_tokens = val,
                                Err(_) => error_exit!("Value must be between 0 and 10")
                            }
                        },
                        "accuracy" => {
                            match parse_float(opts[1], 1.0, ParseFloatType::Accuracy) {
                                Some(val) => settings.top_p = val,
                                None => {},
                            }
                        },
                        _ => error_exit!("Invalid config option {}", &args[index+1])
                    }
                    index += 1;
                } else {
                    error_exit!("Provide a config option");
                }
            }
            val => prompt.push_str(format!("{} ", val).as_str())
        }
        index += 1
    }
    /* Cleanup prompt */
    prompt = prompt.trim().to_string();
    if prompt.is_empty() && !params.interactive {
        error_exit!("Provide a prompt")
    } else {} // else statement to make the rust compiler happy
    
    /* set model tokens */
    match settings.model.as_str() {
        "text-ada-001" | "text-curie-001" | "text-babbage-001" => {
            if temp_tokens >= 5 && temp_tokens <= 2048 {
                settings.max_tokens = temp_tokens
            } else {
                error_exit!("Tokens for model not between 5 and 2048")
            }
        },
        "text-davinci-003" => { // TODO: add gpt-3.5-turbo
            if temp_tokens >= 5 && temp_tokens <= 4096 {
                settings.max_tokens = temp_tokens
            } else {
                error_exit!("Tokens for model not between 5 and 2048")
            }
        },
        &_ => {} // Impossible
    }

    (prompt, params, settings)
}

fn help() {
    /* Arguemtn lines speretaed like the printed lines */
    println!("{}                 _
 _ __ __ _ _ __ | |_   An insane cli ChatGPT client
| '__/ _` | '_ \\| __|  {}{}Author: NewDawn0
{}| | | (_| | |_) | |_   {}{}License: MIT
{}|_|  \\__, | .__/ \\__|  {}{}Copyright: ©NewDawn0 2023
{}     |___/|_|  {}{}https://github.com/NewDawn0/rgpt

{}OPTIONS
    {}{}-h                          {}Print this help menu
    {}{}--help                      {}Print this help menu

    {}{}-c                          {}Return code as answer only           {}Incompatible with: {}--no-fmt
    {}{}--code                      {}Return code as answer only           {}Incompatible with: {}--no-fmt

    {}{}-s                          {}Returns a shell command
    {}{}--shell                     {}Returns a shell command
    {}{}-e                          {}Executes the shell command           {}Depends on: {}--shell
    {}{}--execute                   {}Executes the shell command           {}Depends on: {}--shell

    {}{}-i                          {}Starts interactive mode
    {}{}--interactive               {}Starts interactive mode
    {}{}--no-parse                  {}Sets if arguments need to be parsed  {}Depends on: {}--interactive

    {}{}-r                          {}Roasts person
    {}{}--roast                     {}Roasts person

    {}{}--no-timout                 {}Disables request timed out message
    {}{}--no-spinner                {}Disables the waiting spinner
    {}{}--no-fmt                    {}Disables answer string formatting    {}Incompatible with: {}--code   {}Depends on: {}--no-spinner

    {}{}-k                          {}By using this flag, you can specify an API key which will override the environment variable
    {}{}--key                       {}By using this flag, you can specify an API key which will override the environment variable

    {}{}--config   {}<{}key{}={}value{}>      Configures gpt itself:
               {}model{}=<{}String{}>       {}davinci{}|{}ada{}|{}curie{}|{}babbage        {}Defalt:{} davinci
               {}maxTokens{}=<{}int{}>      ada|curie|babbage: {}5{} - {}2048      {}Defalt:{} 1024
                                    davinci: {}4000
               {}temperature{}=<{}float{}>  {}0{} - {}2                            {}Defalt:{} 0.2
               {}accuracy{}=<{}float{}>     {}0{} - {}1                            {}Defalt:{} 0.9

{}DESCRIPTION
    {}{}Placeholder
    For more info about the tool or its options visit the repo ttps://github.com/NewDawn0/rgpt

{}EXAMPLE
    {}{}$ {}rgpt {}{}--shell {}Remove files containing tmp in filename
    {}rm *tmp*
",
COLOURS.bold_purple,
COLOURS.reset, COLOURS.purple,
COLOURS.bold_purple, COLOURS.reset, COLOURS.purple,
COLOURS.bold_purple, COLOURS.reset, COLOURS.purple,
COLOURS.bold_purple, COLOURS.reset, COLOURS.purple,
COLOURS.bold_purple,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset, COLOURS.blue, COLOURS.cyan,
COLOURS.reset, COLOURS.cyan, COLOURS.reset, COLOURS.blue, COLOURS.cyan,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset, COLOURS.blue, COLOURS.cyan,
COLOURS.reset, COLOURS.cyan, COLOURS.reset, COLOURS.blue, COLOURS.cyan,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,   
COLOURS.reset, COLOURS.cyan, COLOURS.reset, COLOURS.blue, COLOURS.cyan,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset, COLOURS.blue, COLOURS.cyan, COLOURS.blue, COLOURS.cyan,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset, COLOURS.cyan, COLOURS.reset, COLOURS.red, COLOURS.reset, COLOURS.bold_purple, COLOURS.reset,
COLOURS.red, COLOURS.reset, COLOURS.bold_yellow, COLOURS.reset, COLOURS.purple, COLOURS.reset,COLOURS.purple, COLOURS.reset,COLOURS.purple, COLOURS.reset, COLOURS.purple, COLOURS.blue, COLOURS.reset,
COLOURS.red, COLOURS.reset, COLOURS.bold_yellow, COLOURS.reset, COLOURS.purple, COLOURS.reset, COLOURS.purple, COLOURS.blue, COLOURS.reset,
COLOURS.purple,
COLOURS.red, COLOURS.reset, COLOURS.bold_yellow, COLOURS.reset, COLOURS.purple, COLOURS.reset, COLOURS.purple, COLOURS.blue, COLOURS.reset,
COLOURS.red, COLOURS.reset, COLOURS.bold_yellow, COLOURS.reset, COLOURS.purple, COLOURS.reset, COLOURS.purple, COLOURS.blue, COLOURS.reset,
COLOURS.bold_purple,
COLOURS.reset, COLOURS.purple,
COLOURS.bold_purple,
COLOURS.reset, COLOURS.red, COLOURS.bold_purple, COLOURS.reset, COLOURS.cyan, COLOURS.reset,
COLOURS.reset);
}
