// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{ years: f64,}


impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            years:  s as f64 / 31557600 as f64,
        }
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 { d.years / Self::period() }
}

macro_rules! impl_planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n { fn period() -> f64 { $p } }
    }
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);
