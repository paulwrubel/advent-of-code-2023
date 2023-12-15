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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Default + std::fmt::Debug + Clone + Copy + PartialEq,
{
    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![T::default(); width]; height],
            width,
            height,
        }
    }

    pub fn take_data(self) -> Vec<Vec<T>> {
        self.data
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_within_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width as i64 && y >= 0 && y < self.height as i64
    }

    pub fn get(&self, x: i64, y: i64) -> Option<T> {
        if self.is_within_bounds(x, y) {
            self.data
                .get(y as usize)
                .and_then(|row| row.get(x as usize))
                .copied()
        } else {
            None
        }
    }

    /// Get the value at (x, y) without checking if it is within bounds
    ///
    /// # Panics
    ///
    /// Panics if (x, y) is out of bounds
    ///
    /// I wouldn't recommend using this, but it's here anyways
    pub fn must_get(&self, x: i64, y: i64) -> T {
        self.data[y as usize][x as usize]
    }

    pub fn set(&mut self, x: i64, y: i64, value: T) -> Result<(), String> {
        if self.is_within_bounds(x, y) {
            self.data[y as usize][x as usize] = value;
            Ok(())
        } else {
            Err(format!("({}, {}) is out of bounds", x, y))
        }
    }

    pub fn row(&self, y: usize) -> Option<&Vec<T>> {
        if y < self.height {
            Some(&self.data[y])
        } else {
            None
        }
    }

    /// Gets a column as a vector
    ///
    /// Due to the way the data is stored, an "artificial" column is created to hold the data
    /// which results in the column effectively being a cloned vector
    pub fn build_column(&self, x: usize) -> Option<Vec<T>> {
        if x < self.width {
            Some(self.data.iter().map(|row| row[x]).collect())
        } else {
            None
        }
    }

    pub fn rows_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        self.data.iter().map(|row| row.iter())
    }

    pub fn columns_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        (0..self.width).map(move |x| (0..self.height).map(move |y| &self.data[y][x]))
    }

    pub fn insert_row_at(&mut self, y: usize, row: Vec<T>) -> Result<(), String> {
        if row.len() != self.width {
            return Err(format!(
                "Row must have {} elements, but has {}",
                self.width,
                row.len()
            ));
        }
        self.data.insert(y, row);
        self.height += 1;
        Ok(())
    }

    pub fn insert_column_at(&mut self, x: usize, column: Vec<T>) -> Result<(), String> {
        if column.len() != self.height {
            return Err(format!(
                "Column must have {} elements, but has {}",
                self.height,
                column.len()
            ));
        }
        for y in 0..self.height {
            self.data[y].insert(x, column[y]);
        }
        self.width += 1;
        Ok(())
    }

    pub fn entries(&self) -> impl Iterator<Item = GridEntry<&T>> {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, value)| GridEntry { x, y, value })
        })
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.data.iter().enumerate() {
            for value in row {
                write!(f, "{}", value)?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = String;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = value.len();
        let width = value.get(0).map_or(0, Vec::len);
        for row in &value {
            if row.len() != width {
                return Err("All rows must have the same width".to_string());
            }
        }

        Ok(Self {
            data: value,
            width,
            height,
        })
    }
}

pub struct GridEntry<T> {
    pub x: usize,
    pub y: usize,
    pub value: T,
}
