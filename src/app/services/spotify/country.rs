#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, serde::Deserialize, serde::Serialize,
)]
pub struct Country(pub [char; 2]);
impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char(self.0[0])?;
        f.write_char(self.0[1])
    }
}
