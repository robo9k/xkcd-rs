pub type ComicNum = u64;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonComic {
    pub alt: String,
    pub day: String,
    pub img: String,
    pub link: String,
    pub month: String,
    pub news: String,
    pub num: ComicNum,
    pub safe_title: String,
    pub title: String,
    pub transcript: String,
    pub year: String,
}

#[derive(Debug)]
pub struct ComicNumber(pub ComicNum);

#[derive(Debug)]
pub struct Comic {
    number: ComicNumber,
    image: url::Url,
    publication: time::Date,
    title: String,
    title_safe: String,
    alternate: String,
    link: Option<url::Url>,
    transcript: String,
    news: String,
}

static BASE_URL: once_cell::sync::Lazy<url::Url> = once_cell::sync::Lazy::new(|| {
    url::Url::parse("https://xkcd.com").expect("static URL to be parseable")
});

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
