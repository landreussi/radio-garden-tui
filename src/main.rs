use api::RadioGardenApi;
use url::Url;

#[tokio::main]
async fn main() {
    // TODO: Check if we need a config file or env vars.
    let url = Url::parse("https://radio.garden/api/ara/content/").unwrap();
    let api = RadioGardenApi::new(url);
    let places = api.list_places().await.unwrap();

    dbg!(places.data.list);
}
