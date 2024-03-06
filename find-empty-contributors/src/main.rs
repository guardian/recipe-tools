use std::error::Error;

use recipes_lib;
use recipes_lib::recipe_model::RecipeModel;
use clap::{Arg, Parser};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();

    println!("INFO: Downloading index...");
    let idx = recipes_lib::get_recipes_index(&args.host).await?;

    println!("INFO: Downloading recipes...");
    let recipes = idx.all_recipes_content(&args.host).await?;

    println!("INFO: Got a total of {} recipes", (recipes.len()));
    
    let problematic:Vec<RecipeModel> = recipes.into_iter().filter(|recep| {
        recep.contributors.len()==0 && recep.byline.clone().unwrap_or(vec![]).len()==0
    }).collect();

    println!("INFO: Got {} recipes which had no contributors or byline", problematic.len());

    for recep in problematic {
        println!("https://composer.gutools.co.uk/content/{}\t{}", recep.composer_id.unwrap_or("(no composerid)".to_string()), recep.id);
    }
    Ok( () )
}
