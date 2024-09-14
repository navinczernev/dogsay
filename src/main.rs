use clap::Parser;


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
    let eye = if options.dead { "x" } else { "0" };
    println!("{message}");
    println!("\\");
    println!(" \\");
    println!("  /^ ^\\");
    println!(" / {eye} {eye} \\");
    println!(" V\\ Y /V");
    println!("  / - \\");
    println!("  |    \\");
    println!("  || (__V");
}
