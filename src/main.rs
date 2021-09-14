use std::io::{stdin, stdout, BufRead, Write};

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
            Ok(0) => break,
            Err(_) => break,
            Ok(_) => {
                if let Some(s) = split::process_line(&args, &buf) {
                    output.write_all(s.as_bytes())?;
                    output.write_all("\n".as_bytes())?;
                }
            }
        };

        buf.clear();
    }

    output.flush()?;
    Ok(())
}
