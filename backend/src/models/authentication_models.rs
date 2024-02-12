#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CookieSession {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PaypalCookieDetails {
    pub order_id: String,
    pub email: String,
}