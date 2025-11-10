use structopt::StructOpt;
use reqwest::blocking::Client;
use url::{ParseError, Url};
use serde_json::{Value, Map};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
#[derive(StructOpt)]
struct Cli {
    /// URL 
    url: String,

    /// method
    #[structopt(short = "X", long = "method", default_value = "GET")]
    method: String,

    /// POST 
    #[structopt(short = "d", long = "data")]
    data: Option<String>,

    // json
    #[structopt(long = "json")]
    json: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    let url = args.url;
    let mut method = args.method;
    if args.json.is_some() {
        method = "POST".to_string();
    }

    println!("Requesting URL: {}", url);
    println!("Method: {}", method);

    // url error handling
    let parsed_url = match Url::parse(&url) {
        Ok(url) => url,
        Err(e) => {
            match e {
                ParseError::RelativeUrlWithoutBase => {
                    eprintln!("Error: The URL does not have a valid base protocol.");
                }
                ParseError::InvalidIpv6Address => {
                    eprintln!("Error: The URL contains an invalid IPv6 address.");
                }
                ParseError::InvalidIpv4Address => {
                    eprintln!("Error: The URL contains an invalid IPv4 address.");
                }
                ParseError::InvalidPort => {
                    eprintln!("Error: The URL contains an invalid port number.");
                }
                _ => {
                    eprintln!("Error: The URL does not have a valid base protocol.");
                }
            }
        return;
        }
    };

    match parsed_url.scheme() {
        "http" | "https" => {}
        _ => {
            // protocol error
            eprintln!("Error: The URL does not have a valid base protocol.");
            return;
        }
    }

    let client = Client::new();
    let response_result = 
        if method == "POST" {
            if let Some(json_str) = args.json {
                println!("JSON: {}", json_str);
                let _parsed: Value = serde_json::from_str(&json_str).unwrap_or_else(|e| panic!("Invalid JSON: {}", e));

                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                client.post(parsed_url.clone()).headers(headers).body(json_str).send()
            } else if let Some(ref data) = args.data {
                println!("Data: {}", data);

                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
                client.post(parsed_url.clone()).headers(headers).body(data.clone()).send()
            } else {
                client.post(parsed_url.clone()).send()
            }
        } else {
            client.get(parsed_url.clone()).send()
        };

    // web server error handling
    // cannot resolve
    let response = match response_result {
        Ok(resp) => resp,
        Err(_e) => {
            eprintln!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
            return;
        }
    };

    // return status code
    if !response.status().is_success() {
        eprintln!("Error: Request failed with status code: {}.", response.status().as_u16());
        return;
    }

    let body = response.text().expect("REASON");

    if let Ok(mut json_value) = serde_json::from_str::<Value>(&body) {
        sort_json_keys(&mut json_value);
        println!("Response body (JSON with sorted keys):\n{}", serde_json::to_string_pretty(&json_value).unwrap());
    } else {
        println!("Response body:\n{}", body);
    }
    
    return;
}

fn sort_json_keys(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<(String, Value)> = map.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            // sort by key
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            // reconstruct
            let mut new_map = Map::new();
            for (k, mut v) in entries {
                sort_json_keys(&mut v);
                new_map.insert(k, v);
            }

            *map = new_map;
        }
        Value::Array(arr) => {
            for item in arr {
                sort_json_keys(item);
            }
        }
        _ => {}
    }
}