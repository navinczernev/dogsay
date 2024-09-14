use std::io::{self, Read};

use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};


#[derive(Parser)]
struct Options {
    #[clap(default_value="woof!")]
    /// What does the dog say?
    message: String,
    #[clap(short='d', long="dead")]
    /// Make the dog appear dead
    dead: bool,
    #[clap(short='f', long="file")]
    /// Load the cat picture from the specified file
    dogfile: Option<std::path::PathBuf>,
    #[clap(short='i', long="stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }
    match &options.dogfile {
        Some(path) => {
            let dog_template = std::fs::read_to_string(path)
                                        .with_context(
                                            || format!("Could not read file {:?}", path)
                                        )?;
            let eye = if options.dead { "x" } else { "0" };
            let eye = format!("{}", eye.red().bold());
            let dog_picture = dog_template.replace("{eye}", &eye);
            println!("{}", message.underline().bright_green().on_blue());
            println!("{}", &dog_picture);
        },
        None => {
            if message.to_lowercase().contains("meow") {
                eprintln!("A dog shouldn't say meow.");
            }
            let eye = if options.dead { "x" } else { "0" };
            let eye = format!("{}", eye.red().bold());
            println!("{}", message.underline().bright_green().on_blue());
            println!("\\");
            println!(" \\");
            println!("  /^ ^\\");
            println!(" / {eye} {eye} \\");
            println!(" V\\ Y /V");
            println!("  / - \\");
            println!("  |    \\");
            println!("  || (__V");
        },
    }
    Ok(())
}
