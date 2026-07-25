# funpay-spec

Enterprise-grade webspec specification for [FunPay.com](https://funpay.com) — a Russian marketplace for game items and services.

This is an **instance** of the [webspec protocol](https://github.com/vitkuz573/webspec-proto), describing how to scrape and interact with FunPay.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     funpay-spec                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐              │
│  │ spec/    │    │ src/     │    │ tests/   │              │
│  │          │    │          │    │          │              │
│  │ funpay   │───▶│ spec.rs  │───▶│ drift_   │              │
│  │  .yaml   │    │ drift.rs │    │ tests.rs │              │
│  │          │    │ error.rs │    │          │              │
│  └──────────┘    └──────────┘    │fixtures/ │              │
│       │              │           │ *.html   │              │
│       │              │           └──────────┘              │
│       ▼              ▼                                     │
│  ┌──────────────────────────────────┐                      │
│  │         Generated SDKs           │                      │
│  │  ┌─────┐ ┌──────┐ ┌────┐ ┌──┐  │                      │
│  │  │Rust │ │  TS  │ │ Py │ │Go│  │                      │
│  │  └─────┘ └──────┘ └────┘ └──┘  │                      │
│  └──────────────────────────────────┘                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Entity Relationship Diagram

```
┌─────────┐     ┌────────────┐     ┌──────────┐
│  Game   │────▶│  Category  │────▶│  Offer   │
│         │     │            │     │          │
│ id      │     │ id         │     │ id       │
│ title   │     │ title      │     │ seller_id│───┐
│ icon_url│     │ game_id    │     │ price    │   │
│ url     │     │ offers_cnt │     │ server   │   │
└─────────┘     └────────────┘     │ currency │   │
                    │              └──────────┘   │
                    ▼                             │
              ┌──────────────┐     ┌──────────┐  │
              │ SubCategory  │     │  Seller  │◀─┘
              │              │     │          │
              │ id           │     │ user_id  │──────┐
              │ name         │     │ name     │      │
              │ parent_id    │     │ rating   │      │
              └──────────────┘     │ reviews  │      │
                                   │ online   │      │
┌──────────┐                       └──────────┘      │
│  Order   │                              ▲          │
│          │     ┌──────────────┐         │          │
│ order_id │     │   Review     │─────────┘          │
│ status   │     │              │                    │
│ price    │     │ review_id    │    ┌──────────┐   │
│ date     │     │ author       │◀───│   User   │   │
│ seller   │     │ text         │    │          │   │
│ buyer    │     │ rating       │    │ id       │   │
└──────────┘     └──────────────┘    │ username │   │
                                     │ avatar   │   │
┌──────────────┐                      │ online   │   │
│ Transaction  │                      └──────────┘   │
│              │                                     │
│ id           │    ┌──────────────┐                  │
│ type         │    │    Chat      │                  │
│ amount       │    │              │    ┌──────────┐  │
│ date         │    │ chat_id      │    │   Lot    │  │
│ balance_after│    │ user         │───▶│          │  │
└──────────────┘    │ last_message │    │ id       │  │
                    └──────────────┘    │ game     │  │
                            │           │ seller   │◀─┘
                            ▼           │ price    │
                    ┌──────────────┐    └──────────┘
                    │ ChatMessage  │
                    │              │
                    │ message_id   │
                    │ sender       │
                    │ text         │
                    │ date         │
                    └──────────────┘
```

## Field Mapping Table (FunPay HTML → Entity Fields)

### Offer Entity

| Entity Field    | CSS Selector              | HTML Attribute | Transform    | Description                    |
|-----------------|---------------------------|----------------|--------------|--------------------------------|
| `id`            | `.tc-item`                | `data-order`   | —            | Unique offer identifier        |
| `seller_id`     | `.tc-item`                | `data-user-id` | —            | Seller's user ID               |
| `server`        | `.tc-server`              | —              | —            | Game server name               |
| `price`         | `.tc-price`               | —              | `parse_price`| Offer price                    |
| `currency`      | `.tc-price .currency`     | —              | —            | Price currency symbol          |
| `description`   | `.tc-desc-text`           | —              | —            | Seller's description           |
| `sale_type`     | `.tc-item`                | `data-mark`    | —            | single / bulk                  |
| `item_count`    | `.tc-item`                | `data-lot-size`| —            | Number of items in lot         |
| `image_url`     | `.tc-item img`            | `src`          | —            | Offer preview image            |

### User Entity

| Entity Field       | CSS Selector         | HTML Attribute  | Transform     | Description              |
|--------------------|----------------------|-----------------|---------------|--------------------------|
| `id`               | `.profile-user-id`   | `data-user-id`  | —             | Unique user identifier   |
| `username`         | `.profile-title`     | —               | —             | Display username         |
| `avatar_url`       | `.profile-avatar img`| `src`           | —             | Avatar image URL         |
| `status`           | `.user-status`       | —               | —             | Custom status text       |
| `registration_date`| `.profile-regdate`   | —               | `parse_date`  | Account creation date    |
| `online_status`    | `.profile-online`    | —               | —             | Current online status    |

### Seller Entity

| Entity Field  | CSS Selector          | HTML Attribute | Transform | Description           |
|---------------|-----------------------|----------------|-----------|-----------------------|
| `user_id`     | `.seller-info`        | `data-user-id` | —         | Seller's user ID      |
| `name`        | `.seller-name`        | —              | —         | Seller display name   |
| `rating`      | `.seller-rating`      | —              | —         | Seller rating score   |
| `reviews_count`| `.seller-reviews`    | —              | —         | Number of reviews     |
| `avatar_url`  | `.seller-avatar img`  | `src`          | —         | Avatar image URL      |
| `online`      | `.seller-online`      | —              | —         | Online status         |

### Order Entity

| Entity Field | CSS Selector      | HTML Attribute  | Transform     | Description            |
|--------------|-------------------|-----------------|---------------|------------------------|
| `order_id`   | `.order-item`     | `data-order-id` | —             | Unique order ID        |
| `game`       | `.order-game`     | —               | —             | Game name              |
| `status`     | `.order-status`   | —               | —             | Order status           |
| `price`      | `.order-price`    | —               | `parse_price` | Order total price      |
| `date`       | `.order-date`     | —               | `parse_date`  | Order creation date    |

## What's Included

- `spec/funpay.yaml` — The webspec spec (19 entities, 15 pages, 9 enums, 5 transforms)
- Drift detection for monitoring FunPay HTML changes (17 critical selectors)
- Type mappings for Rust, TypeScript, Python, Go, Java
- Sample HTML fixtures for offline testing
- Comprehensive test suite with 40+ tests

## Entity Coverage

| Entity         | Description                          | Key Selectors                  |
|----------------|--------------------------------------|--------------------------------|
| `Game`         | Game catalog entry                   | `.game-title`                  |
| `Category`     | Game category                        | `.category-item`               |
| `SubCategory`  | Subcategory within a category        | `.subcategory-item`            |
| `GameServer`   | Game server                          | `.server-item`                 |
| `Offer`        | Buy/sell offer                       | `.tc-item`                     |
| `OfferLot`     | Individual lot in an offer           | `.tc-item`                     |
| `User`         | User profile                         | `.profile-title`               |
| `LotItem`      | Item in a lot listing                | `.tc-item-text`                |
| `Seller`       | Seller information card              | `.seller-name`                 |
| `Lot`          | Combined lot + seller listing        | `.tc-item` + `.tc-seller`      |
| `Order`        | Purchase order                       | `.order-item`                  |
| `Transaction`  | Financial transaction                | `.transaction-item`            |
| `Chat`         | Chat conversation                    | `.chat-item`                   |
| `ChatMessage`  | Individual chat message              | `.msg`                         |
| `Review`       | Seller review                        | `.review-item`                 |
| `Notification` | User notification                    | `.notification-item`           |
| `Search`       | Search results page                  | `.search-input` + `.tc-item`   |
| `Settings`     | User settings page                   | `.settings-email`              |
| `GameCategory` | Category within a game               | `.category-item`               |

## Generate SDK

```bash
# Using webspec
cargo run --manifest-path ../webspec/Cargo.toml -- generate \
    --spec spec/funpay.yaml \
    --target rust \
    --output ../funpay-rs/

# Or using funpay-rs build script
cd ../funpay-rs && bash build.sh
```

## Validate

```bash
cargo run --manifest-path ../webspec/Cargo.toml -- validate \
    --spec spec/funpay.yaml
```

## Run Tests

```bash
cargo test

# Run drift tests against live FunPay (requires network)
cargo test -- --ignored
```

## Contributing: Updating Selectors

When FunPay changes their HTML structure, selectors may break. Here's how to update:

### 1. Identify the Broken Selector

Run drift detection to find which selectors fail:

```bash
cargo test test_drift_detection_live -- --ignored
```

### 2. Inspect the New HTML

Open the relevant FunPay page in a browser and inspect the element:

1. Open DevTools (F12)
2. Use the element picker (Ctrl+Shift+C)
3. Find the new selector for the element
4. Note any new `data-*` attributes

### 3. Update the Spec

Edit `spec/funpay.yaml`:

```yaml
Offer:
  fields:
    price:
      type: "Price"
      selector: ".new-price-selector"  # ← updated
      transform: "parse_price"
      description: "Offer price"
```

### 4. Update HTML Fixtures

Update the corresponding fixture in `tests/fixtures/`:

```html
<div class="new-price-selector">1 234.56 <span class="currency">₽</span></div>
```

### 5. Run Tests

```bash
cargo test
```

### 6. Submit a PR

1. Create a branch: `git checkout -b fix/selector-price-update`
2. Commit with a descriptive message
3. Push and open a PR

### Selector Best Practices

- Prefer `.class` selectors over `#id` for stability
- Use `data-*` attributes when available (less likely to change)
- Avoid deeply nested selectors like `div > span > a > img`
- Always add a `description` to new fields
- Test against sample HTML fixtures before pushing

## License

MIT
