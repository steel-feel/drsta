use clap::{Parser, ValueEnum};

#[derive(Copy, Clone,Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Converters {
    Hex
}

/// Drsta CLI for converstion
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   ///Value to convert
   #[arg(short, long)]
   value: u128,

   ///Type to convert
   #[arg(short, long,value_enum)]
   to: Converters
}

fn main() {
   let args = Cli::parse();
   
   println!("{}", args.to);

}