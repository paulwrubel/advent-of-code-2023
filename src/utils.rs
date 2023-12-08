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

    let seconds = d.as_secs() % 60;
    if seconds > 0 {
        s.push_str(&format!("{:>.1}s", seconds));
    }

    let milliseconds = (d.as_secs_f64() % 1.0) * 1000.0;
    if milliseconds > 0.0 {
        s.push_str(&format!("{:>.2}ms", milliseconds));
    }

    // let microseconds = d.as_micros() % 1000;
    // if microseconds > 0 {
    //     s.push_str(&format!("{:>.1}us", microseconds));
    // }

    // let nanoseconds = d.as_nanos() % 1000;
    // if nanoseconds > 0 {
    //     s.push_str(&format!("{:>.1}ns", nanoseconds));
    // }

    s
}
