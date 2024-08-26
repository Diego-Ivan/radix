/* main.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod converter;

use anyhow::anyhow;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum DecimalSeparator {
    Comma,
    Point,
}

impl DecimalSeparator {
    pub fn to_char(&self) -> char {
        match *self {
            Self::Comma => ',',
            Self::Point => '.',
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "radix")]
#[command(about = "Convert numbers from a system to another", long_about = None)]
struct Cli {
    /// The source value in the source radix
    value: String,
    /// The source radix
    #[arg(long)]
    from: String,
    /// The destination radix
    #[arg(long)]
    to: String,
    /// The decimal separator that the input uses
    #[arg(short, long)]
    decimal_separator: Option<DecimalSeparator>,
    /// Just print the result instead of LaTex formatted output
    #[arg(short, long)]
    simple: bool,
}

fn string_to_radix(value: &str) -> anyhow::Result<u8> {
    match value {
        "binary" | "bin" => Ok(2),
        "octal" | "oct" => Ok(8),
        "decimal" | "dec" => Ok(10),
        "hexadecimal" | "hex" => Ok(16),
        _ => {
            let num: u8 = value.parse()?;
            if num < 2 || num > 36 {
                return Err(anyhow!("Radix must be in range [2, 36]"));
            }
            Ok(num)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let from_radix = string_to_radix(args.from.to_lowercase().trim())?;
    let to_radix = string_to_radix(args.to.to_lowercase().trim())?;

    let converter = converter::Converter {
        from_radix,
        to_radix,
        decimal_separator: args
            .decimal_separator
            .unwrap_or(DecimalSeparator::Point)
            .to_char(),
        max_decimals: 8,
    };

    let result = converter.convert(&args.value)?;

    if args.simple {
        println!("{result}");
    } else {
        println!(
            "{source}_{{{from_radix}}} = {result}_{{{to_radix}}}",
            source = &args.value
        );
    }

    Ok(())
}
