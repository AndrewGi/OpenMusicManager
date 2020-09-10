#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]

pub struct Followers {
    pub href: String,
    pub total: usize,
}
