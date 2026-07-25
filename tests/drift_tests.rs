use funpay_spec::FunPaySpec;

#[test]
fn test_spec_loads() {
    let spec = FunPaySpec::from_str(include_str!("../spec/funpay.yaml")).unwrap();
    assert_eq!(spec.version, "1.0");
    assert!(spec.entities.contains_key("Offer"));
    assert!(spec.entities.contains_key("Game"));
    assert!(spec.entities.contains_key("User"));
}

#[test]
fn test_spec_has_all_pages() {
    let spec = FunPaySpec::from_str(include_str!("../spec/funpay.yaml")).unwrap();
    assert!(spec.pages.contains_key("game_list"));
    assert!(spec.pages.contains_key("category_offers"));
    assert!(spec.pages.contains_key("user_profile"));
}

#[test]
fn test_spec_has_drift_config() {
    let spec = FunPaySpec::from_str(include_str!("../spec/funpay.yaml")).unwrap();
    assert!(spec.drift_detection.enabled);
    assert!(!spec.drift_detection.critical_selectors.is_empty());
    assert!(!spec.drift_detection.test_urls.is_empty());
}
