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
    T: Default + std::fmt::Debug + Clone + PartialEq,
{
    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![T::default(); width]; height],
            width,
            height,
        }
    }

    pub fn from_data(data: Vec<Vec<T>>) -> Result<Self, String> {
        let height = data.len();
        let width = data.get(0).map_or(0, Vec::len);
        for row in &data {
            if row.len() != width {
                return Err("All rows must have the same width".to_string());
            }
        }
        Ok(Self {
            data,
            width,
            height,
        })
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

    pub fn is_within_bounds(&self, point: GridPoint) -> bool {
        let GridPoint { x, y } = point;
        x >= 0 && x < self.width as i64 && y >= 0 && y < self.height as i64
    }

    pub fn get(&self, point: GridPoint) -> Option<&T> {
        if self.is_within_bounds(point) {
            self.data
                .get(point.y as usize)
                .and_then(|row| row.get(point.x as usize))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: GridPoint) -> Option<&mut T> {
        if self.is_within_bounds(point) {
            self.data
                .get_mut(point.y as usize)
                .and_then(|row| row.get_mut(point.x as usize))
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
    pub fn must_get(&self, point: GridPoint) -> &T {
        &self.data[point.y as usize][point.x as usize]
    }

    pub fn set(&mut self, point: GridPoint, value: T) -> Result<(), String> {
        if self.is_within_bounds(point) {
            self.data[point.y as usize][point.x as usize] = value;
            Ok(())
        } else {
            Err(format!("{} is out of bounds", point))
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
    pub fn build_column(&self, x: usize) -> Option<Vec<&T>> {
        if x < self.width {
            Some(self.data.iter().map(|row| &row[x]).collect())
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

    pub fn insert_column_at(&mut self, x: usize, mut column: Vec<T>) -> Result<(), String> {
        if column.len() != self.height {
            return Err(format!(
                "Column must have {} elements, but has {}",
                self.height,
                column.len()
            ));
        }

        for (y, element) in column.drain(..).enumerate() {
            self.data[y].insert(x, element);
        }
        self.width += 1;
        Ok(())
    }

    pub fn entries(&self) -> impl Iterator<Item = GridEntry<&T>> {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, value)| GridEntry {
                point: GridPoint {
                    x: x as i64,
                    y: y as i64,
                },
                value,
            })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridEntry<T> {
    pub point: GridPoint,
    pub value: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPoint {
    pub x: i64,
    pub y: i64,
}

impl GridPoint {
    pub fn direction_to_orthogonal(&self, other: &GridPoint) -> Result<CardinalDirection, String> {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        if x_diff != 0 && y_diff != 0 {
            return Err(format!("Points {} and {} are not orthogonal", self, other));
        } else if x_diff > 0 {
            Ok(CardinalDirection::East)
        } else if x_diff < 0 {
            Ok(CardinalDirection::West)
        } else if y_diff > 0 {
            Ok(CardinalDirection::South)
        } else {
            Ok(CardinalDirection::North)
        }
    }

    pub fn orthogonal_neighbors(&self) -> [GridPoint; 4] {
        [
            (self.x - 1, self.y).into(),
            (self.x + 1, self.y).into(),
            (self.x, self.y - 1).into(),
            (self.x, self.y + 1).into(),
        ]
    }

    pub fn neighbor_in_direction(&self, direction: CardinalDirection) -> GridPoint {
        match direction {
            CardinalDirection::North => (self.x, self.y - 1).into(),
            CardinalDirection::South => (self.x, self.y + 1).into(),
            CardinalDirection::East => (self.x + 1, self.y).into(),
            CardinalDirection::West => (self.x - 1, self.y).into(),
        }
    }

    pub fn neighbor_in_direction_distance(
        &self,
        direction: CardinalDirection,
        distance: i64,
    ) -> GridPoint {
        match direction {
            CardinalDirection::North => (self.x, self.y - distance).into(),
            CardinalDirection::South => (self.x, self.y + distance).into(),
            CardinalDirection::East => (self.x + distance, self.y).into(),
            CardinalDirection::West => (self.x - distance, self.y).into(),
        }
    }

    pub fn points_between_orthogonal_exclusive(&self, other: &GridPoint) -> Vec<GridPoint> {
        let mut points = Vec::new();
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        if x_diff != 0 && y_diff != 0 {
            // not orthogonal!
            return points;
        } else if x_diff != 0 {
            let min = self.x.min(other.x);
            let max = self.x.max(other.x);
            for x in min + 1..max {
                points.push((x, self.y).into());
            }
        } else {
            let min = self.y.min(other.y);
            let max = self.y.max(other.y);
            for y in min + 1..max {
                points.push((self.x, y).into());
            }
        }
        points
    }
}

impl From<(i32, i32)> for GridPoint {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl From<(i64, i64)> for GridPoint {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl From<(u64, u64)> for GridPoint {
    fn from((x, y): (u64, u64)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl From<(usize, usize)> for GridPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl std::fmt::Display for GridPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn all() -> [Self; 4] {
        [
            CardinalDirection::North,
            CardinalDirection::East,
            CardinalDirection::South,
            CardinalDirection::West,
        ]
    }
    pub fn opposite(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::East,
        }
    }

    pub fn turn(&self, relative_direction: RelativeDirection) -> Self {
        match relative_direction {
            RelativeDirection::Forward => *self,
            RelativeDirection::Backward => self.opposite(),
            RelativeDirection::Left => self.turn_left(),
            RelativeDirection::Right => self.turn_right(),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::South,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::West => CardinalDirection::North,
        }
    }
}

impl From<CardinalDirection> for char {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => '^',
            CardinalDirection::East => '>',
            CardinalDirection::South => 'v',
            CardinalDirection::West => '<',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeDirection {
    Forward,
    Backward,
    Left,
    Right,
}
