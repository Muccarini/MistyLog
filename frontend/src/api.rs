use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "/api";

/// User info returned by GET /api/auth/me
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
}

/// Game model
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub platform: Option<String>,
    pub release_date: Option<String>,
    pub cover_image_url: Option<String>,
    pub rawg_id: Option<i32>,
    pub avg_rating: Option<f64>,
    pub review_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// Paginated game list
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameList {
    pub games: Vec<Game>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

/// Review with user info
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub rating: i16,
    pub title: Option<String>,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub username: String,
    pub display_name: Option<String>,
}

/// Fetch the current logged-in user (returns None if not authenticated)
pub async fn fetch_me() -> Option<User> {
    let resp = Request::get(&format!("{API_BASE}/auth/me"))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .ok()?;

    if resp.ok() {
        resp.json::<User>().await.ok()
    } else {
        None
    }
}

/// Fetch paginated game list
pub async fn fetch_games(
    page: u64,
    search: Option<&str>,
    genre: Option<&str>,
    platform: Option<&str>,
) -> Result<GameList, String> {
    let mut url = format!("{API_BASE}/games?page={page}&per_page=20");
    if let Some(s) = search {
        url.push_str(&format!("&search={}", urlencoding::encode(s)));
    }
    if let Some(g) = genre {
        url.push_str(&format!("&genre={}", urlencoding::encode(g)));
    }
    if let Some(p) = platform {
        url.push_str(&format!("&platform={}", urlencoding::encode(p)));
    }

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<GameList>().await.map_err(|e| e.to_string())
}

/// Fetch a single game by ID
pub async fn fetch_game(id: i32) -> Result<Game, String> {
    let resp = Request::get(&format!("{API_BASE}/games/{id}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Game>().await.map_err(|e| e.to_string())
}

/// Fetch reviews for a game
pub async fn fetch_reviews(game_id: i32) -> Result<Vec<Review>, String> {
    let resp = Request::get(&format!("{API_BASE}/games/{game_id}/reviews"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<Review>>().await.map_err(|e| e.to_string())
}

/// Submit a review
pub async fn create_review(game_id: i32, rating: i16, title: Option<String>, body: &str) -> Result<(), String> {
    let payload = serde_json::json!({
        "rating": rating,
        "title": title,
        "body": body,
    });

    let resp = Request::post(&format!("{API_BASE}/games/{game_id}/reviews"))
        .credentials(web_sys::RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Failed to submit review: {}", resp.status()))
    }
}

/// Logout — clears server-side session
pub async fn logout() -> Result<String, String> {
    #[derive(Deserialize)]
    struct LogoutResp {
        redirect: String,
    }

    let resp = Request::post(&format!("{API_BASE}/auth/logout"))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = resp.json::<LogoutResp>().await.map_err(|e| e.to_string())?;
    Ok(body.redirect)
}

/// Get the login redirect URL
pub fn login_url() -> String {
    format!("{API_BASE}/auth/login")
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        js_sys::encode_uri_component(s).into()
    }
}
