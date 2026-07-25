use crate::spec::FunPaySpec;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct DriftReport {
    pub passed: Vec<SelectorCheckResult>,
    pub failed: Vec<SelectorCheckResult>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SelectorCheckResult {
    pub selector: String,
    pub context: String,
    pub found: bool,
    pub match_count: usize,
}

impl DriftReport {
    pub fn is_healthy(&self) -> bool {
        self.failed.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "Drift check: {} passed, {} failed, {} warnings",
            self.passed.len(),
            self.failed.len(),
            self.warnings.len()
        )
    }
}

pub async fn check_drift(
    spec: &FunPaySpec,
    base_url: &str,
) -> Result<DriftReport, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("funpay-spec/0.1.0 (drift-detector)")
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let mut passed = Vec::new();
    let mut failed = Vec::new();
    let mut warnings = Vec::new();

    for test_url in &spec.drift_detection.test_urls {
        let url = format!("{}{}", base_url, test_url.url);
        match client.get(&url).send().await {
            Ok(resp) => {
                let html = resp.text().await.unwrap_or_default();
                let document = Html::parse_document(&html);

                for check in &spec.drift_detection.critical_selectors {
                    let selector = Selector::parse(&check.selector);
                    match selector {
                        Ok(sel) => {
                            let count = document.select(&sel).count();
                            let result = SelectorCheckResult {
                                selector: check.selector.clone(),
                                context: check.context.clone().unwrap_or_default(),
                                found: count > 0,
                                match_count: count,
                            };
                            if result.found {
                                passed.push(result);
                            } else {
                                failed.push(result);
                                warnings.push(format!(
                                    "CRITICAL: Selector '{}' not found on {} ({})",
                                    check.selector,
                                    test_url.url,
                                    check.description.as_deref().unwrap_or("no description")
                                ));
                            }
                        }
                        Err(e) => {
                            warnings.push(format!("Invalid selector '{}': {}", check.selector, e));
                        }
                    }
                }
            }
            Err(e) => {
                warnings.push(format!("Failed to fetch {}: {}", test_url.url, e));
            }
        }
    }

    Ok(DriftReport {
        passed,
        failed,
        warnings,
    })
}
