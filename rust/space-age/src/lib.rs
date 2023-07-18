const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

// 4. Consider using a macro to avoid repetition in trait implementation:
macro_rules! implement_planet {
    ($planet:ident, $orbital_period:expr) => {
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 / ($orbital_period * EARTH_YEAR_IN_SECONDS)
            }
        }
    };
}

// 1. Understand the Problem: Convert an age in seconds into years on different planets.
// Like this math
implement_planet!(Mercury, 0.2408467);
implement_planet!(Venus, 0.61519726);
implement_planet!(Earth, 1.0);
implement_planet!(Mars, 1.8808158);
implement_planet!(Jupiter, 11.862615);
implement_planet!(Saturn, 29.447498);
implement_planet!(Uranus, 84.016846);
implement_planet!(Neptune, 164.79132);

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

// 2. Define a Trait `Planet`:
pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

// 3. Create structs for each planet. e.g.:
// This was already created so I leave it as it is
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;
