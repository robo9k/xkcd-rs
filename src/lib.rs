#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Comic {
    alt: String,
    day: String,
    img: String,
    link: String,
    month: String,
    news: String,
    num: u64,
    safe_title: String,
    title: String,
    transcript: String,
    year: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
