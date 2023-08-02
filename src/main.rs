use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    text: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let mut result = "".to_owned();

    for (idx, text) in args.text.iter().enumerate() {
        let pier = "pier-";

        if idx > 0 {
            result.push_str(" ")
        }

        result.push_str(pier);
        result.push_str(&text);
    }

    print!("{}", result)
}
