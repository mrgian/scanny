use std::time::Duration;

use reqwest::{blocking::Client, redirect};
mod error;
mod model;
mod subdomains;
use subdomains::enumerate;

fn main() -> Result<(), anyhow::Error> {
    let target = "gian.im";

    let http_timeout = Duration::from_secs(5);
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
