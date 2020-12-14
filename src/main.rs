#[allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

mod getData;
mod printArrayofStrings;
mod recipeStructure;
mod thetypeof;

fn main() {
    let data = read_to_string("recipes.json").unwrap();

    let recipes_json: recipeStructure::res_data = serde_json::from_str(&data).unwrap();
    let arrayofstrings = vec!["kojo", "jeorge", "alfred"];
    let myArrayofSttings = ["kojo", "jeorge", "alfred"];

    // let vidId = recipes_json.items[0].id.videoId;
    // let opt: Option<String> = Some(vidId.to_owned());
    // let value = opt.as_deref().unwrap_or("default string");
    // let newres = getData::get_test(value);

    println!(
        "Please call {} at the number {}",
        recipes_json.kind,
        thetypeof::type_of(&recipes_json.items[0].id.videoId)
    );
    printArrayofStrings::print_str(myArrayofSttings);
    println!("{}", thetypeof::type_of(myArrayofSttings));
}
