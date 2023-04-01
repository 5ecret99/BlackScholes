# BlackScholes
```rust
pub fn option_price(
    spot: f64,
    strike: f64,
    risk_free_rate: f64,
    years: f64,
    volatility: f64,
    divident_yield: f64,
) -> f64
```
because of my recent read of the "When genius failed" book i decided to make a simple black_scholes calculator function in rust despite being an already made crate for it (https://docs.rs/black_scholes/latest/black_scholes/) feel free to include my function in your project if you feel is more suitable for your needs.

You can see my simple test by running
```linux
cargo test
```
