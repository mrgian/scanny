use std::collections::HashSet;

use crate::error::*;
use crate::model::*;
use reqwest::blocking::Client;

pub fn enumerate(http_client: &Client, target: &str) -> Result<HashSet<String>, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|subdomain| subdomain != target)
        .filter(|subdomain| !subdomain.contains("*"))
        .collect();

    subdomains.insert(target.to_string());

    Ok(subdomains)
}
