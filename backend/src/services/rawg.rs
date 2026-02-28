use serde::{Deserialize, Serialize};

/// Client for the RAWG Video Games Database API.
/// See https://rawg.io/apidocs
pub struct RawgService {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawgSearchResult {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub released: Option<String>,
    pub background_image: Option<String>,
    pub rating: Option<f64>,
    pub genres: Option<Vec<RawgGenre>>,
    pub platforms: Option<Vec<RawgPlatformWrapper>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawgGenre {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawgPlatformWrapper {
    pub platform: RawgPlatform,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawgPlatform {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawgResponse {
    pub count: i64,
    pub results: Vec<RawgSearchResult>,
}

impl RawgService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn search_games(&self, query: &str) -> Result<RawgResponse, reqwest::Error> {
        let url = format!(
            "https://api.rawg.io/api/games?key={}&search={}&page_size=10",
            self.api_key,
            urlencoding::encode(query),
        );

        let response = self.client.get(&url).send().await?.json::<RawgResponse>().await?;
        Ok(response)
    }

    pub async fn get_game(&self, rawg_id: i32) -> Result<RawgSearchResult, reqwest::Error> {
        let url = format!(
            "https://api.rawg.io/api/games/{}?key={}",
            rawg_id, self.api_key,
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<RawgSearchResult>()
            .await?;
        Ok(response)
    }
}
