/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: net.rs
 *  desc: Some networking
 *  date: 22.02.2023
 *  lang: rust
*/

/* Imports */
use std::error::Error;
use regex::Regex;
use serde_json::json;
use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use crate::common;
use crate::common::RgptError;
use crate::util;

/* fn do_request: send blocking web request 
 * @PARAM prompt: String
 * @PARAM settings: struct Settings
 * @PARAM params: struct Params
 * @RVAL: Result<String, String> */
fn do_request(prompt: &str, params: common::Params, settings: common::Settings) -> Result<String, Box<dyn Error>> {
    let mut prompt = prompt.to_string();
    if params.code {
        prompt.push_str(". Provide only code as output.")
    } else if params.shell {
        prompt.push_str(". Provide only a single line bash command as output.")
    } else if params.roast {
        prompt = format!("Formulate some roasts against {}", prompt);
    }
    let client = match params.no_timout {
        true => Client::builder().timeout(None).build()?,
        false => Client::new()
    };
    let url = "https://api.openai.com/v1/completions";
    match util::get_env("OPENAI_API_KEY"){
        Ok(key) => {
            let res = client.post(url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", key))
                .json(&json!({
                    "model": settings.model,
                    "prompt": prompt,
                    "temperature": settings.temperature,
                    "max_tokens": settings.max_tokens,
                    "top_p": settings.top_p,
                }))
            .send()?;
            Ok(res.text()?)
        },
        Err(e) => Err(Box::new(e))
    }
}

/* fn do_parse_response: Parses the json response
 * @PARAM json_str: &str
 * @RVAL: Option<String>*/
fn do_parse_response(json_str: &str) -> Option<String> {
    // Parse response
    let response: Response = serde_json::from_str(json_str).ok()?;
    let choice = response.choices.into_iter().next()?;
    let re = Regex::new(r"^\n\n").unwrap();
    let res = re.replace_all(&choice.text, "");
    Some(res.to_string())
}

/* fn handle: Does the request and response
 * @PARAM prompt: &str
 * @PARAM params: crate::common::Params
 * @PARAM settings: crate::common::Settings
 * @RVAL: String */
pub fn handle(prompt: &str, params: common::Params, settings: common::Settings) -> Result<String, Box<dyn Error>> {
    match do_request(prompt, params, settings) {
        Ok(response) => {
            match do_parse_response(&response.as_str()) {
                Some(res) => Ok(res),
                None => Err(Box::new(RgptError::new(common::ErrorType::NetParsingErr(response.clone()))))
            }
        },
        Err(e) => Err(e),
    }
}

/* fn parse_error: Parse the error json for the error message
 * @PARAM json_str: &str
 * @RVAL: Option<String> */
pub fn parse_error(json_str: &str) -> Option<String> {
    let error_response: ErrorResponse = serde_json::from_str(json_str).ok()?;
    Some(error_response.error.message)
}

/* Helper stucts for response parsing */
#[derive(Serialize, Deserialize)]
struct Choice {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

/* Helper structs for the error parsing */
#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: ParsingError,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParsingError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    param: Option<String>,
    code: Option<String>,
}
