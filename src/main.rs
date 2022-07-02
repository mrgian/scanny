use std::{env, time::Duration};

use model::Subdomain;
use reqwest::{blocking::Client, redirect};
mod error;
use error::Error;

mod consts;
mod model;
mod subdomains;
use subdomains::enumerate;

mod ports;
use ports::scan_ports;

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

    let subdomains: Vec<Subdomain> = enumerate(&http_client, target)?
        .into_iter()
        .map(scan_ports)
        .collect();

    for subdomain in subdomains {
        println!("Domain: {}", &subdomain.domain);
        println!("Open ports:");
        for port in &subdomain.open_ports {
            println!("    {}", port.port);
        }

        println!();
    }

    Ok(())
}
