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
        .user_agent("funpay-spec/2.0.0 (drift-detector)")
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

pub fn validate_selector(selector_str: &str) -> Result<(), String> {
    Selector::parse(selector_str)
        .map(|_| ())
        .map_err(|e| format!("Invalid CSS selector '{}': {}", selector_str, e))
}

pub fn check_selector_on_html(html: &str, selector_str: &str) -> Result<usize, String> {
    let sel = Selector::parse(selector_str)
        .map_err(|e| format!("Invalid CSS selector '{}': {}", selector_str, e))?;
    let document = Html::parse_document(html);
    Ok(document.select(&sel).count())
}

pub fn validate_all_selectors(spec: &FunPaySpec) -> Vec<(String, Result<(), String>)> {
    let mut results = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for entity in spec.entities.values() {
        if let Some(fields) = &entity.fields {
            for field in fields.values() {
                if let Some(sel) = &field.selector {
                    if seen.insert(sel.clone()) {
                        let result = validate_selector(sel);
                        results.push((sel.clone(), result));
                    }
                }
            }
        }
    }

    for page in spec.pages.values() {
        if let Some(sel) = &page.list_selector {
            if seen.insert(sel.clone()) {
                let result = validate_selector(sel);
                results.push((sel.clone(), result));
            }
        }
    }

    for check in &spec.drift_detection.critical_selectors {
        if seen.insert(check.selector.clone()) {
            let result = validate_selector(&check.selector);
            results.push((check.selector.clone(), result));
        }
    }

    results
}

pub fn parse_entities_from_html(
    html: &str,
    entity_name: &str,
    spec: &FunPaySpec,
) -> Option<Vec<std::collections::HashMap<String, String>>> {
    let entity = spec.entities.get(entity_name)?;
    let fields = entity.fields.as_ref()?;

    // Determine the item-level selector: use the id/key field's selector
    // or the first field that has a selector
    let item_selector_str = fields.iter().find_map(|(_, f)| f.selector.as_deref())?;

    let item_sel = Selector::parse(item_selector_str).ok()?;
    let document = Html::parse_document(html);

    let mut results = Vec::new();

    // Find all item-level elements
    let item_elements: Vec<_> = document.select(&item_sel).collect();

    for item_el in &item_elements {
        let mut field_map = std::collections::HashMap::new();

        for (field_name, field_def) in fields {
            if let Some(ref sel_str) = field_def.selector {
                // Try to find the field relative to the item element first
                // (for nested selectors), or fall back to searching the whole document
                if let Ok(sel) = Selector::parse(sel_str) {
                    let value = if sel_str == item_selector_str {
                        // This is the item selector itself - extract from the current element
                        if let Some(ref attr) = field_def.attribute {
                            item_el.value().attr(attr).unwrap_or("").to_string()
                        } else {
                            item_el.text().collect::<Vec<_>>().join(" ").trim().to_string()
                        }
                    } else {
                        // Search relative to the item element
                        match item_el.select(&sel).next() {
                            Some(target) => {
                                if let Some(ref attr) = field_def.attribute {
                                    target.value().attr(attr).unwrap_or("").to_string()
                                } else {
                                    target.text().collect::<Vec<_>>().join(" ").trim().to_string()
                                }
                            }
                            None => {
                                // Fallback: search from document root
                                match document.select(&sel).next() {
                                    Some(target) => {
                                        if let Some(ref attr) = field_def.attribute {
                                            target.value().attr(attr).unwrap_or("").to_string()
                                        } else {
                                            target.text().collect::<Vec<_>>().join(" ").trim().to_string()
                                        }
                                    }
                                    None => continue,
                                }
                            }
                        }
                    };
                    field_map.insert(field_name.clone(), value);
                }
            }
        }

        if !field_map.is_empty() {
            results.push(field_map);
        }
    }

    Some(results)
}
