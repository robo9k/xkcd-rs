use bytes::BytesMut;
use http::{Request, Response};
use http_body::Body as _;
use hyper::Client;
use hyper_tls::HttpsConnector;
use sqlx::sqlite::SqlitePool;
use tower::{Service as _, ServiceBuilder, ServiceExt as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

    sqlx::migrate!().run(&pool).await?;

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
    // - convert buffer to `xkcd:Comic` via serde

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

    let comic: xkcd::Comic = serde_json::from_slice(&bytes)?;
    println!("{:#?}", comic);

    let comic_row: xkcd::ComicRow = comic.into();

    let mut conn = pool.acquire().await?;

    sqlx::query!(r#"
INSERT OR REPLACE INTO comics (number, image, publication, title, title_safe, alternate, link, transcript, news)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);
    "#,
        comic_row.number,
        comic_row.image,
        comic_row.publication,
        comic_row.title,
        comic_row.title_safe,
        comic_row.alternate,
        comic_row.link,
        comic_row.transcript,
        comic_row.news,
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}
