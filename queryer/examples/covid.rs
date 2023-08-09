use std::io::Cursor;
use anyhow::Result;
use polars::prelude::*;
use reqwest::Proxy;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();
    // let url = "https://raw.github.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    //
    // let proxy = Proxy::http("http://127.0.0.1:7890")?;
    // let res = reqwest::Client::builder().
    //     proxy(proxy).build()
    //     .unwrap()
    //     .get(url)
    //     .send()
    //     .await?;
    //
    // let data = res.text().await?;
    // let df = CsvReader::new(Cursor::new(data)).infer_schema(Some(16)).finish()?;

    let path = "C:\\workspace\\hello-rust\\queryer\\examples\\latest_owid-covid-latest.csv";
    let df = CsvReader::from_path(path)?.finish()?;

    let filtered = df.filter(&df.column("new_deaths")?.gt(10)?)?;

    println!("{:?}", filtered.select(["location", "total_cases", "new_cases", "total_deaths", "new_deaths"]));
    Ok(())
}