use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8)]
    days: u64,
}

fn main() {
    let args = Args::parse();

    let output = calendar::display_day(args.days);

    println!("{output}");
}
