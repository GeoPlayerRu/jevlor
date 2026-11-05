use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {}

fn main() {
    let args = Args::parse();
}
