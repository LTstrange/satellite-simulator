use crate::prelude::*;

use chrono::{DateTime, NaiveDateTime, Utc};

/// Example of SatelliteData:
/// {
///     "OBJECT_NAME":"STARLINK-1008",
///     "OBJECT_ID":"2019-074B",
///     "EPOCH":"2024-10-27T04:10:58.101312",
///     "MEAN_MOTION":15.06400535,
///     "ECCENTRICITY":0.0001539,
///     "INCLINATION":53.0535,
///     "RA_OF_ASC_NODE":264.5418,
///     "ARG_OF_PERICENTER":101.0513,
///     "MEAN_ANOMALY":259.0649,
///     "EPHEMERIS_TYPE":0,
///     "CLASSIFICATION_TYPE":"U",
///     "NORAD_CAT_ID":44714,
///     "ELEMENT_SET_NO":999,
///     "REV_AT_EPOCH":27361,
///     "BSTAR":0.00030439,
///     "MEAN_MOTION_DOT":4.255e-5,
///     "MEAN_MOTION_DDOT":0
/// }
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct RawSatelliteData {
    pub OBJECT_ID: String,
    pub EPOCH: String,
    pub MEAN_MOTION: f32, // (rev/day)
    pub ECCENTRICITY: f32,
    pub INCLINATION: f32,       // (degrees)
    pub RA_OF_ASC_NODE: f32,    // (degrees)
    pub ARG_OF_PERICENTER: f32, // (degrees)
    pub MEAN_ANOMALY: f32,      // (degrees)
}

// Copy from rastro crate (https://github.com/duncaneddy/rastro)
pub fn anomaly_mean_to_true(anm_mean: f32, e: f32) -> Result<f32, String> {
    // Set constants of iteration
    let max_iter = 10;
    let eps = 100.0 * f32::EPSILON; // Convergence with respect to data-type precision

    // Initialize starting iteration values
    let anm_mean = anm_mean % (2.0 * PI);
    let mut anm_ecc = if e < 0.8 { anm_mean } else { PI };

    let mut f = anm_ecc - e * anm_ecc.sin() - anm_mean;
    let mut i = 0;

    // Iterate until convergence
    while f.abs() > eps {
        f = anm_ecc - e * anm_ecc.sin() - anm_mean;
        anm_ecc = anm_ecc - f / (1.0 - e * anm_ecc.cos());

        i += 1;
        if i > max_iter {
            return Err(format!(
                "Reached maximum number of iterations ({}) before convergence for (M: {}, e: {}).",
                max_iter, anm_mean, e
            ));
        }
    }

    // Finish conversion from eccentric to true anomaly
    Ok(anomaly_eccentric_to_true(anm_ecc, e))
}

// Copy from rastro crate (https://github.com/duncaneddy/rastro)
pub fn anomaly_eccentric_to_true(anm_ecc: f32, e: f32) -> f32 {
    (anm_ecc.sin() * (1.0 - e.powi(2)).sqrt()).atan2(anm_ecc.cos() - e)
}

pub fn parse_time_from_str(time_str: &str) -> Result<DateTime<Utc>, String> {
    // 2024-10-27T04:10:58.101312
    let naive_datetime = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S%.6f");
    match naive_datetime {
        Ok(naive_datetime) => Ok(naive_datetime.and_utc()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chrono() {
        let date = "2024-10-27T04:10:58.101312";
        let parsed_time = parse_time_from_str(date).unwrap();
        println!("{}", parsed_time);
    }
}
