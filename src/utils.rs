pub fn integers_from_string(input: &str, delim: &str) -> Vec<i32> {
    input
        .split(delim)
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

// pub fn integers_from_string
