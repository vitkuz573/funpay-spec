# funpay-spec

Webspec specification for [FunPay.com](https://funpay.com) — a Russian marketplace for game items and services.

This is an **instance** of the [webspec protocol](https://github.com/vitkuz573/webspec-proto), describing how to scrape and interact with FunPay.

## What's Here

```
funpay-spec/
├── spec/
│   └── funpay.webspec    # The webspec spec
├── CHANGELOG.md
├── LICENSE
└── README.md
```

This repository contains **only the YAML specification** — no Rust code, no generators. The spec is consumed by [webspec](https://github.com/vitkuz573/webspec) to generate SDKs.

## Spec Coverage

The `spec/funpay.webspec` defines:

| Section | Count |
|---------|-------|
| Entities | 19 |
| Pages | 15 |
| Enums | 9 |
| Transforms | 5 |

### Entities

| Entity | Description |
|--------|-------------|
| `Game` | Game catalog entry |
| `Category` | Game category |
| `SubCategory` | Subcategory within a category |
| `GameServer` | Game server |
| `Offer` | Buy/sell offer |
| `OfferLot` | Individual lot in an offer |
| `User` | User profile |
| `LotItem` | Item in a lot listing |
| `Seller` | Seller information card |
| `Lot` | Combined lot + seller listing |
| `Order` | Purchase order |
| `Transaction` | Financial transaction |
| `Chat` | Chat conversation |
| `ChatMessage` | Individual chat message |
| `Review` | Seller review |
| `Notification` | User notification |
| `Search` | Search results page |
| `Settings` | User settings page |
| `GameCategory` | Category within a game |

## Generate SDK

Use [webspec](https://github.com/vitkuz573/webspec) to generate SDKs from this spec:

```bash
# Generate Rust SDK for funpay-rs
webspec generate \
    --spec spec/funpay.webspec \
    --target rust \
    --output ../funpay-rs/

# Generate TypeScript SDK
webspec generate \
    --spec spec/funpay.webspec \
    --target typescript \
    --output ./ts-sdk/

# Generate Python SDK
webspec generate \
    --spec spec/funpay.webspec \
    --target python \
    --output ./py-sdk/
```

Or use the [funpay-rs](https://github.com/vitkuz573/funpay-rs) build script:

```bash
cd ../funpay-rs && bash build.sh
```

## Validate

```bash
webspec validate --spec spec/funpay.webspec
```

## How This Is Used

1. `funpay-spec` defines the spec (this repo)
2. `webspec` generates code from the spec
3. `funpay-rs` is the generated Rust SDK
4. `funpay-deal-finder` and `funpay-cli` consume `funpay-rs`

## License

MIT
