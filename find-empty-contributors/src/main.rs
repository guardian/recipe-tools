use std::error::Error;

use recipes_lib::{self, RecipesIndex};
use recipes_lib::recipe_model::RecipeModel;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    host: String,
}

fn dump_list(receplist:&Vec<&RecipeModel>, idx:&RecipesIndex) {
    for recep in receplist {
        let index_entry = idx.find_uid(&recep.id).map(|ent| &ent.checksum);
        let composer_id = match &recep.composer_id {
            Some(id)=>format!("https://composer.gutools.co.uk/content/{}", id),
            None=>"(no_composerid)".to_string(),
        };

        let capi_path = match &recep.canonical_article {
            Some(id)=>format!("https://www.theguardian.com/{}", id),
            None=>"(no_capi_path)".to_string(),
        };

        println!("\"{}\",\"{}\",\"{}\",\"{}\"", composer_id, capi_path, recep.id, index_entry.unwrap_or(&"(no checksum)".to_string()));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();

    println!("INFO: Downloading index...");
    let idx = recipes_lib::get_recipes_index(&args.host).await?;

    println!("INFO: Downloading recipes...");
    let recipes = idx.all_recipes_content(&args.host).await?;

    println!("INFO: Got a total of {} recipes", (recipes.len()));
    
    let not_app_ready:Vec<&RecipeModel> = recipes.iter().filter(|recep| {
        !recep.is_app_ready.unwrap_or(true)
    }).collect();

    println!("INFO Got {} recipes which were not marked as app-ready", not_app_ready.len());
    dump_list(&not_app_ready, &idx);

    let problematic:Vec<&RecipeModel> = recipes.iter().filter(|recep| {
        recep.contributors.len()==0 && recep.byline.clone().unwrap_or(vec![]).len()==0
    }).collect();

    println!("INFO: Got {} recipes which had no contributors or byline", problematic.len());
    dump_list(&problematic, &idx);

    Ok( () )
}
