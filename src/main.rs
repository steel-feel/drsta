use clap::{Parser, ValueEnum};
use arboard::Clipboard;

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
   #[arg(short, long,default_value ="")]
   value: String,

   ///Type to convert
   #[arg(short, long,value_enum)]
   to: Converters
}

fn main() {
    let v:String;
    let args = Cli::parse();

   if args.value.len() == 0 {
    let mut clipboard = Clipboard::new().unwrap();
    v = clipboard.get_text().unwrap();
   }else{
    v = args.value;
   }

match args.to {
    Converters::Hex => println!("{:x}", v.parse::<i64>().unwrap()  ),
    Converters::Dec => println!("{:?}", i64::from_str_radix(v.trim_start_matches("0x") ,16).unwrap() ),
    _ => println!("could not convert")
}

}