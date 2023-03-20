/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: main.rs
 *  desc: Main file
 *  date: 22.02.2023
 *  lang: rust
 *
 *
 * MIT License
 *
 * Copyright (c) 2023 NewDawn0
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the follwing conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
*/

/* Modules & Imports */
mod io;
mod util;
mod common;
mod net;
mod argparsing;
mod highlight;
use crate::common::*;
use spinners::{Spinner, Spinners::Dots};
use std::{
    process::exit,
    io::{stdout, Write}
};

/* TODO: add Help menu
 * TODO: add dynamic language detection */

/* fn main */
fn main() {
    let mut stdout = stdout();
    let ret = argparsing::parse_args();
    let (mut prompt, default_params, default_settings)  =  (ret.0, ret.1, ret.2);
    let (mut params, mut settings) = (default_params.clone(), default_settings.clone());
    let mut history = Vec::<String>::new();
    loop {
        if prompt.is_empty() {
            print!("{}>{} ", COLOURS.red, COLOURS.reset);
            stdout.flush().expect("Could not flush stdout");
            // checks if argmuments need to be parsed when read from stdin
            if params.no_parse {
                prompt = io::read_stdin(&mut history);
            } else {
                prompt = io::parse_io(&mut params, &mut settings, &mut history);
            }
        }
        // Ctrl-c or Ctrl-q recieved from interactive
        if prompt.is_empty() {
            println!("{:?}", history);
            exit(0);
        }
        query(prompt.clone(), &params, settings.clone(), history.clone());
        if !params.interactive {
            break;
        }
        // reset prompt args and settigns
        params = default_params.clone();
        params.interactive = true;
        settings = default_settings.clone();
        // reset prmpt
        prompt.clear();
    }
}

/* fn query: runs all the required functions to query chat gpt
 * @PARAM prompt: String
 * @PARAM params: crate::common::Params
 * @PARAM settings: crate::common::Settings */
fn query(prompt: String, params: &Params, settings: Settings, history: Vec<String>) {
    let mut sp: Option<Spinner> = None;
    if params.spinner {
        sp = Some(Spinner::new(Dots, format!("{}Waiting for response...", COLOURS.purple)));
    }
    match net::handle(prompt.as_str(), params.clone(), settings) {
        Ok(response) => {
            if params.shell {
                match sp {
                    Some(mut sp) => sp.stop_with_message(format!("{}{}", COLOURS.reset, highlight::highlight(&response, "bash"))),
                    None => println!("{}{}", COLOURS.reset, highlight::highlight(&response, "bash"))
                }
                if params.execute {
                    if io::confirm() {
                        util::run_command(&response)
                    } 
                }
            } else if params.code {
                match sp {
                    Some(mut sp) => sp.stop_with_message(format!("{}{}", COLOURS.reset, highlight::highlight(&response, "rs"))),
                    None => println!("{}{}", COLOURS.reset, highlight::highlight(&response, "rs"))
                }
            } else {
                match sp {
                    Some(mut sp) => sp.stop_with_message(format!("{}{}", COLOURS.reset, response)),
                    None => println!("{}{}", COLOURS.reset, response)
                }
            }
        }
        Err(e) => {
            match sp {
                Some(mut sp) => sp.stop_with_message(format!("{}{}Error{} :: {}", COLOURS.reset, COLOURS.red, COLOURS.reset, e)),
                None => eprintln!("{}{}Error{} :: {}", COLOURS.reset, COLOURS.red, COLOURS.reset, e)
            }
            if !params.interactive {
                exit(1)
            }
        }
    }
}
