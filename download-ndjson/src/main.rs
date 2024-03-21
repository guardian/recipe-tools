use std::{error::Error, fs::File};
use std::io::Write;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    host: String,

    #[arg(long, short)]
    output: String,

    #[arg(long, short)]
    roundtrip: Option<bool>
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Writing to {}", args.output);
    let mut file = File::create(args.output)?;

    println!("Downloading index from {}...", &args.host);
    let idx = recipes_lib::get_recipes_index(&args.host).await?;
    println!("Downloading {} recipes...", idx.count());
    let receps = idx.all_recipes_content(&args.host, args.roundtrip).await?;

    let newline = "\n";
    for r in receps {
        let serialized = serde_json::to_string(&r)?;
        file.write_all(serialized.as_bytes())?;
        file.write(newline.as_bytes())?;
    }
    Ok( () )
}