
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExternalIDs {
	#[serde(default)]
	isrc: Option<String>,
	#[serde(default)]
	ean: Option<String>,
	#[serde(default)]
	upc: Option<String>
}
#[cfg(test)]
pub mod tests {
	use super::ExternalIDs;

	#[test]
	pub fn test01() -> Result<(), serde_json::Error> {
		const TEST_INPUT: &'static str = r#"{
    "isrc": "USUM71703861"
  }"#;
		let external_id: ExternalIDs = serde_json::from_str(TEST_INPUT)?;
		println!("{:?}", &external_id);
		Ok(())
	}
}