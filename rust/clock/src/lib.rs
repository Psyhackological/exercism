#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

use std::fmt;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Ensures hours and minutes are displayed as two digits with leading zeros if necessary.
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Calculates total of minutes
        let total_minutes = hours * 60 + minutes;
        // Create a constant for 24-hours day in minutes (24 hours * 60 minutes)
        const MINTUES_PER_DAY: i32 = 24 * 60;
        // Wraping the total minutes around a 24-hour day
        let total_minutes = total_minutes.rem_euclid(MINTUES_PER_DAY); // Ensures the time is always positive and within one day
        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    // Adds minutes to the current time, taking into account wrapping around the clock.
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

// Coded just for fun.
use std::ops::{Add, Sub};

impl Add for Clock {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add_minutes(other.hours * 60 + other.minutes)
    }
}

impl Sub for Clock {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.add_minutes(-(other.hours * 60 + other.minutes))
    }
}
