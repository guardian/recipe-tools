use serde::{Deserialize, Serialize};
use reqwest::{self, Client, StatusCode};
use simple_error::bail;
use std::error::Error;
pub mod recipe_model;
use crate::recipe_model::RecipeModel;

#[derive(Serialize, Deserialize)]
pub struct RecipeIndexEntry {
    #[serde(rename = "recipeUID")]
    pub recipe_uid: String,
    pub checksum: String
}

#[derive(Serialize, Deserialize)]
pub struct RecipesIndex {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u8,
    pub recipes: Vec<RecipeIndexEntry>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

pub async fn get_recipes_index(hostname: &str) -> Result<RecipesIndex, Box<dyn Error>>{
    let url = format!("https://{}/index.json", hostname);

    let content = reqwest::get(url)
        .await?
        .text()
        .await?;

    let unmarshalled:RecipesIndex = serde_json::from_str(&content)?;
    Ok( unmarshalled )
}

impl RecipesIndex {
    pub async fn all_recipes_content(&self, hostname: &str) -> Result<Vec<RecipeModel>, Box<dyn Error>> {
        let mut results:Vec<RecipeModel> = vec![];

        let client = Client::new();

        for recep in self.recipes.iter() {
            let url = format!("https://{}/content/{}", hostname, recep.checksum); 
            let req = client.get(url).build()?;

            let response = client.execute(req).await?;
            match response.status() {
                StatusCode::OK=>{
                    let content = response.text().await?;
                    let unmarshalled:RecipeModel = serde_json::from_str(&content)?;
                    results.push(unmarshalled);
                },
                StatusCode::NOT_FOUND=>{
                    println!("WARNING Recipe with uid {} and checksum {} was not found", recep.recipe_uid, recep.checksum);
                },
                _=>{
                    bail!(format!("The backend returned a {} error", response.status()));
                }
            }
        }
        Ok ( results )
    }

    pub fn find_uid(&self, uid: &str) -> Option<&RecipeIndexEntry> {
        self.recipes.iter().find(|recep| recep.recipe_uid==uid)
    }
}