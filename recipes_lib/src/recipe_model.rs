use std::collections::btree_map::Range;

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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RangeValue {
    min: Option<f64>,
    max: Option<f64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ingredient {
    pub name:String,
    #[serde(rename = "ingredientId")]
    pub ingredient_id: Option<String>,
    pub amount: Option<RangeValue>,
    pub unit: String,
    pub prefix:Option<String>,
    pub suffix:Option<String>,
    pub text: Option<String>,
    pub optional: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngredientGroup {
    #[serde(rename = "recipeSection")]
    pub recipe_section:Option<String>,
    #[serde(rename = "ingredientsList")]
    pub ingredients_list:Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Serves {
    pub amount: RangeValue,
    pub unit: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Timings {
    pub qualifier: String,
    #[serde(rename = "durationInMins")]
    pub duration_in_mins:RangeValue,
    pub text:Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instruction {
    pub description: String,
    pub images: Option<Vec<RecipeImage>>
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

#[derive(Serialize, Deserialize)]
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
    #[serde(rename = "previewImage")]
    pub preview_image: Option<RecipeImage>,
    pub contributors: Option<Vec<String>>,
    pub ingredients: Vec<IngredientGroup>,
    pub byline: Option<Vec<String>>,
    #[serde(rename = "suitableForDietIds")]
    pub suitable_for_diet_ids: Option<Vec<String>>,
    #[serde(rename = "cuisineIds")]
    pub cuisine_ids: Option<Vec<String>>,
    #[serde(rename = "mealTypeIds")]
    pub meal_type_ids: Option<Vec<String>>,
    #[serde(rename = "celebrationIds")]
    pub celebration_ids: Option<Vec<String>>,
    #[serde(rename = "utensilsAndApplianceIds")]
    pub utensil_and_appliance_ids: Option<Vec<String>>,
    #[serde(rename = "techniquesUsedIds")]
    pub techniques_used_ids: Option<Vec<String>>,
    #[serde(rename = "difficultyLevel")]
    pub difficulty_level: Option<String>,
    pub serves: Vec<Serves>,
    pub timings: Vec<Timings>,
    pub instructions: Vec<Instruction>,
    #[serde(rename = "bookCredit")]
    pub book_credit: Option<String>,
}

