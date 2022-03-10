use hyper::Client;
use hyper_tls::HttpsConnector;

use bytes::BytesMut;
use http::{Request, Response};
use http_body::Body as _;
use tower::{Service as _, ServiceBuilder, ServiceExt as _};
use tower_http::BoxError;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
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

    let https = HttpsConnector::new();
    let hyper_client = Client::builder().build::<_, _/*hyper::Body*/>(https);

    let mut client = ServiceBuilder::new().service(hyper_client);

    let comic_id: xkcd::ComicId = 1506.into();
    let request: Request<_> = comic_id.into();

    let response: Response<_> = client.ready().await?.call(request).await?;

    let mut body = response.into_body();
    let mut bytes = BytesMut::new();
    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        bytes.extend_from_slice(&chunk[..]);
    }

    let comic_json: xkcd::Comic = serde_json::from_slice(&bytes)?;
    println!("{:#?}", comic_json);

    Ok(())
}
