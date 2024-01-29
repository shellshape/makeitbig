use anyhow::Result;
use clap::{command, Parser};
use rand::prelude::*;
use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// Tiny CLI tool to generate random files of given sizes.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File to write output to.
    #[arg(
        short,
        long,
        long_help = "File to write output to. If not specified, output is written to StdOut."
    )]
    output: Option<PathBuf>,

    /// Size of the output.
    #[arg(
        short,
        long,
        long_help = "Size of the output.\nFormat examples: 512, 2m, 8G, ..."
    )]
    size: String,

    /// Size of each write buffer.
    #[arg(
        long,
        default_value = "4k",
        long_help = "Size of each write buffer. \
                     Larger buffer size can improve write speed but increases memory consumption.\n\
                     Format examples: 512, 2m, 8G, ..."
    )]
    chunk_size: String,

    /// Write zero bytes instead of random data.
    #[arg(short, long)]
    zeroed: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let size = parse_size(&cli.size)?;
    let chunk_size = parse_size(&cli.chunk_size)?;

    let mut out = get_output(cli.output)?;
    let mut rng = rand::thread_rng();

    let mut buf = vec![0u8; chunk_size];
    for _ in 0..size / chunk_size {
        if !cli.zeroed {
            rng.fill_bytes(&mut buf);
        }
        out.write_all(&buf)?;
    }

    let mut buf_rest = vec![0u8; size % chunk_size];
    if !cli.zeroed {
        rng.fill_bytes(&mut buf_rest);
    }
    out.write_all(&buf_rest)?;

    Ok(())
}

fn get_output<P: AsRef<Path>>(path: Option<P>) -> Result<Box<dyn io::Write>> {
    Ok(match path {
        Some(p) => Box::new(File::create(p)?),
        None => Box::new(io::stdout()),
    })
}

fn parse_size(v: &str) -> Result<usize> {
    let v = v.trim().to_lowercase();

    let mut number_part = String::new();
    let mut mult = 1;

    for (i, c) in v.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            if i != v.len() - 1 {
                anyhow::bail!("scale suffix must ({c}) must be at the end of input")
            }
            mult = scale_mult(c)?;
            break;
        }

        if !c.is_ascii_digit() {
            anyhow::bail!("invalid digit {c}")
        }

        number_part.push(c);
    }

    let n: usize = number_part.parse()?;
    Ok(n * mult)
}

fn scale_mult(r: char) -> Result<usize> {
    Ok(match r {
        'k' => 1024,
        'm' => 1024 * 1024,
        'g' => 1024 * 1024 * 1024,
        't' => 1024 * 1024 * 1024 * 1024,
        _ => anyhow::bail!("invalid scale character {r}"),
    })
}
