use statrs::distribution::{Normal, ContinuousCDF};

pub fn option_price(
    spot: f64,
    strike: f64,
    risk_free_rate: f64,
    years: f64,
    volatility: f64,
    divident_yield: f64,
) -> f64 {
    // calculating d1 and d2
    let years_sqrt: f64 = years.sqrt();
    let yv: f64 = years_sqrt * volatility;
    let vv: f64 = volatility * volatility/2.0;
    let lnsk: f64 = (spot / strike).ln();
    let rd: f64 = risk_free_rate - divident_yield;
    let d1: f64 = 1.0/yv * (lnsk + (rd + vv) * years);
    let d2: f64 = d1 - yv;

    let rdy_exp: f64 = ((divident_yield - risk_free_rate) * years).exp();

    let norm_dist = Normal::new(0.0, 1.0).expect("error");
    let cdf_d1: f64 = norm_dist.clone().cdf(d1); 
    let cdf_d2: f64 = norm_dist.cdf(d2); 

    cdf_d1 * spot - rdy_exp * cdf_d2 * strike
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}

#[test]
fn it_works() {
    // online calculator used to calculate comparison value https://www.mystockoptions.com/black-scholes.cfm?ticker=&s=31.55&x=22.75&t=3.5&r=5%25&v=50%25&calculate=Calculate
    assert_eq!(round(option_price(31.55, 22.75, 0.05, 3.5, 0.5, 0.0), 3), 16.504);
}
