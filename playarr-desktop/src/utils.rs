use std::time::Duration;

pub fn seconds_to_video_duration(seconds: f64) -> String {
    let duration = chrono::Duration::from_std(Duration::from_secs(seconds as u64)).unwrap();
    let seconds_padded = format!("{:02}", duration.num_seconds() % 60);
    let minutes_padded = format!("{:02}", duration.num_minutes() % 60);
    if duration.num_hours() > 0 {
        return format!(
            "{}:{}:{}",
            duration.num_hours(),
            minutes_padded,
            seconds_padded
        );
    }

    format!("{}:{}", duration.num_minutes(), seconds_padded)
}

pub fn season_or_specials_label(season_nr: i64) -> String {
    if season_nr != 0 {
        format!("Season {season_nr}")
    } else {
        "Specials".to_string()
    }
}
