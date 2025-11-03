use rssbot::rss::main::read_rss_feed_config_from_json;
fn main() {
    let rssfeeds = read_rss_feed_config_from_json("./rssfeeds.json");
    println!("{}", rssfeeds.);
}
