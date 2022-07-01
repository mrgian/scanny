use std::{env, time::Duration};

use reqwest::{blocking::Client, redirect};
mod error;
use error::Error;

mod model;
mod subdomains;
use subdomains::enumerate;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(30);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let subdomains = enumerate(&http_client, target)?;

    for subdomain in subdomains {
        println!("{}", subdomain.domain)
    }

    Ok(())
}
