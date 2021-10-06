#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]

use std::io::{stderr, stdin, stdout, BufRead, Write};

use anyhow::Result;

use crate::args::ProcessedArgs;
use args::Args;

mod args;
mod split;

#[paw::main]
fn main(args: Args) -> Result<()> {
    let stdin = stdin();
    let mut lock = stdin.lock();
    let mut output = stdout();
    let mut buf = String::new();

    let args = ProcessedArgs::new(args, &mut lock)?;

    loop {
        match lock.read_line(&mut buf) {
            Ok(0) => {
                // nothing left in stdin
                return Ok(());
            }
            Err(e) => {
                let mut stderr = stderr();
                stderr.write_all(e.to_string().as_ref())?;
                return Err(anyhow::Error::from(e));
            }
            Ok(_) => {
                if let Some(s) = split::process_line(&args, &buf) {
                    output.write_all(s.as_bytes())?;
                    output.write_all("\n".as_bytes())?;
                }
            }
        };

        buf.clear();
        output.flush()?;
    }
}
