use clap::Parser;
use colored::Colorize;


#[derive(Parser)]
struct Options {
    #[clap(default_value="woof!")]
    /// What does the dog say?
    message: String,
    #[clap(short='d', long="dead")]
    /// Make the dog appear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    if message.to_lowercase().contains("meow") {
        eprintln!("A dog shouldn't say meow.");
    }
    let eye = (if options.dead { "x" } else { "0" }).red().bold();
    println!("{}", message.underline().bright_green().on_blue());
    println!("\\");
    println!(" \\");
    println!("  /^ ^\\");
    println!(" / {eye} {eye} \\");
    println!(" V\\ Y /V");
    println!("  / - \\");
    println!("  |    \\");
    println!("  || (__V");
}
