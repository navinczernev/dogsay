fn main() {
    let message = std::env::args().nth(1)
                        .expect("Missing the message. Usage dogsay <message>");
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
