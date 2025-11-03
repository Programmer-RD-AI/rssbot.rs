use crate::models::RSSFeeds;
use std::fs::File;
use std::io::copy;
use std::io::stdout;

// TODO: Cache
pub fn read_rss_feed_config_from_json(path: &str) -> RSSFeeds {
    let mut file = File::open(path).unwrap();
    let mut stdout = stdout();
    let file_content = &copy(&mut file, &mut stdout).unwrap().to_string();
    let rssfeeds: RSSFeeds = serde_json::from_str(&file_content).unwrap();
    rssfeeds
}
