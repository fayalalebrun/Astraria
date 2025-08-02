/// Physical constants and unit conversions for astronomical simulations
/// Ported from the original Java Units.java

/// Gravitational constant in SI units (m³ kg⁻¹ s⁻²)
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67408e-11;

/// One Astronomical Unit in meters
pub const AU_TO_METERS: f64 = 149_597_870_691.0;

/// Conversion from meters to astronomical units
pub fn meters_to_au(meters: f64) -> f64 {
    meters / AU_TO_METERS
}

/// Conversion from astronomical units to meters
pub fn au_to_meters(au: f64) -> f64 {
    au * AU_TO_METERS
}

/// Conversion from meters to kilometers
pub fn meters_to_km(meters: f64) -> f64 {
    meters / 1000.0
}

/// Conversion from kilometers to meters
pub fn km_to_meters(km: f64) -> f64 {
    km * 1000.0
}

/// Additional astronomical constants for realistic simulations

/// Solar mass in kilograms
pub const SOLAR_MASS: f64 = 1.989e30;

/// Earth mass in kilograms
pub const EARTH_MASS: f64 = 5.972e24;

/// Jupiter mass in kilograms
pub const JUPITER_MASS: f64 = 1.898e27;

/// Solar radius in meters
pub const SOLAR_RADIUS: f64 = 6.96e8;

/// Earth radius in meters
pub const EARTH_RADIUS: f64 = 6.371e6;

/// Seconds per day
pub const SECONDS_PER_DAY: f64 = 86400.0;

/// Seconds per year (approximate)
pub const SECONDS_PER_YEAR: f64 = 31_557_600.0;

/// Speed of light in m/s
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Conversion utilities for time scales
pub mod time {
    use super::*;

    /// Convert seconds to days
    pub fn seconds_to_days(seconds: f64) -> f64 {
        seconds / SECONDS_PER_DAY
    }

    /// Convert days to seconds
    pub fn days_to_seconds(days: f64) -> f64 {
        days * SECONDS_PER_DAY
    }

    /// Convert seconds to years
    pub fn seconds_to_years(seconds: f64) -> f64 {
        seconds / SECONDS_PER_YEAR
    }

    /// Convert years to seconds
    pub fn years_to_seconds(years: f64) -> f64 {
        years * SECONDS_PER_YEAR
    }
}

/// Conversion utilities for mass
pub mod mass {
    use super::*;

    /// Convert mass to solar masses
    pub fn kg_to_solar_masses(kg: f64) -> f64 {
        kg / SOLAR_MASS
    }

    /// Convert solar masses to kg
    pub fn solar_masses_to_kg(solar_masses: f64) -> f64 {
        solar_masses * SOLAR_MASS
    }

    /// Convert mass to Earth masses
    pub fn kg_to_earth_masses(kg: f64) -> f64 {
        kg / EARTH_MASS
    }

    /// Convert Earth masses to kg
    pub fn earth_masses_to_kg(earth_masses: f64) -> f64 {
        earth_masses * EARTH_MASS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_conversions() {
        let meters = 1.0e11;
        let au = meters_to_au(meters);
        let back_to_meters = au_to_meters(au);

        assert!((back_to_meters - meters).abs() < 1e-6);

        let km = meters_to_km(1000.0);
        assert_eq!(km, 1.0);

        let back_to_m = km_to_meters(km);
        assert_eq!(back_to_m, 1000.0);
    }

    #[test]
    fn test_time_conversions() {
        let days = time::seconds_to_days(SECONDS_PER_DAY);
        assert_eq!(days, 1.0);

        let years = time::seconds_to_years(SECONDS_PER_YEAR);
        assert!((years - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mass_conversions() {
        let solar_masses = mass::kg_to_solar_masses(SOLAR_MASS);
        assert!((solar_masses - 1.0).abs() < 1e-10);

        let earth_masses = mass::kg_to_earth_masses(EARTH_MASS);
        assert!((earth_masses - 1.0).abs() < 1e-10);
    }
}
