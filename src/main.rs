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
use std::{process::exit, io::Write};
use spinners::{Spinner, Spinners::Dots};

/* TODO: add Help menu
 * TODO: add dynamic language detection */

/* fn main */
fn main() {
    let ret = argparsing::parse_args();
    let (mut prompt, params, settings)  =  (ret.0, ret.1, ret.2);
    loop {
        if prompt.is_empty() {
            print!("{}>{} ", COLOURS.red, COLOURS.reset);
            std::io::stdout().flush().expect("Could not flush stdout");
            prompt = io::read_stdin();
        }
        query(prompt.clone(), params, settings.clone());
        if !params.interactive {
            break;
        }
        prompt.clear();
    }
}

/* fn query: runs all the required functions to query chat gpt
 * @PARAM prompt: String
 * @PARAM params: crate::common::Params
 * @PARAM settings: crate::common::Settings */
fn query(prompt: String, params: Params, settings: Settings) {
    // Ctrl-c or Ctrl-q recieved from interactive
    if prompt.is_empty() {
        exit(0);
    }
    let mut sp = Spinner::new(Dots, format!("{}Waiting for response...", COLOURS.purple));
    match net::handle(prompt.as_str(), params, settings) {
        Ok(response) => {
            if params.shell {
                sp.stop_with_message(format!("{}{}", COLOURS.reset, highlight::highlight(&response, "bash")));
                if params.execute {
                    if io::confirm() {
                        util::run_command(&response)
                    } 
                }
            } else if params.code {
                sp.stop_with_message(format!("{}{}", COLOURS.reset, highlight::highlight(&response, "rs")))
            } else {
                sp.stop_with_message(format!("{}{}", COLOURS.reset, response))
            }
        }
        Err(e) => {
            sp.stop_with_message(format!("{}{}Error{} :: {}", COLOURS.reset, COLOURS.red, COLOURS.reset, e));
            if !params.interactive {
                exit(1)
            }
        }
}
}
