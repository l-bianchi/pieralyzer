use clap::Parser;

#[derive(Parser)]
struct Cli {
    param: String,
}

fn main() {
    let args = Cli::parse();

    println!("pier-{}", args.param);
}
