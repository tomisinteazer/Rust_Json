use std::collections::HashMap;

#[tokio::main]
pub async fn get_test(urlString: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(urlString)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
