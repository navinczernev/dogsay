use clap::Parser;


#[derive(Parser)]
struct Options {
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
