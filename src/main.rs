#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let comic_json: xkcd::Comic = reqwest::Client::new()
        .get("https://xkcd.com/1506/info.0.json")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", comic_json);
    Ok(())
}
