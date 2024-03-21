use json_structural_diff::JsonDiff;
use serde::{Deserialize, Serialize};
use reqwest::{self, Client, StatusCode};
use simple_error::bail;
use std::error::Error;
pub mod recipe_model;
use crate::recipe_model::RecipeModel;
use serde_json::{Value, Map};

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

impl RecipesIndex {
    pub fn count(&self) -> usize {
        self.recipes.len()
    }
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
    fn internal_unmarshal(content:&str) -> Result<RecipeModel, Box<dyn Error>> {
        let unmarshalled:RecipeModel = serde_json::from_str(content)?;
        Ok( unmarshalled )
    }

    // fn significant_terms(from:&Map<String, Value>) -> Value {
    //    from.
    // }
    fn json_diff(&self, incoming:&str, remarshalled:&str, labelled: &str) -> Result<bool, Box<dyn Error>> {
        let incoming_src: Value = serde_json::from_str(incoming)?;
        let remarshalled_src: Value = serde_json::from_str(remarshalled)?;
    
        match JsonDiff::diff_string(&incoming_src, &remarshalled_src, false) {
            None=>Ok( false ),
            Some(diff)=>{
                println!("----------------------");
                println!("{}", labelled);
                println!("{}", &diff);
                println!("----------------------");
                Ok(true)
            }
        }
    } 

    pub async fn all_recipes_content(&self, hostname: &str, validate_model: Option<bool>) -> Result<Vec<RecipeModel>, Box<dyn Error>> {
        let mut results:Vec<RecipeModel> = vec![];

        let client = Client::new();

        let should_validate = validate_model.unwrap_or(false);

        for recep in self.recipes.iter() {
            let url = format!("https://{}/content/{}", hostname, recep.checksum); 
            let req = client.get(url).build()?;

            let response = client.execute(req).await?;
            match response.status() {
                StatusCode::OK=>{
                    let content = response.text().await?;
                    match RecipesIndex::internal_unmarshal(&content) {
                        Ok(unmarshalled)=>{
                            let _ = match should_validate {
                                true=>{
                                    let remarshalled = serde_json::to_string(&unmarshalled)?;
                                    let label = format!("{} / {}", recep.recipe_uid, recep.checksum);
                                    self.json_diff(&content, &remarshalled, &label)?
                                    // let remarshalled = serde_json::to_string(&unmarshalled)?;
                                    // if remarshalled != content {
                                    //     println!("WARNING {} / {}: failed marshalling check", recep.recipe_uid, recep.checksum);
                                    //     println!("Original: {}", content);
                                    //     println!("Remarshalled: {}", remarshalled);
                                    //     return Err("Roundtrip test failed".into());
                                    // }
                                },
                                false=>false,
                            };

                            results.push(unmarshalled)
                        },
                        Err(err)=>println!("ERROR Could not unmarshal data for recipe {} / {}: {}", recep.recipe_uid, recep.checksum, err),
                    }
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