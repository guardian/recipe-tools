use clap::Parser;
use recipes_lib::{capi_client::capi_single_article, recipe_model::RecipeModel};
use std::{borrow::BorrowMut, error::Error};
mod update;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    host: String,

    #[arg(short, long)]
    modify: bool,

    #[arg(short, long)]
    key: String,

    #[arg(short,long)]
    capi_base: String,

    #[arg(short,long)]
    write_updates: bool
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

    let http_client = reqwest::Client::new();
    let fields = vec!["internalComposerCode"];

    for recep in recipes_no_composerid_with_capi_id.into_iter() {
        let canonical_article_id = recep.canonical_article.clone().unwrap();

        let capi_article = capi_single_article(&http_client, &args.capi_base,&canonical_article_id, &args.key, Some(&fields)).await?;
        let maybeComposerId = &capi_article.response.content
                                            .map(|content| content.fields.map(|f| f.internal_composer_code).flatten())
                                            .flatten();
        match maybeComposerId {
            Some(composer_id)=>{
                println!("{} -> {}", canonical_article_id, composer_id);

                let mut updated = (*recep).clone();
                updated.composer_id = Some(composer_id.clone());
                println!("{:?}", &updated);

                update::update_file(&updated, &idx, args.write_updates).await?;

            },
            None=>{
                println!("ERROR Unable to get a Composer ID for {}!", canonical_article_id);
                break;
            }
        }
    }

    Ok( () )
}
