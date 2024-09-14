use clap::Parser;


#[derive(Parser)]
struct Options {
    #[clap(default_value="woof!")]
    /// What does the dog say?
    message: String,
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    println!("{message}");
    println!("\\");
    println!(" \\");
    println!("  /^ ^\\");
    println!(" / 0 0 \\");
    println!(" V\\ Y /V");
    println!("  / - \\");
    println!("  |    \\");
    println!("  || (__V");
}
