# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-07-25

### Added

- **7 new entities**: `Transaction`, `Notification`, `Settings`, `Search`, `SubCategory`, `GameServer`, `GameCategory`
- **7 new pages**: `search`, `notifications`, `settings`, `transactions`, `reviews`, `game_servers`, `subcategories`
- **5 new enums**: `NotificationType`, `TransactionType`, `ChatType`, `SearchSort`, `CategoryId`
- **5 new transforms**: `parse_price`, `parse_date`, `parse_id_from_url`, `parse_online_status`, `parse_balance`
- **5 new types**: `CategoryId`, `ServerId`, `TransactionId`, `NotificationId`, `Email`, `Phone`, `ChatId`, `MessageId`, `ReviewId`, `Balance`
- **15 HTML test fixtures** for offline parser testing
- **40+ unit tests** covering spec loading, selector validation, entity parsing, and serialization
- `validate_all_selectors()` — validates all CSS selectors in spec at once
- `parse_entities_from_html()` — generic entity parser using spec selectors
- `entity_names()`, `page_names()`, `enum_names()` convenience methods
- Field descriptions on all entity fields
- Transform examples and maps
- Comprehensive README with architecture diagram, ERD, field mapping tables, and contributing guide

### Changed

- Upgraded from spec v1.0 to v2.0
- Expanded critical selectors from 8 to 17 for better drift detection coverage
- Expanded test URLs from 2 to 6 for comprehensive monitoring
- Improved rate limiting config with `backoff_multiplier` and `max_backoff_seconds`
- All entity fields now have descriptions
- Spec struct now supports `name`, `protocol`, `transforms`, and `description` fields

### Fixed

- Fixed `data lot-size` typo → `data-lot-size` in Offer entity
- Fixed inconsistent selector patterns across entities

## [1.0.0] - 2026-07-20

### Added

- Initial spec with 12 entities: `Game`, `Category`, `Offer`, `OfferLot`, `User`, `LotItem`, `Seller`, `Lot`, `Order`, `Chat`, `ChatMessage`, `Review`, `GameCategory`
- Basic drift detection with 8 critical selectors
- Type mappings for Rust, TypeScript, Python, Go, Java
- Cookie-based auth configuration
- Basic test suite
