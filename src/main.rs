use serde::{Deserialize, Serialize};
use std::fmt;
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
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = read_to_string("recipes.json").unwrap();

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let recipes_json: res_data = serde_json::from_str(&data).unwrap();

    // Do things just like with any other Rust data structure.
    println!(
        "Please call {} at the number {:#?}",
        recipes_json.kind, recipes_json.items[0].snippet.title
    );
}

// impl fmt::Debug for item_data {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Hi")
//     }
// }
