use std::{fs};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "url")]
struct Opt {
    #[structopt(short, long)]
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    println!("{:#?}", args);

    let url = args.url;
    let output = "rust.md";

    println!("Fetching url:{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);
    Ok(())
}