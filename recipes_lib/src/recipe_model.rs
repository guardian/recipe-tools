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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
  #[serde(rename = "ingredientsList")]
  pub ingredients_list: Vec<IngredientData>,
  #[serde(rename = "recipeSection")]
  pub recipe_section: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IngredientData {
  pub name: String,
  #[serde(rename = "ingredientId")]
  pub ingredient_id: Option<String>,
  pub amount: Option<Amount>,
  pub unit: Option<String>,
  pub prefix: Option<String>,
  pub suffix: Option<String>,
  pub text: Option<String>,
  pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
  pub min: Option<f32>,
  pub max: Option<f32>
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeModel {
    pub id: String,
    #[serde(rename = "composerId")]
    pub composer_id: Option<String>,
    #[serde(rename = "canonicalArticle")]
    pub canonical_article: Option<String>,
    pub title: String,
    pub description: String,
    #[serde(rename = "isAppReady")]
    pub is_app_ready: Option<bool>,
    #[serde(rename = "featuredImage")]
    pub featured_image: Option<RecipeImage>,
    pub contributors: Vec<String>,
    pub byline: Option<Vec<String>>,
}

