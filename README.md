# funpay-spec

Language-agnostic specification and drift detection for [FunPay.com](https://funpay.com).

## What is this?

`funpay-spec` defines a machine-readable YAML specification of FunPay.com's HTML structure, enabling:

- **SDK generation** in any language (Rust, TypeScript, Python, Go, Java)
- **Drift detection** when FunPay changes their HTML
- **Type-safe mapping** of FunPay concepts to native types

## The Spec

The specification lives in `spec/funpay.yaml` and defines:

- **Types**: Mapping of abstract types (OfferId, Price, etc.) to language-specific types
- **Entities**: Game, Category, Offer, User — with CSS selectors and transforms
- **Pages**: URL patterns and entity associations
- **Drift Detection**: Critical selectors and test URLs for monitoring
- **Auth**: Cookie-based authentication requirements
- **Rate Limits**: Request throttling configuration

## Usage

### Load the spec

```rust
use funpay_spec::FunPaySpec;

// From file
let spec = FunPaySpec::load("spec/funpay.yaml")?;

// From string
let spec = FunPaySpec::from_str(yaml_content)?;
```

### Run drift detection

```rust
use funpay_spec::{FunPaySpec, check_drift};

let spec = FunPaySpec::load("spec/funpay.yaml")?;
let report = check_drift(&spec, "https://funpay.com").await?;

if report.is_healthy() {
    println!("No drift detected");
} else {
    println!("Drift detected! {}", report.summary());
    for warning in &report.warnings {
        eprintln!("  - {}", warning);
    }
}
```

## Consumers

- [funpay-rs](https://github.com/vitkuz573/funpay-rs) — Rust SDK (first consumer)

## License

MIT
