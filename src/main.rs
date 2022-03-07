#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let comic_json: xkcd::Comic = reqwest::Client::new()
        .get("https://xkcd.com/1506/info.0.json")
        .send()
        .await?
        .json()
        .await?;

    // Abstract TODOs for `tower-http`
    // - construct hyper client with TLS as default impl
    // - create tower client service fn (with decompression layer ?) from hyper client
    // - set `user-agent` HTTP request header
    // - set `accept` HTTP request header (in "comic by id" request only ?) ?
    // - call HTTP client service fn
    // - check HTTP response status ?
    // - check response `content-length` ?
    // - check response `content-type` ?
    // - convert response to BytesMut buffer or such
    // - convert buffer to `xckd:Comic` via serde

    println!("{:#?}", comic_json);
    Ok(())
}
