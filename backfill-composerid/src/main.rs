use clap::Parser;
use recipes_lib::recipe_model::RecipeModel;
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    host: String,

    #[arg(short, long)]
    modify: bool
}


#[tokio::main]
async fn main() -> Result< (), Box<dyn Error>>{
    let args = Args::parse();
    
    println!("INFO: Downloading index...");
    let idx = recipes_lib::get_recipes_index(&args.host).await?;
    println!("INFO Index has {} recipes", idx.count());

    println!("INFO: Downloading recipes...");
    let recipes = idx.all_recipes_content(&args.host).await?;

    println!("INFO: Got a total of {} recipes", (recipes.len()));

    let recipes_no_composerid:Vec<&RecipeModel> = recipes.iter().filter(|recep| match &recep.composer_id {
        None=>true,
        Some(id_value)=>id_value==""
    }).collect();

    println!("INFO {} / {} recipes were missing a composer ID", recipes_no_composerid.len(), recipes.len());

    let recipes_no_composerid_with_capi_id:Vec<&&RecipeModel> = recipes_no_composerid.iter().filter(|recep| match &recep.canonical_article {
        None=>false,
        Some(capi_id)=>capi_id!=""
    }).collect();

    println!("INFO {} / {} recipes had no composer ID AND no CAPI ID therefore are not processable", recipes_no_composerid.len() - recipes_no_composerid_with_capi_id.len(), recipes_no_composerid.len() );

    Ok( () )
}
