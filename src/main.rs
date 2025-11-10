use reqwest::{self};
use structopt::StructOpt;
use url::{self, Url};

#[derive(StructOpt, Debug)]
#[structopt(name = "options")]
struct Opt {
    /// input URL
    #[structopt(parse(from_str))]
    url: String,

    // undecided if I will need it
    /// debug mode
    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.debug {
        println!("{:?}", opt);
    }

    println!("Requesting URL: {}\nMethod: GET", opt.url);

    let body = match Url::parse(&opt.url) {
        Ok(_) => match get_request(&opt.url) {
            Ok(text) => format!("Response body:\n{text}"),
            Err(_) => "Error: The URL does not have a valid base protocol.".to_string(),
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
    };

    println!("{body}");
}

fn get_request(url: &String) -> Result<String, reqwest::Error> {
    let body = reqwest::blocking::get(url)?.text();
    body
}
