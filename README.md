# funpay-spec

Webspec specification for [FunPay.com](https://funpay.com) — a Russian marketplace for game items and services.

This is an **instance** of the [webspec protocol](https://github.com/vitkuz573/webspec-proto), describing how to scrape and interact with FunPay.

## What is webspec?

webspec is a universal protocol for describing web scraping operations. See [webspec-proto](https://github.com/vitkuz573/webspec-proto) for the full specification.

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

## What's Included

- `spec/funpay.yaml` — The webspec spec (entities, pages, types, drift detection)
- Drift detection for monitoring FunPay HTML changes
- Type mappings for Rust, TypeScript, Python, Go, Java

## License

MIT
