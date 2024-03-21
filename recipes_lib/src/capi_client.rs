use serde::{Deserialize, Serialize};
use reqwest::{self, Client, StatusCode};
use simple_error::bail;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct CapiArticleFields {
    #[serde(rename = "internalComposerCode")]
    pub internal_composer_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapiArticle {
    pub id: String,
    #[serde(rename = "type")]
    pub article_type: String,
    #[serde(rename = "sectionId")]
    pub section_id: String,
    #[serde(rename = "sectionName")]
    pub section_name: String,
    #[serde(rename = "webPublicationDate")]
    pub web_publication_date: String,   //OK so this should be a datetime :shrug:
    #[serde(rename="webTitle")]
    pub web_title: String,
    #[serde(rename = "webUrl")]
    pub web_url: String,
    #[serde(rename = "apiUrl")]
    pub api_url: String,
    pub fields: Option<CapiArticleFields>,
    #[serde(rename = "isHosted")]
    pub is_hosted: bool,
    #[serde(rename = "pillarId")]
    pub pillar_id: Option<String>,
    #[serde(rename = "pillarName")]
    pub pillar_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapiArticleResponse {
    pub status: String,
    #[serde(rename = "userTier")]
    pub user_tier: String,
    pub total: usize,
    pub content: Option<CapiArticle>,   //only populated if total != 0
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapiArticleEnvelope {
    pub response:CapiArticleResponse,
}

pub async fn capi_single_article(http_client: &Client, base_url:&str, canonical_article_id: &str, api_key:&str, show_fields:Option<&Vec<&str>>) -> Result<CapiArticleEnvelope, Box<dyn Error>> {
    let params:Vec<String> = vec!(
        ("api-key", Some(api_key)),
        ("show-fields", show_fields.map(|fields| fields.join(",")).as_deref()),
    ).iter()
        .filter(|kv| kv.1.is_some())
        .map(|kv| format!("{}={}", kv.0, kv.1.unwrap()))
        .collect();

    let url = format!("{}/{}?{}", base_url, canonical_article_id, params.join("&"));

    println!("DEBUG CAPI URL is {}", url);

    let req = http_client.get(url).build()?;

    let response = http_client.execute(req).await?;
    match response.status() {
        StatusCode::OK=>{
            let content = response.text().await?;
            //println!("{}", &content);
            let unmarshalled:CapiArticleEnvelope = serde_json::from_str(&content)?;
            Ok(unmarshalled)
        },
        _=>{
            bail!(format!("CAPI returned a {} error", response.status()));
        }
    }
}