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
use crate::common::{COLOURS, ParseFloatType};
use crate::util::parse_float;
use crate::common::{Params, Settings};

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
            "-h" | "--help" => help(),
            "-e" | "--execute" => params.execute = true,
            "-i" | "--interactive" => params.interactive = true,
            "-c" | "--code" => {
                params.code = true;
                settings.temperature = 0.8;
                settings.top_p = 0.;
            },
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
    if prompt.is_empty() {
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
        "text-davinci-003" => {
            if temp_tokens >= 5 && temp_tokens <= 4000 {
                settings.max_tokens = temp_tokens
            } else {
                error_exit!("Tokens for model not between 5 and 2048")
            }
        },
        &_ => {}
    }

    (prompt, params, settings)
}

fn help() {
    print!("// TODO: Impl help()");
}
