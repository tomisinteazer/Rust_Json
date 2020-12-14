use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct page_info {
    totalResults: u16,
    resultsPerPage: u16,
}
#[derive(Serialize, Deserialize)]
pub struct res_data {
    pub kind: String,
    etag: String,
    nextPageToken: String,
    regionCode: String,
    pageInfo: page_info,
    pub items: Vec<item_data>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct id_data {
    kind: String,
    pub videoId: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct image_data {
    url: String,
    width: u16,
    height: u16,
}
#[derive(Serialize, Deserialize)]
pub struct thumcnail_data {
    default: image_data,
    medium: image_data,
    high: image_data,
}
#[derive(Serialize, Deserialize)]
pub struct snippet_data {
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
pub struct item_data {
    kind: String,
    etag: String,
    pub id: id_data,
    snippet: snippet_data,
}
