use recipes_lib::{recipe_model::RecipeModel, RecipesIndex};
use std::error::Error;
use aws_sdk_s3 as s3;

pub async fn update_file(updated: &RecipeModel, idx:&RecipesIndex, write_updates:bool) -> Result<(), Box<dyn Error>> {
    let config = aws_config::load_from_env().await;
    let s3client = s3::Client::new(&config);

    match idx.find_uid(&updated.id) {
        None=> {
            let msg = format!("Could not find {} in the index", &updated.id);
            Err(msg.into())
        },
        Some(idx_entry)=>{
            let path = format!("content/{}", idx_entry.checksum);

            println!("INFO Writing to {}", path);
            let marshalled = serde_json::to_string(idx_entry)?;

            if write_updates {
                let result = s3client.put_object()
                    .key(&path)
                    .body(marshalled.into_bytes().into())
                    .send()
                    .await?;

                println!("INFO Written {} with version ID {}", &path, result.version_id.unwrap_or("(no version)".to_string()));
            } else {
                println!("INFO I would write to {}: {}", &path, &marshalled);
            }
            Ok( () )

        }
    }

}