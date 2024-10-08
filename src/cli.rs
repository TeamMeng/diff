use anyhow::{anyhow, Result};
use clap::Parser;

use crate::ExtraArgs;

#[derive(Debug, Parser)]
#[command(name = "cli", version, author, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Parser)]
#[non_exhaustive]
pub enum Action {
    #[command(name = "xdiff")]
    Run(RunArgs),
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    #[arg(short, long, value_parser)]
    pub profile: String,

    // For query params, user `-e key=value`
    // For headers params, user `-e %key=value`
    // For body, user `-e @key=value`
    #[arg(short, long, value_parser = parse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    #[arg(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

pub fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let key = retrieve(parts.next())?;
    let value = retrieve(parts.next())?;

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

fn retrieve(s: Option<&str>) -> Result<&str> {
    match s {
        Some(s) => Ok(s.trim()),
        None => Err(anyhow!("Invalid key value pair: {:?}", s)),
    }
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(value: Vec<KeyVal>) -> Self {
        let mut headers = Vec::new();
        let mut query = Vec::new();
        let mut body = Vec::new();

        for arg in value {
            match arg.key_type {
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
                KeyValType::Body => body.push((arg.key, arg.value)),
            }
        }

        Self {
            headers,
            query,
            body,
        }
    }
}
