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
use crate::common::COLOURS;
use std::process::exit;
use spinners::{Spinner, Spinners::Dots};

/* TODO: add interactive mode
 * TODO: add Help menu
 * TODO: add dynamic language detection */

/* fn main */
fn main() {
    let ret = argparsing::parse_args();
    let (prompt, params, settings)  =  (ret.0, ret.1, ret.2);
    let mut sp = Spinner::new(Dots, format!("{}Waiting for response...", COLOURS.purple));
    match net::handle(prompt.as_str(), params, settings) {
        Ok(response) => {
            if params.shell {
                sp.stop_with_message(format!("{}{}", COLOURS.reset, highlight::highlight(&response, "sh")));
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
            exit(1)
        }
    }
}
