use serde::{Deserialize, Serialize};

/*
    url: string;
    mediaId: string;
    cropId: string;
    source?: string;
    photographer?: string;
    imageType?: string;
    caption?: string;
    mediaApiUri?: string;
    displayCredit?: boolean;
    width: number;
    height: number; */

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipeImage {
    pub url: String,
    #[serde(rename = "mediaId")]
    pub media_id: String,
    #[serde(rename = "cropId")]
    pub crop_id: String,
    pub source: Option<String>,
    pub photographer: Option<String>,
    #[serde(rename = "imageType")]
    pub image_type: Option<String>,
    pub caption: Option<String>,
    #[serde(rename = "displayCredit")]
    pub display_credit: Option<bool>,
    pub width: Option<u32>,
    pub height: Option<u32>
}

/*
    id: string;
    composerId: string;
    canonicalArticle: string;
    title: string;
    description: string;
    isAppReady: boolean;
    featuredImage?: RecipeImage;
    contributors: Contributor[];
    ingredients: Ingredient[];
    suitableForDietIds: string[];
    cuisineIds: string[];
    mealTypeIds: string[];
    celebrationIds: string[];
    utensilsAndApplianceIds: string[];
    techniquesUsedIds: string[];
    difficultyLevel: string;
    serves: Serves[];
    timings: Timing[];
    instructions: Instruction[];
    bookCredit: string; */

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipeModel {
    pub id: String,
    #[serde(rename = "composerId")]
    pub composer_id: Option<String>,
    #[serde(rename = "canonicalArticle")]
    pub canonical_article: Option<String>,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "isAppReady")]
    pub is_app_ready: Option<bool>,
    #[serde(rename = "featuredImage")]
    pub featured_image: Option<RecipeImage>,
    pub contributors: Vec<String>,
    pub byline: Option<Vec<String>>,
}

