#![warn(clippy::pedantic)]

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        #[allow(clippy::cast_precision_loss)]
        Duration(s as f64)
    }
}

pub trait Planet {
    const EARTH_ORBITAL_PERIOD: f64 = 31_557_600.0;
    const COEFFICIENT: f64;

    #[must_use]
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::EARTH_ORBITAL_PERIOD / Self::COEFFICIENT
    }
}

macro_rules! planets {
    [$(($i:ident, $j:expr)),+ $(,)?] => {
        $(
            pub struct $i {}

            impl Planet for $i {
                const COEFFICIENT: f64 = $j;
            }
        )*
    }
}

planets![
    (Mercury, 0.240_846_7_f64),
    (Venus, 0.615_197_26_f64),
    (Earth, 1.0_f64),
    (Mars, 1.880_815_8_f64),
    (Jupiter, 11.862_615_f64),
    (Saturn, 29.447_498_f64),
    (Uranus, 84.016_846_f64),
    (Neptune, 164.79132_f64),
];
