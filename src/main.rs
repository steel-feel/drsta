use clap::{Parser, ValueEnum};

#[derive(Copy, Clone,Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Converters {
    /// convert to hex
    Hex,
    // convert to num
    Dec
}

/// Drsta CLI for converstion
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   ///Value to convert
   #[arg(short, long)]
   value: String,

   ///Type to convert
   #[arg(short, long,value_enum)]
   to: Converters
}

fn main() {
   let args = Cli::parse();

match args.to {
    Converters::Hex => println!("{:x}", args.value.parse::<i64>().unwrap()  ),
    Converters::Dec => println!("{:?}", i64::from_str_radix(args.value.trim_start_matches("0x") ,16).unwrap() ),
    _ => println!("could not convert")
}


}