use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RSSFeeds {
    pub rssfeeds: Vec<String>,
}
