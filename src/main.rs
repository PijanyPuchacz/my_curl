use reqwest::{self};
use serde_json::json;
use structopt::StructOpt;
use strum_macros::{Display, EnumString};
use url::{self, Url};

#[derive(StructOpt, Debug)]
#[structopt(name = "options")]
struct Opt {
    /// Input URL
    #[structopt(parse(from_str))]
    url: String,

    /// Method type selection - GET, POST
    #[structopt(short = "-X", default_value = "GET")]
    method: MethodOpt,

    /// POST json
    #[structopt(long = "--json")]
    json: Option<String>,

    /// Debug mode
    #[structopt(long = "--debug")]
    debug: bool,

    /// key-value pairs - <key>=<value>&...
    #[structopt(short = "-d", required_if("method", "POST"))]
    key_val: Option<String>,
}

#[derive(StructOpt, Debug, EnumString, Display)]
enum MethodOpt {
    GET,
    POST,
}

fn main() {
    //process input args
    let mut opt = Opt::from_args();

    if opt.debug {
        println!("{:?}", opt);
    }

    //check for --json param
    let mut is_json = false;
    if opt.json.is_some() {
        opt.method = MethodOpt::POST;
        is_json = true;
    }

    println!("Requesting URL: {}\nMethod: {}", opt.url, opt.method);

    //determine method to use
    let body = match &opt.method {
        MethodOpt::GET => get_request(&opt.url), //return get body
        MethodOpt::POST => {
            //check if json exists
            let post_body = match opt.json {
                Some(json) => json,
                //else check for key_val
                None => match opt.key_val {
                    Some(key_val) => key_val,
                    None => panic!("Error, no <key>=<value>&... provided."),
                },
            };

            if opt.debug {
                println!("{:?}", json!(post_body));
            }

            post_requst(&opt.url, post_body, is_json) //return post body
        }
    };

    println!("{body}"); //print mesage
}

//Takes URL and returns response body if  successful
fn get_request(url: &String) -> String {
    match Url::parse(url) {
        Ok(_) => match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    response.text().unwrap()
                } else {
                    format!(
                        "Error: Request failed with status code: {}.",
                        response.status().as_u16()
                    )
                }
            }
            Err(e) => {
                if e.is_builder() {
                    "Error: The URL does not have a valid base protocol.".to_string()
                } else if e.is_request() {
                    "Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.".to_string()
                } else {
                    e.to_string()
                }
            }
        },
        Err(e) => match e {
            url::ParseError::RelativeUrlWithoutBase => {
                "Error: The URL does not have a valid base protocol.".to_string()
            }
            url::ParseError::InvalidIpv6Address => {
                "Error: The URL contains an invalid IPv6 address.".to_string()
            }
            url::ParseError::InvalidIpv4Address => {
                "Error: The URL contains an invalid IPv4 address.".to_string()
            }
            url::ParseError::InvalidPort => {
                "Error: The URL contains an invalid port number.".to_string()
            }
            _ => "URL Err".to_string(),
        },
    }
}

//Takes URL, Body, and a bool to ID the body as json and returns the response body if successful
fn post_requst(url: &String, body: String, json_bool: bool) -> String {
    match Url::parse(url) {
        Ok(_) => {
            let client = reqwest::blocking::Client::new();

            // if --json parameter passed
            if json_bool {
                //I keep getting 400 error here and I can't find out why
                match client.post(url).json(&body).send() {
                    //match client.post(url).body(json!(body).to_string()).send() {
                    Ok(response) => {
                        if response.status().is_success() {
                            response.text().unwrap()
                        } else {
                            format!(
                                "Error: Request failed with status code: {}.",
                                response.status().as_u16()
                            )
                        }
                    }
                    Err(e) => {
                        if e.is_builder() {
                            "Error: The URL does not have a valid base protocol.".to_string()
                        } else if e.is_request() {
                            "Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.".to_string()
                        } else {
                            e.to_string()
                        }
                    }
                }
            // else do -X POST
            } else {
                match client.post(url).body(body).send() {
                    Ok(response) => {
                        if response.status().is_success() {
                            response.text().unwrap()
                        } else {
                            format!(
                                "Error: Request failed with status code: {}.",
                                response.status().as_u16()
                            )
                        }
                    }
                    Err(e) => {
                        if e.is_builder() {
                            "Error: The URL does not have a valid base protocol.".to_string()
                        } else if e.is_request() {
                            "Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.".to_string()
                        } else {
                            e.to_string()
                        }
                    }
                }
            }
        }
        Err(e) => match e {
            url::ParseError::RelativeUrlWithoutBase => {
                "Error: The URL does not have a valid base protocol.".to_string()
            }
            url::ParseError::InvalidIpv6Address => {
                "Error: The URL contains an invalid IPv6 address.".to_string()
            }
            url::ParseError::InvalidIpv4Address => {
                "Error: The URL contains an invalid IPv4 address.".to_string()
            }
            url::ParseError::InvalidPort => {
                "Error: The URL contains an invalid port number.".to_string()
            }
            _ => "URL Err".to_string(),
        },
    }
}
