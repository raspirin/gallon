use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arg {

    install: Option<String>
}

fn main() {
    let args = Arg::parse();

}
