#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let comic_json: serde_json::Value = reqwest::Client::new()
        .get("https://xkcd.com/info.0.json")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", comic_json);
    Ok(())
}
