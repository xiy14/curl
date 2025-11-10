use structopt::StructOpt;
use reqwest::blocking::Client;
use url::Url;
use serde_json::Value;

#[derive(StructOpt)]
struct Cli {
    /// URL to request
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    // parse
    let parsed_url = Url::parse(&args.url)?;
    println!("Requesting URL: {}", parsed_url);
    println!("Method: GET");

    let client = Client::new();
    let response = client.get(parsed_url.clone()).send()?;

    let body = response.text()?;

    println!("Response body:\n{}", body);
    Ok(())
}
