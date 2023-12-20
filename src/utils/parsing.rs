use std::str::FromStr;

pub fn integers_from_string<T: FromStr>(input: &str, delim: &str) -> Vec<T> {
    input
        .split(delim)
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}
