use chrono::{DateTime, Datelike, NaiveDate, Utc};

/// Converts a given DateTime to the day of the year.
///
/// # Arguments
/// * `date` - A DateTime input.
///
/// # Returns
/// * A Result that is either:
///   - Ok(u32): the day of the year as an u32 if the input date is valid.
///   - Err(String): an error string indicating what went wrong (e.g., invalid date format).
///
pub fn day_of_year(date: &DateTime<Utc>) -> Result<u32, String> {
    // Get the day of the year
    Ok(date.ordinal())
}

/// Converts a given date (in the format yyyy-mm-dd) to the day of the year.
///
/// # Arguments
/// * `date_str` - A string slice that holds the date in the format "yyyy-mm-dd".
///
/// # Returns
/// * A Result that is either:
///   - Ok(u32): the day of the year as a u32 if the input date is valid.
///   - Err(String): an error string indicating what went wrong (e.g., invalid date format).
///
pub fn day_of_year_str(date_str: &str) -> Result<u32, String> {
    // Parse the date string to a NaiveDate
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| "Invalid date format".to_string())?;

    // Get the day of the year
    Ok(naive_date.ordinal())
}

mod tests {
    use super::*;

    #[test]
    fn test_day_of_year() {
        let naive_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let day_of_year = day_of_year(&DateTime::from_naive_utc_and_offset(naive_datetime, Utc)).unwrap();
        assert_eq!(day_of_year, 1);
    }

    #[test]
    fn test_day_of_year_leap_year() {
        let naive_date = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let day_of_year = day_of_year(&DateTime::from_naive_utc_and_offset(naive_datetime, Utc)).unwrap();
        assert_eq!(day_of_year, 60);
    }

    #[test]
    fn test_day_of_year_str() {
        let day_of_year = day_of_year_str("2023-01-01").unwrap();
        assert_eq!(day_of_year, 1);
    }

    #[test]
    fn test_day_of_year_str_invalid_format() {
        assert_eq!(day_of_year_str("2023-01-32"), Err("Invalid date format".to_string()));
    }
}