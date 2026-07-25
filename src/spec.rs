use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct FunPaySpec {
    pub version: String,
    pub name: Option<String>,
    pub protocol: Option<String>,
    pub base_url: String,
    pub types: HashMap<String, TypeMapping>,
    pub transforms: Option<HashMap<String, TransformDef>>,
    pub enums: HashMap<String, EnumDef>,
    pub entities: HashMap<String, EntityDef>,
    pub pages: HashMap<String, PageDef>,
    pub drift_detection: DriftDetection,
    pub auth: AuthDef,
    pub rate_limits: RateLimits,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeMapping {
    pub rust: Option<String>,
    pub typescript: Option<String>,
    pub python: Option<String>,
    pub go: Option<String>,
    pub java: Option<String>,
    pub newtype: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransformDef {
    pub input: String,
    pub output: String,
    pub description: Option<String>,
    pub examples: Option<Vec<TransformExample>>,
    pub map: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransformExample {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumDef {
    pub values: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityDef {
    pub newtype: Option<String>,
    pub fields: Option<HashMap<String, FieldDef>>,
    pub source: Option<String>,
    pub item_selector: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDef {
    pub r#type: String,
    pub selector: Option<String>,
    pub attribute: Option<String>,
    pub transform: Option<String>,
    pub default: Option<serde_yaml::Value>,
    pub nullable: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageDef {
    pub url: Option<String>,
    pub url_pattern: Option<String>,
    pub list_selector: Option<String>,
    pub entity: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DriftDetection {
    pub enabled: bool,
    pub interval: Option<String>,
    pub critical_selectors: Vec<SelectorCheck>,
    pub test_urls: Vec<TestUrl>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectorCheck {
    pub selector: String,
    pub context: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestUrl {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthDef {
    pub r#type: String,
    pub cookie_name: Option<String>,
    pub required_for: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RateLimits {
    pub requests_per_second: Option<f64>,
    pub max_retries: Option<u32>,
    pub backoff_multiplier: Option<f64>,
    pub max_backoff_seconds: Option<u32>,
}

impl FunPaySpec {
    pub fn load(path: &str) -> Result<Self, serde_yaml::Error> {
        let content = std::fs::read_to_string(path).expect("Failed to read spec file");
        serde_yaml::from_str(&content)
    }

    pub fn from_str(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    pub fn entity_names(&self) -> Vec<&str> {
        self.entities.keys().map(|s| s.as_str()).collect()
    }

    pub fn page_names(&self) -> Vec<&str> {
        self.pages.keys().map(|s| s.as_str()).collect()
    }

    pub fn enum_names(&self) -> Vec<&str> {
        self.enums.keys().map(|s| s.as_str()).collect()
    }

    pub fn all_selectors(&self) -> Vec<&str> {
        let mut selectors = Vec::new();
        for entity in self.entities.values() {
            if let Some(fields) = &entity.fields {
                for field in fields.values() {
                    if let Some(sel) = &field.selector {
                        selectors.push(sel.as_str());
                    }
                }
            }
        }
        selectors
    }

    pub fn selectors_with_transforms(&self) -> Vec<(&str, &str)> {
        let mut result = Vec::new();
        for entity in self.entities.values() {
            if let Some(fields) = &entity.fields {
                for field in fields.values() {
                    if let (Some(sel), Some(t)) = (&field.selector, &field.transform) {
                        result.push((sel.as_str(), t.as_str()));
                    }
                }
            }
        }
        result
    }
}
