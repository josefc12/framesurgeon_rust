use clap::Parser;

mod structs;
mod process_frames;

fn main() {

    let args = structs::CliInput::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)

}