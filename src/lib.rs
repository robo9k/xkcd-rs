use http::{Request as HttpRequest, Uri};
use once_cell::sync::Lazy;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use time::Date;
use url::Url;

pub type ComicNum = u64;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy)]
pub struct ComicNumber(pub ComicNum);

impl From<u64> for ComicNumber {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<ComicNumber> for u64 {
    fn from(value: ComicNumber) -> Self {
        value.0
    }
}

impl std::str::FromStr for ComicNumber {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u64::from_str(s)?))
    }
}

#[derive(Debug)]
pub struct Comic {
    number: ComicNumber,
    image: Url,
    publication: Date,
    title: String,
    title_safe: String,
    alternate: String,
    link: Option<Url>,
    transcript: String,
    news: String,
}

static BASE_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://xkcd.com").expect("static URL to be parseable"));

impl<'de> Deserialize<'de> for Comic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected};

        let json = JsonComic::deserialize(deserializer)?;

        let number = ComicNumber(json.num);

        let image = BASE_URL
            .join(&json.img)
            .map_err(|_| Error::invalid_value(Unexpected::Str(&json.img), &"a parseable URL"))?;

        let year = json.year.parse::<i32>().map_err(|_| {
            Error::invalid_value(Unexpected::Str(&json.year), &"a signed 32 bit integer")
        })?;
        let month = json
            .month
            .parse::<u8>()
            .map_err(|_| {
                Error::invalid_value(Unexpected::Str(&json.month), &"an usigned 8 bit integer")
            })?
            .try_into()
            .map_err(|_| Error::invalid_value(Unexpected::Other(&json.month), &"a valid month"))?;
        let day = json.day.parse::<u8>().map_err(|_| {
            Error::invalid_value(Unexpected::Str(&json.day), &"an unsigned 8 bit integer")
        })?;

        let publication = Date::from_calendar_date(year, month, day).map_err(|_| {
            Error::invalid_value(
                Unexpected::Other(&json.img),
                &"a valid date from year + month + day",
            )
        })?;

        let link = if json.link.is_empty() {
            None
        } else {
            let url = BASE_URL.join(&json.link).map_err(|_| {
                Error::invalid_value(Unexpected::Str(&json.link), &"a parseable URL")
            })?;

            Some(url)
        };

        Ok(Comic {
            number,
            image,
            publication,
            title: json.title,
            title_safe: json.safe_title,
            alternate: json.alt,
            link,
            transcript: json.transcript,
            news: json.news,
        })
    }
}

impl From<ComicNumber> for Url {
    fn from(value: ComicNumber) -> Self {
        Self::parse(&format!("https://xkcd.com/{}/info.0.json", value.0))
            .expect("typed URL to be parseable")
    }
}

impl From<ComicNumber> for Uri {
    fn from(value: ComicNumber) -> Self {
        format!("https://xkcd.com/{}/info.0.json", value.0)
            .try_into()
            .expect("typed URL to be parseable")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComicId {
    Current,
    Number(ComicNumber),
}

impl<T> From<T> for ComicId
where
    T: Into<ComicNumber>,
{
    fn from(value: T) -> Self {
        Self::Number(value.into())
    }
}

static CURRENT_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://xkcd.com/info.0.json").expect("static URL to be parseable"));

static CURRENT_URI: Lazy<Uri> = Lazy::new(|| {
    "https://xkcd.com/info.0.json"
        .try_into()
        .expect("static URL to be parseable")
});

impl From<ComicId> for Url {
    fn from(value: ComicId) -> Self {
        match value {
            ComicId::Current => CURRENT_URL.clone(),
            ComicId::Number(num) => num.into(),
        }
    }
}

impl From<ComicId> for Uri {
    fn from(value: ComicId) -> Self {
        match value {
            ComicId::Current => CURRENT_URI.clone(),
            ComicId::Number(num) => num.into(),
        }
    }
}

impl From<ComicId> for HttpRequest<hyper::Body> {
    fn from(value: ComicId) -> Self {
        let uri: Uri = value.into();
        HttpRequest::get(uri)
            .body(hyper::Body::empty())
            .expect("HTTP request to be valid")
    }
}

#[derive(Debug)]
pub struct ComicRow {
    pub number: u32,
    pub image: String,
    pub publication: String,
    pub title: String,
    pub title_safe: String,
    pub alternate: String,
    pub link: String,
    pub transcript: String,
    pub news: String,
}

const PUBLICATION_FORMAT_DESCRIPTION: &[time::format_description::FormatItem] =
    time::macros::format_description!("[year]-[month]-[day]");

impl From<Comic> for ComicRow {
    fn from(value: Comic) -> Self {
        let number: u64 = value.number.into();
        let number = number.try_into().expect("comic number should fit into u32");
        let image = value.image.into();
        let publication = value
            .publication
            .format(PUBLICATION_FORMAT_DESCRIPTION)
            .expect("date to be formattable");
        let link = match value.link {
            Some(link) => link.into(),
            None => "".to_string(),
        };
        Self {
            number,
            image,
            publication,
            title: value.title,
            title_safe: value.title_safe,
            alternate: value.alternate,
            link,
            transcript: value.transcript,
            news: value.news,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
