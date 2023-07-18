const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! define_and_implement_planet {
    ($planet_name:ident, $orbital_period:literal) => {
        pub struct $planet_name;

        impl Planet for $planet_name {
            fn years_during(d: &Duration) -> f64 {
                d.0 / ($orbital_period * EARTH_YEAR_IN_SECONDS)
            }
        }
    };
}

define_and_implement_planet!(Mercury, 0.2408467);
define_and_implement_planet!(Venus, 0.61519726);
define_and_implement_planet!(Earth, 1.0);
define_and_implement_planet!(Mars, 1.8808158);
define_and_implement_planet!(Jupiter, 11.862615);
define_and_implement_planet!(Saturn, 29.447498);
define_and_implement_planet!(Uranus, 84.016846);
define_and_implement_planet!(Neptune, 164.79132);
