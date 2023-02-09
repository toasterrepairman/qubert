mod generator;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    prompt: String,

    /// Minimum generation length
    #[arg(long, default_value_t = 20)]
    min: i64,

    /// Maximum generation length
    #[arg(long, default_value_t = 100)]
    max: i64,
}

fn main() {
    let args = Args::parse();

    println!("{}", generator::generate(&args.prompt, args.min, Some(args.max)))
}
