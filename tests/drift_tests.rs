use funpay_spec::{FunPaySpec, validate_selector, check_selector_on_html, validate_all_selectors, parse_entities_from_html};

const SPEC_YAML: &str = include_str!("../spec/funpay.yaml");

fn load_spec() -> FunPaySpec {
    FunPaySpec::from_str(SPEC_YAML).unwrap()
}

// ─── Spec Loading Tests ───

#[test]
fn test_spec_loads() {
    let spec = load_spec();
    assert_eq!(spec.version, "2.0");
    assert_eq!(spec.name.as_deref(), Some("FunPay"));
    assert_eq!(spec.protocol.as_deref(), Some("webspec"));
}

#[test]
fn test_spec_has_all_entities() {
    let spec = load_spec();
    let expected = vec![
        "Game", "Category", "SubCategory", "GameServer", "Offer",
        "OfferLot", "User", "LotItem", "Seller", "Lot", "Order",
        "Transaction", "Chat", "ChatMessage", "Review", "Notification",
        "Search", "Settings", "GameCategory",
    ];
    for name in expected {
        assert!(
            spec.entities.contains_key(name),
            "Missing entity: {}",
            name
        );
    }
}

#[test]
fn test_entity_count() {
    let spec = load_spec();
    assert!(
        spec.entities.len() >= 14,
        "Expected at least 14 entities, got {}",
        spec.entities.len()
    );
}

#[test]
fn test_spec_has_all_pages() {
    let spec = load_spec();
    let expected = vec![
        "game_list", "category_offers", "user_profile", "user_offers",
        "seller_profile", "orders", "chats", "chat_messages", "search",
        "notifications", "settings", "transactions", "reviews",
        "game_servers", "subcategories",
    ];
    for name in expected {
        assert!(
            spec.pages.contains_key(name),
            "Missing page: {}",
            name
        );
    }
}

#[test]
fn test_page_count() {
    let spec = load_spec();
    assert!(
        spec.pages.len() >= 13,
        "Expected at least 13 pages, got {}",
        spec.pages.len()
    );
}

#[test]
fn test_spec_has_drift_config() {
    let spec = load_spec();
    assert!(spec.drift_detection.enabled);
    assert!(!spec.drift_detection.critical_selectors.is_empty());
    assert!(!spec.drift_detection.test_urls.is_empty());
    assert!(
        spec.drift_detection.critical_selectors.len() >= 10,
        "Expected at least 10 critical selectors"
    );
}

#[test]
fn test_spec_has_transforms() {
    let spec = load_spec();
    let transforms = spec.transforms.as_ref().expect("transforms missing");
    assert!(transforms.contains_key("parse_price"));
    assert!(transforms.contains_key("parse_date"));
    assert!(transforms.contains_key("parse_id_from_url"));
    assert!(transforms.contains_key("parse_online_status"));
    assert!(transforms.contains_key("parse_balance"));
}

#[test]
fn test_spec_has_enums() {
    let spec = load_spec();
    let expected = vec![
        "LotSaleType", "OfferOrderType", "ServerTagType", "OrderStatus",
        "OnlineStatus", "NotificationType", "TransactionType", "ChatType", "SearchSort",
    ];
    for name in expected {
        assert!(spec.enums.contains_key(name), "Missing enum: {}", name);
    }
}

#[test]
fn test_entity_names_method() {
    let spec = load_spec();
    let names = spec.entity_names();
    assert!(names.contains(&"Offer"));
    assert!(names.contains(&"Transaction"));
    assert!(names.contains(&"Notification"));
    assert!(names.len() >= 14);
}

#[test]
fn test_page_names_method() {
    let spec = load_spec();
    let names = spec.page_names();
    assert!(names.contains(&"search"));
    assert!(names.contains(&"notifications"));
    assert!(names.contains(&"transactions"));
}

#[test]
fn test_rate_limits() {
    let spec = load_spec();
    assert_eq!(spec.rate_limits.requests_per_second, Some(2.0));
    assert_eq!(spec.rate_limits.max_retries, Some(3));
    assert_eq!(spec.rate_limits.backoff_multiplier, Some(2.0));
    assert_eq!(spec.rate_limits.max_backoff_seconds, Some(60));
}

#[test]
fn test_auth_config() {
    let spec = load_spec();
    assert_eq!(spec.auth.r#type, "cookie");
    assert_eq!(spec.auth.cookie_name.as_deref(), Some("golden_key"));
    assert!(spec.auth.required_for.len() >= 5);
}

// ─── Selector Validation Tests ───

#[test]
fn test_all_spec_selectors_are_valid() {
    let spec = load_spec();
    let results = validate_all_selectors(&spec);
    let failures: Vec<_> = results
        .iter()
        .filter(|(_, r)| r.is_err())
        .collect();
    assert!(
        failures.is_empty(),
        "Invalid selectors found:\n{}",
        failures
            .iter()
            .map(|(s, e)| format!("  {}: {:?}", s, e))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn test_valid_selectors() {
    assert!(validate_selector(".tc-item").is_ok());
    assert!(validate_selector(".profile-title").is_ok());
    assert!(validate_selector("[data-order]").is_ok());
    assert!(validate_selector(".tc-price .currency").is_ok());
}

#[test]
fn test_invalid_selector_syntax() {
    assert!(validate_selector("...").is_err());
    assert!(validate_selector("[").is_err());
}

#[test]
fn test_all_selector_attribute_pairs_unique_per_entity() {
    let spec = load_spec();
    for (entity_name, entity) in &spec.entities {
        if let Some(fields) = &entity.fields {
            let mut seen = std::collections::HashSet::new();
            for (field_name, field) in fields {
                if let Some(sel) = &field.selector {
                    let key = format!("{}|{}", sel, field.attribute.as_deref().unwrap_or(""));
                    assert!(
                        seen.insert(key.clone()),
                        "Duplicate selector+attribute '{}' in {}.{}",
                        key,
                        entity_name,
                        field_name
                    );
                }
            }
        }
    }
}

// ─── HTML Fixture Tests ───

#[test]
fn test_parse_game_list() {
    let html = include_str!("fixtures/game_list.html");
    let spec = load_spec();
    let games = parse_entities_from_html(html, "Game", &spec).unwrap();
    assert_eq!(games.len(), 3);
    assert_eq!(games[0].get("id").unwrap(), "123");
    assert_eq!(games[0].get("title").unwrap(), "Dota 2");
    assert_eq!(games[1].get("title").unwrap(), "CS2");
}

#[test]
fn test_parse_category_offers() {
    let html = include_str!("fixtures/category_offers.html");
    let spec = load_spec();
    let offers = parse_entities_from_html(html, "Offer", &spec).unwrap();
    assert_eq!(offers.len(), 3);
    assert_eq!(offers[0].get("id").unwrap(), "111111");
    assert_eq!(offers[0].get("seller_id").unwrap(), "22222");
}

#[test]
fn test_parse_user_profile() {
    let html = include_str!("fixtures/user_profile.html");
    let spec = load_spec();
    let users = parse_entities_from_html(html, "User", &spec).unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].get("username").unwrap(), "GameMaster_Pro");
}

#[test]
fn test_parse_seller_profile() {
    let html = include_str!("fixtures/seller_profile.html");
    let spec = load_spec();
    let sellers = parse_entities_from_html(html, "Seller", &spec).unwrap();
    assert_eq!(sellers.len(), 1);
    assert_eq!(sellers[0].get("name").unwrap(), "GameMaster_Pro");
    assert_eq!(sellers[0].get("rating").unwrap(), "4.8");
    assert_eq!(sellers[0].get("reviews_count").unwrap(), "127");
}

#[test]
fn test_parse_orders() {
    let html = include_str!("fixtures/orders.html");
    let spec = load_spec();
    let orders = parse_entities_from_html(html, "Order", &spec).unwrap();
    assert_eq!(orders.len(), 2);
    assert_eq!(orders[0].get("order_id").unwrap(), "ORD-001");
    assert_eq!(orders[0].get("status").unwrap(), "completed");
}

#[test]
fn test_parse_chats() {
    let html = include_str!("fixtures/chats.html");
    let spec = load_spec();
    let chats = parse_entities_from_html(html, "Chat", &spec).unwrap();
    assert_eq!(chats.len(), 2);
    assert_eq!(chats[0].get("chat_id").unwrap(), "1001");
}

#[test]
fn test_parse_chat_messages() {
    let html = include_str!("fixtures/chat_messages.html");
    let spec = load_spec();
    let messages = parse_entities_from_html(html, "ChatMessage", &spec).unwrap();
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0].get("message_id").unwrap(), "9001");
    assert_eq!(messages[0].get("sender").unwrap(), "22222");
}

#[test]
fn test_parse_reviews() {
    let html = include_str!("fixtures/reviews.html");
    let spec = load_spec();
    let reviews = parse_entities_from_html(html, "Review", &spec).unwrap();
    assert_eq!(reviews.len(), 2);
    assert_eq!(reviews[0].get("review_id").unwrap(), "3001");
    assert_eq!(reviews[0].get("rating").unwrap(), "5.0");
}

#[test]
fn test_parse_notifications() {
    let html = include_str!("fixtures/notifications.html");
    let spec = load_spec();
    let notifs = parse_entities_from_html(html, "Notification", &spec).unwrap();
    assert_eq!(notifs.len(), 3);
    assert_eq!(notifs[0].get("id").unwrap(), "5001");
    assert_eq!(notifs[0].get("type").unwrap(), "order");
}

#[test]
fn test_parse_transactions() {
    let html = include_str!("fixtures/transactions.html");
    let spec = load_spec();
    let txns = parse_entities_from_html(html, "Transaction", &spec).unwrap();
    assert_eq!(txns.len(), 3);
    assert_eq!(txns[0].get("id").unwrap(), "TXN-001");
    assert_eq!(txns[0].get("type").unwrap(), "purchase");
}

#[test]
fn test_parse_search_results() {
    let html = include_str!("fixtures/search_results.html");
    let spec = load_spec();
    let offers = parse_entities_from_html(html, "Offer", &spec).unwrap();
    assert_eq!(offers.len(), 2);
}

#[test]
fn test_parse_game_servers() {
    let html = include_str!("fixtures/game_servers.html");
    let spec = load_spec();
    let servers = parse_entities_from_html(html, "GameServer", &spec).unwrap();
    assert_eq!(servers.len(), 3);
    assert_eq!(servers[0].get("id").unwrap(), "2001");
    assert_eq!(servers[0].get("name").unwrap(), "EU West");
    assert_eq!(servers[0].get("platform").unwrap(), "PC");
}

#[test]
fn test_parse_user_lots() {
    let html = include_str!("fixtures/user_lots.html");
    let spec = load_spec();
    let lots = parse_entities_from_html(html, "LotItem", &spec).unwrap();
    assert_eq!(lots.len(), 2);
    assert_eq!(lots[0].get("id").unwrap(), "L-001");
}

#[test]
fn test_parse_lot_items() {
    let html = include_str!("fixtures/lot_items.html");
    let spec = load_spec();
    let items = parse_entities_from_html(html, "LotItem", &spec).unwrap();
    assert!(items.len() >= 3);
    assert_eq!(items[0].get("id").unwrap(), "LI-001");
    assert!(items[0].get("name").unwrap().contains("Керамбит"));
}

#[test]
fn test_check_selector_on_html() {
    let html = include_str!("fixtures/category_offers.html");
    assert_eq!(check_selector_on_html(html, ".tc-item").unwrap(), 3);
    assert_eq!(check_selector_on_html(html, ".tc-price").unwrap(), 3);
}

#[test]
fn test_selector_not_found_returns_zero() {
    let html = include_str!("fixtures/category_offers.html");
    assert_eq!(check_selector_on_html(html, ".nonexistent").unwrap(), 0);
}

// ─── Transform Definition Tests ───

#[test]
fn test_transform_examples_present() {
    let spec = load_spec();
    let transforms = spec.transforms.as_ref().unwrap();

    let price_transform = transforms.get("parse_price").unwrap();
    assert!(price_transform.examples.is_some());
    let examples = price_transform.examples.as_ref().unwrap();
    assert!(!examples.is_empty());
    assert_eq!(examples[0].input, "1 234.56 ₽");
    assert_eq!(examples[0].output, "1234.56");
}

#[test]
fn test_transform_map_present() {
    let spec = load_spec();
    let transforms = spec.transforms.as_ref().unwrap();
    let online_transform = transforms.get("parse_online_status").unwrap();
    assert!(online_transform.map.is_some());
    let map = online_transform.map.as_ref().unwrap();
    assert_eq!(map.get("online").unwrap(), "online");
    assert_eq!(map.get("offline").unwrap(), "offline");
}

// ─── Field Description Tests ───

#[test]
fn test_entity_fields_have_descriptions() {
    let spec = load_spec();
    let offer = spec.entities.get("Offer").unwrap();
    let fields = offer.fields.as_ref().unwrap();
    for (name, field) in fields {
        assert!(
            field.description.is_some(),
            "Offer.{} missing description",
            name
        );
    }
}

#[test]
fn test_all_entities_have_selectors() {
    let spec = load_spec();
    let mut missing = Vec::new();
    for (entity_name, entity) in &spec.entities {
        if let Some(fields) = &entity.fields {
            for (field_name, field) in fields {
                if field.selector.is_none() && field.description.is_some() {
                    missing.push(format!("{}.{}", entity_name, field_name));
                }
            }
        }
    }
    // Some fields legitimately have no selector (e.g., game_id set programmatically)
    // Just ensure most fields have selectors
    assert!(
        missing.len() < 5,
        "Too many fields without selectors: {:?}",
        missing
    );
}

// ─── Drift Detection Config Tests ───

#[test]
fn test_drift_test_urls_comprehensive() {
    let spec = load_spec();
    let urls: Vec<&str> = spec.drift_detection.test_urls.iter().map(|u| u.url.as_str()).collect();
    assert!(urls.contains(&"/lots/"));
    assert!(urls.contains(&"/users/1/"));
    assert!(urls.contains(&"/chats/"));
    assert!(urls.contains(&"/orders/"));
    assert!(urls.contains(&"/notifications/"));
}

#[test]
fn test_critical_selectors_cover_main_pages() {
    let spec = load_spec();
    let contexts: Vec<&str> = spec.drift_detection.critical_selectors
        .iter()
        .filter_map(|c| c.context.as_deref())
        .collect();
    assert!(contexts.contains(&"offer_list"));
    assert!(contexts.contains(&"user_profile"));
    assert!(contexts.contains(&"game_list"));
    assert!(contexts.contains(&"chat_list"));
    assert!(contexts.contains(&"order_list"));
    assert!(contexts.contains(&"notification_list"));
    assert!(contexts.contains(&"search"));
    assert!(contexts.contains(&"review_list"));
}

// ─── Serialization Round-trip Test ───

#[test]
fn test_spec_serialization_roundtrip() {
    let spec = load_spec();
    let yaml = serde_yaml::to_string(&spec).unwrap();
    let deserialized = FunPaySpec::from_str(&yaml).unwrap();
    assert_eq!(deserialized.version, spec.version);
    assert_eq!(deserialized.entities.len(), spec.entities.len());
    assert_eq!(deserialized.pages.len(), spec.pages.len());
}
