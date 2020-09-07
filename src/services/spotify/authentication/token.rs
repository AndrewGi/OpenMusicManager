#[derive(Clone, Debug)]
pub struct TokenResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub expires_at: std::time::Instant,
}
impl TokenResponse {
    pub fn is_expired(&self) -> bool {
        std::time::Instant::now() > self.expires_at
    }
}
#[derive(Clone, Debug)]
pub struct AccessToken(pub String);
#[derive(Clone, Debug)]
pub struct RefreshToken(pub String);
