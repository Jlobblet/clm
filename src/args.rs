use anyhow::{anyhow, Result};
use regex::{Error, Regex};
use std::io::{BufRead, StdinLock};
use structopt::{clap::ArgGroup, StructOpt};

fn parse_regex(s: &str) -> Result<Regex, Error> {
    Regex::new(s)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "clm", group = ArgGroup::with_name("mode").required(true))]
pub struct Args {
    #[structopt(short, long, default_value = r"\s+", parse(try_from_str = parse_regex))]
    pub delimiter: Regex,

    #[structopt(long, parse(try_from_str = parse_regex))]
    pub filter: Option<Regex>,

    #[structopt(short, long, group = "mode")]
    pub field: Option<usize>,

    #[structopt(short, long, group = "mode")]
    pub col_name: Option<String>,
}

pub struct ProcessedArgs {
    pub delimiter: Regex,
    pub filter: Option<Regex>,
    pub field: usize,
    pub col_name: Option<String>,
}

impl ProcessedArgs {
    pub fn new(args: Args, lock: &mut StdinLock) -> Result<Self> {
        Ok(match &args.col_name {
            Some(n) => {
                // If a column name has been provided, get the index of it
                let mut buf = String::new();
                lock.read_line(&mut buf)?;
                let index = args
                    .delimiter
                    .split(buf.as_str())
                    .position(|s| s == n)
                    .ok_or_else(|| anyhow!("Could not find column {}", n))?;
                ProcessedArgs {
                    delimiter: args.delimiter,
                    filter: args.filter,
                    field: index + 1, // 1 is the first column, not 0
                    col_name: args.col_name,
                }
            }
            None => ProcessedArgs {
                delimiter: args.delimiter,
                filter: args.filter,
                field: args.field.unwrap(),
                col_name: None,
            },
        })
    }
}
