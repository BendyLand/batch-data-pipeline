use uuid::Uuid;
use chrono::{DateTime, Duration, Utc, Datelike, TimeZone};
use rand::Rng;
use rand::rng;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_datetime() -> DateTime<Utc> {
    let now = Utc::now();
    let current_year = now.date_naive().year();
    let start = Utc.with_ymd_and_hms(current_year, 1, 1, 0, 0, 0)
        .single()
        .expect("Invalid date");

    let end = now + Duration::days(7);
    let seconds_range = end.timestamp() - start.timestamp();

    let mut rng = rng();
    let random_seconds = rng.random_range(0..seconds_range);

    start + Duration::seconds(random_seconds)
}

pub fn round_decimal(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}
