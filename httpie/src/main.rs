use std::collections::HashMap;
use std::str::FromStr;
use anyhow::anyhow;
use clap::{Parser, Subcommand, Args};
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, header, Response, Url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops: Opts = Opts::parse();
    let client = reqwest::Client::builder().build()?;

    let result = match ops.sub_cmd {
        Commands::Get(ref args) => get(client, args).await?,
        Commands::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<(), Box<dyn std::error::Error>> {
    let resp = client.get(&args.url).send().await?;

    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<(), Box<dyn std::error::Error>> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;

    Ok(print_resp(resp).await?)
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    sub_cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get(Get),
    Post(Post),
}

#[derive(Args, Debug)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Args, Debug)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv)]
    body: Vec<KvPair>,
}

fn parse_url(s: &str) -> Result<String, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Debug)]
#[derive(Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("failed to parse {}",s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv(s: &str) -> Result<KvPair, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    Ok(s.parse()?)
}

async fn print_resp(resp: Response) -> Result<(), Box<dyn std::error::Error>> {
    print_status(&resp);
    print_headers(&resp);

    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!("\n")
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body)
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).map(|v| (v.to_str().unwrap().parse().unwrap()))
}
