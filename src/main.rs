#[allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;




#[derive(Serialize, Deserialize)]
struct page_info {
    totalResults: u16,
    resultsPerPage: u16,
}
#[derive(Serialize, Deserialize)]
struct res_data {
    kind: String,
    etag: String,
    nextPageToken: String,
    regionCode: String,
    pageInfo: page_info,
    items: Vec<item_data>,
}
#[derive(Serialize, Deserialize, Debug)]
struct id_data {
    kind: String,
    videoId: String,
}
#[derive(Serialize, Deserialize)]
struct image_data {
    url: String,
    width: u16,
    height: u16,
}
#[derive(Serialize, Deserialize)]
struct thumcnail_data {
    default: image_data,
    medium: image_data,
    high: image_data,
}
#[derive(Serialize, Deserialize)]
struct snippet_data {
    publishedAt: String,
    channelId: String,
    title: String,
    description: String,
    thumbnails: thumcnail_data,
    channelTitle: String,
    liveBroadcastContent: String,
    publishTime: String,
}
#[derive(Serialize, Deserialize)]
struct item_data {
    kind: String,
    etag: String,
    id: id_data,
    snippet: snippet_data,
}





fn main() {
    let data = read_to_string("recipes.json").unwrap();
    let recipes_json: res_data = serde_json::from_str(&data).unwrap();
    println!(
        "Please call {} at the number {:#?}",
        recipes_json.kind, recipes_json.items[0].snippet.title
    );
}
