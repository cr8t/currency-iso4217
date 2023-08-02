# ISO 4217 Currency Codes

Minimalistic implementation of ISO 4217 <https://en.wikipedia.org/wiki/ISO_4217> currency codes.

By default, `no-std` compatible.

### Using with `std`

To enable `std` features, use:

```toml
currency_iso4217 = { version = "x.x", features = ["std"] }
```

### Features

- `std`: use standard library types/functions instead of `core`
- `serde`: enable `serde` de/serialization
- `serde-std`: enable `serde` de/serialization using `std` types/functions
