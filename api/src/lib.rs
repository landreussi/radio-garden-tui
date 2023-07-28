use reqwest::{Client, Result};
use serde::Deserialize;
use url::Url;

pub type Point = [f64; 2];

#[derive(Debug, Deserialize)]
pub struct ListPlacesResponse {
    //api_version: usize,
    //version: String,
    pub data: Data
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub list: Vec<Place>,
    // version: String,
}

#[derive(Debug, Deserialize)]
pub struct Place {
    pub size: u16,
    pub id: String,
    pub geo: Point,
    pub url: String,
    pub country: String,
    pub title: String,
    // boost: bool
}

pub struct RadioGardenApi {
    url: Url,
    client: Client,
}

impl RadioGardenApi {
    pub fn new(url: Url) -> Self {
        Self { url, client: Client::new() }
    }

    pub async fn list_places(&self) -> Result<ListPlacesResponse> {
        let url = self.url.join("secure/places").expect("Could not join API url to path");

        self.client.get(url)
            .send()
            .await?
            .json()
            .await
    }
}
