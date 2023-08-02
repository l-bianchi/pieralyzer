use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    param: String,
}

fn main() {
    let args = Cli::parse();

    println!("pier-{}", args.param);
}
