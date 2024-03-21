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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeImage {
    pub url: String,
    #[serde(rename = "mediaId")]
    pub media_id: String,
    #[serde(rename = "cropId")]
    pub crop_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photographer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageType")]
    pub image_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mediaApiUri")]
    pub media_api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayCredit")]
    pub display_credit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]   //deprecated field from Hatch
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
enum Number {
    Integer(i64),
    Float(f64)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RangeValue {
    min: Option<Number>,
    max: Option<Number>
}

// impl Serialize for RangeValue {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
        

//         match &self.min {
//             Some(floatVal)=>{
//                 if floatVal.fract() == 0.0 {  //can serialize as int
//                     &serializer.serialize_i64(*floatVal as i64);
//                 } else {
//                     &serializer.serialize_f64(*floatVal);
//                 }
//             },
//             None=>{}
//         }

//         match &self.max {
//             Some(floatVal)=>{
//                 if floatVal.fract() == 0.0 {  //can serialize as int
//                     serializer.serialize_i64(*floatVal as i64)
//                 } else {
//                     serializer.serialize_f64(*floatVal)
//                 }
//             },
//             None=>serializer.serialize_none()
//         }

//     }
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ingredient {
    pub name:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ingredientId")]
    pub ingredient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<RangeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngredientGroup {
    #[serde(rename = "recipeSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_section:Option<String>,
    #[serde(rename = "ingredientsList")]
    pub ingredients_list:Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Serves {
    pub amount: RangeValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Timings {
    pub qualifier: String,
    #[serde(rename = "durationInMins")]
    pub duration_in_mins:RangeValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text:Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instruction {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<RecipeImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stepNumber")]
    pub step_number:Option<i32>
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composer_id: Option<String>,
    #[serde(rename = "canonicalArticle")]
    pub canonical_article: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isAppReady")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_app_ready: Option<bool>,
    #[serde(rename = "featuredImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<RecipeImage>,
    #[serde(rename = "previewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<RecipeImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<String>>,
    pub ingredients: Vec<IngredientGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byline: Option<Vec<String>>,
    #[serde(rename = "suitableForDietIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitable_for_diet_ids: Option<Vec<String>>,
    #[serde(rename = "cuisineIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cuisine_ids: Option<Vec<String>>,
    #[serde(rename = "mealTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meal_type_ids: Option<Vec<String>>,
    #[serde(rename = "celebrationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebration_ids: Option<Vec<String>>,

    #[serde(rename = "celebrationsIds")]    //This is a misnamed version that has been seen in some data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrations_ids: Option<Vec<String>>,  

    #[serde(rename = "utensilsAndApplianceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utensil_and_appliance_ids: Option<Vec<String>>,
    #[serde(rename = "techniquesUsedIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub techniques_used_ids: Option<Vec<String>>,
    #[serde(rename = "difficultyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty_level: Option<String>,
    pub serves: Vec<Serves>,
    pub timings: Vec<Timings>,
    pub instructions: Vec<Instruction>,
    #[serde(rename = "bookCredit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_credit: Option<String>,
}

