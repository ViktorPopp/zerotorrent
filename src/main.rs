use clap::{Parser, Subcommand, command};

mod bencode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Decodes a Bencoded string")]
    Decode {
        #[arg()]
        value: String,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Decode { value } => {
            let (value, rest) = bencode::decode_value(value);
            println!("Decoded Value: {value}");
            if !rest.is_empty() {
                println!("Remainder:     \"{rest}\"");
            }
        }
    }
}
