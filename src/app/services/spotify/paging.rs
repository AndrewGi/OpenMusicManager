#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct PagingObject<T> {
    pub limit: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    pub offset: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub href: String,
    pub total: usize,
    pub items: Vec<T>,
}
