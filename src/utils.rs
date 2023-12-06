use std::{str::FromStr, time::Duration};

pub fn integers_from_string<T: FromStr>(input: &str, delim: &str) -> Vec<T> {
    input
        .split(delim)
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}

pub fn format_duration(d: Duration) -> String {
    let mut s = String::new();

    let hours = d.as_secs() / 3600;
    if hours > 0 {
        s.push_str(&format!("{}h", hours));
    }

    let minutes = (d.as_secs() % 3600) / 60;
    if minutes > 0 {
        s.push_str(&format!("{}m", minutes));
    }

    let seconds = d.as_secs_f64() % 60.0;
    if seconds > 0.0 {
        s.push_str(&format!("{:>.1}s", seconds));
    }

    s
}
