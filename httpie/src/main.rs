use clap::{Parser, Subcommand, Args};
use reqwest::Url;

fn main() {
    let ops: Opts = Opts::parse();
    println!("{:?}", ops);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    sub_cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        #[clap(parse(try_from_str = parse_url))]
        url: String,
    },
    Post(Post),
}

struct Get {
    url: String,
}

#[derive(Args, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

fn parse_url(s: &str) -> Result<String, dyn std::error::Error> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}