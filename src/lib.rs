/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: main.rs
 *  desc: Main file (bin)
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

//! An insane Rust ChatGPT client
//!
//! This library provides a function to query ChatGPT

/* Crate level macros */
#![allow(unused)]

/* Modules & Imports */
pub mod common;
mod net;
mod util;
use std::error::Error;
use common::{Params, Settings};
use net::handle;

/// This function sends a prompt to ChatGPT,
/// using the specified model settings and parameters, 
/// and returns the response as a String if successful. If unsuccessful, it returns a Box of dyn Error
/// 
/// # Arguments
///
/// * `prompt` - A promt of type std::string::String which is asked to ChatGPT
/// * `params` - Parameters to configure the prompt
/// * `settings` - Settings to configure the model
///
/// #
/// # Example
/// ```rust
/// use rgpt::{query, common::*};
/// 
/// let prompt = String::from("Write a hello world program");
/// let mut params = Params::new();
/// params.code = true;
/// let settings = Settings::new();
/// match query(prompt, params, settings) {
///     Ok(e) => println!("Got resp: {}", e),
///     Err(e) => println!("Err: {}", e),
/// }
/// ```
pub fn query(prompt: String, params: Params, settings: Settings) -> Result<String, Box<dyn Error>> {
    match handle(prompt.as_str(), params, settings) {
        Ok(response) => Ok(response),
        Err(e) => Err(e)
    }
}
