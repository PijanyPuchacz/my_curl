use std::path::PathBuf;
use structopt::StructOpt;
use url::{ParseError, Url};

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

    let url = Url::parse(&opt.url);

    let body = reqwest::get(url)
}
