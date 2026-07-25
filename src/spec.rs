use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct FunPaySpec {
    pub version: String,
    pub base_url: String,
    pub types: HashMap<String, TypeMapping>,
    pub enums: HashMap<String, EnumDef>,
    pub entities: HashMap<String, EntityDef>,
    pub pages: HashMap<String, PageDef>,
    pub drift_detection: DriftDetection,
    pub auth: AuthDef,
    pub rate_limits: RateLimits,
}

#[derive(Debug, Deserialize)]
pub struct TypeMapping {
    pub rust: Option<String>,
    pub typescript: Option<String>,
    pub python: Option<String>,
    pub go: Option<String>,
    pub java: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EnumDef {
    pub values: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct EntityDef {
    pub newtype: Option<String>,
    pub fields: Option<HashMap<String, FieldDef>>,
    pub source: Option<String>,
    pub item_selector: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FieldDef {
    pub r#type: String,
    pub selector: Option<String>,
    pub attribute: Option<String>,
    pub transform: Option<String>,
    pub default: Option<serde_yaml::Value>,
    pub nullable: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PageDef {
    pub url: Option<String>,
    pub url_pattern: Option<String>,
    pub list_selector: Option<String>,
    pub entity: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DriftDetection {
    pub enabled: bool,
    pub interval: Option<String>,
    pub critical_selectors: Vec<SelectorCheck>,
    pub test_urls: Vec<TestUrl>,
}

#[derive(Debug, Deserialize)]
pub struct SelectorCheck {
    pub selector: String,
    pub context: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TestUrl {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthDef {
    pub r#type: String,
    pub cookie_name: Option<String>,
    pub required_for: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RateLimits {
    pub requests_per_second: Option<f64>,
    pub max_retries: Option<u32>,
}

impl FunPaySpec {
    pub fn load(path: &str) -> Result<Self, serde_yaml::Error> {
        let content = std::fs::read_to_string(path).expect("Failed to read spec file");
        serde_yaml::from_str(&content)
    }

    pub fn from_str(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}
