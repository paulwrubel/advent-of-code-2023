use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    top_left: GridPoint,
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
            top_left: GridPoint { x: 0, y: 0 },
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
            top_left: GridPoint { x: 0, y: 0 },
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

    pub fn top_left(&self) -> &GridPoint {
        &self.top_left
    }

    pub fn set_top_left(&mut self, point: GridPoint) {
        self.top_left = point;
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn expand_right(&mut self, amount: usize) {
        for row in &mut self.data {
            row.resize_with(row.len() + amount, T::default);
        }
        self.width += amount;
    }

    pub fn expand_bottom(&mut self, amount: usize) {
        self.data
            .resize_with(self.data.len() + amount, || vec![T::default(); self.width]);
        self.height += amount;
    }

    pub fn expand_left(&mut self, amount: usize) {
        for row in &mut self.data {
            let mut new_row = vec![T::default(); amount];
            new_row.append(row);
            *row = new_row;
        }

        self.width += amount;
        self.top_left.x -= amount as i64;
    }

    pub fn expand_top(&mut self, amount: usize) {
        let mut new_data = vec![vec![T::default(); self.width]; amount];
        new_data.append(&mut self.data);
        self.data = new_data;

        self.height += amount;
        self.top_left.y -= amount as i64;
    }

    pub fn is_within_bounds(&self, point: &GridPoint) -> bool {
        self.is_within_bounds_x(point.x) && self.is_within_bounds_y(point.y)
    }

    pub fn is_within_bounds_x(&self, x: i64) -> bool {
        x >= self.top_left.x && x < self.top_left.x + self.width as i64
    }

    pub fn is_within_bounds_y(&self, y: i64) -> bool {
        y >= self.top_left.y && y < self.top_left.y + self.height as i64
    }

    pub fn get(&self, point: &GridPoint) -> Option<&T> {
        if self.is_within_bounds(point) {
            let ipoint = point.to_index_point(&self.top_left);
            self.data.get(ipoint.y).and_then(|row| row.get(ipoint.x))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: &GridPoint) -> Option<&mut T> {
        if self.is_within_bounds(point) {
            let ipoint = point.to_index_point(&self.top_left);
            self.data
                .get_mut(ipoint.y)
                .and_then(|row| row.get_mut(ipoint.x))
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
    pub fn must_get(&self, point: &GridPoint) -> &T {
        let ipoint = point.to_index_point(&self.top_left);
        &self.data[ipoint.y][ipoint.x]
    }

    pub fn set(&mut self, point: &GridPoint, value: T) -> Result<(), String> {
        if self.is_within_bounds(point) {
            let ipoint = point.to_index_point(&self.top_left);
            self.data[ipoint.y][ipoint.x] = value;
            Ok(())
        } else {
            Err(format!("{} is out of bounds", point))
        }
    }

    /// Set the value at (x, y), expanding the grid if necessary
    ///
    /// This still disallows setting negative coordinates
    pub fn set_expand(&mut self, point: &GridPoint, value: T) -> Result<(), String> {
        // println!(
        //     "set_expand({}, _), top_left = {}, dimensions = ({}, {})",
        //     point, self.top_left, self.width, self.height
        // );
        if point.x >= self.top_left.x + self.width as i64 {
            let amount = (point.x - (self.top_left.x + self.width as i64) + 1) as usize;
            self.expand_right(amount);
        } else if point.x < self.top_left.x {
            let amount = (self.top_left.x - point.x) as usize;
            self.expand_left(amount);
        }

        if point.y >= self.top_left.y + self.height as i64 {
            let amount = (point.y - (self.top_left.y + self.height as i64) + 1) as usize;
            self.expand_bottom(amount);
        } else if point.y < self.top_left.y {
            let amount = (self.top_left.y - point.y) as usize;
            self.expand_top(amount);
        }

        self.set(point, value)
    }

    pub fn row(&self, y: i64) -> Option<&Vec<T>> {
        if self.is_within_bounds_y(y) {
            let iy = (y - self.top_left.y) as usize;
            Some(&self.data[iy])
        } else {
            None
        }
    }

    /// Gets a column as a vector
    ///
    /// Due to the way the data is stored, an "artificial" column is created to hold the data
    /// which results in the column effectively being a cloned vector
    pub fn build_column(&self, x: i64) -> Option<Vec<&T>> {
        if self.is_within_bounds_x(x) {
            let ix = (x - self.top_left.x) as usize;
            Some(self.data.iter().map(|row| &row[ix]).collect())
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

    pub fn insert_row_at(&mut self, y: i64, row: Vec<T>) -> Result<(), String> {
        if row.len() != self.width {
            return Err(format!(
                "Row must have {} elements, but has {}",
                self.width,
                row.len()
            ));
        }
        if !self.is_within_bounds_y(y as i64) {
            return Err(format!("Row {} is out of bounds", y));
        }

        let iy = (y - self.top_left.y) as usize;
        self.data.insert(iy, row);
        self.height += 1;
        Ok(())
    }

    pub fn insert_column_at(&mut self, x: i64, mut column: Vec<T>) -> Result<(), String> {
        if column.len() != self.height {
            return Err(format!(
                "Column must have {} elements, but has {}",
                self.height,
                column.len()
            ));
        }
        if !self.is_within_bounds_x(x) {
            return Err(format!("Column {} is out of bounds", x));
        }

        for (y, element) in column.drain(..).enumerate() {
            let ix = (x - self.top_left.x) as usize;
            self.data[y].insert(ix, element);
        }
        self.width += 1;
        Ok(())
    }

    pub fn append_in_direction(
        &mut self,
        direction: &CardinalDirection,
        grid: Grid<T>,
    ) -> Result<(), String> {
        match direction {
            CardinalDirection::North => self.append_north(grid),
            CardinalDirection::South => self.append_south(grid),
            CardinalDirection::East => self.append_east(grid),
            CardinalDirection::West => self.append_west(grid),
        }
    }

    fn append_north(&mut self, other: Grid<T>) -> Result<(), String> {
        if self.width != other.width {
            return Err(format!(
                "Cannot append grid of width {} onto grid of width {}",
                other.width, self.width
            ));
        }

        self.height += other.height;
        self.top_left.y -= other.height as i64;
        let mut new_data = other.take_data();
        new_data.append(self.data.as_mut());

        self.data = new_data;

        Ok(())
    }

    fn append_south(&mut self, other: Grid<T>) -> Result<(), String> {
        if self.width != other.width {
            return Err(format!(
                "Cannot append grid of width {} onto grid of width {}",
                other.width, self.width
            ));
        }

        self.height += other.height;
        self.data.append(other.take_data().as_mut());

        Ok(())
    }

    fn append_east(&mut self, other: Grid<T>) -> Result<(), String> {
        if self.height != other.height {
            return Err(format!(
                "Cannot append grid of height {} onto grid of height {}",
                other.height, self.height
            ));
        }

        self.width += other.width;
        let mut other_data = other.take_data();
        for y in 0..self.height {
            self.data[y].append(other_data[y].as_mut());
        }

        Ok(())
    }

    fn append_west(&mut self, other: Grid<T>) -> Result<(), String> {
        if self.height != other.height {
            return Err(format!(
                "Cannot append grid of height {} onto grid of height {}",
                other.height, self.height
            ));
        }

        self.width += other.width;
        self.top_left.x -= other.width as i64;
        let mut other_data = other.take_data();
        for (y, mut other_row) in other_data.drain(..).enumerate() {
            other_row.append(self.data[y].as_mut());
            self.data[y] = other_row;
        }

        Ok(())
    }

    pub fn flood<FV, FF>(
        &mut self,
        point: &GridPoint,
        value: &FV,
        is_floodable: &FF,
    ) -> Result<(), String>
    where
        FV: Fn() -> T,
        FF: Fn(&T) -> bool,
    {
        // check if this location even makes sense
        if !self.is_within_bounds(point) {
            return Ok(());
        }

        // check if we've already flooded this location
        if !is_floodable(self.must_get(point)) {
            return Ok(());
        }

        self.set(point, value())?;

        // flood the rest
        for neighbor in point.orthogonal_neighbors() {
            self.flood(&neighbor, value, is_floodable)?;
        }

        Ok(())
    }

    pub fn map_all<U>(&mut self, mut map_fn: impl FnMut(&T) -> U) -> Result<Grid<U>, String>
    where
        U: Default + std::fmt::Debug + Clone + PartialEq,
    {
        let mut new_data = vec![vec![U::default(); self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let new_value = map_fn(&self.data[y][x]);
                new_data[y][x] = new_value;
            }
        }
        Ok(Grid {
            top_left: self.top_left,
            data: new_data,
            width: self.width,
            height: self.height,
        })
    }

    pub fn set_all_matching(
        &mut self,
        predicate: impl FnMut(&T) -> bool,
        new_element: T,
    ) -> Result<(), String> {
        let matches = self
            .entries_matching(predicate)
            .map(|entry| entry.point)
            .collect_vec();
        for point in matches {
            self.set(&point, new_element.clone())?;
        }
        Ok(())
    }

    pub fn find(&self, predicate: impl Fn(&T) -> bool) -> Option<GridEntry<&T>> {
        for entry in self.entries() {
            if predicate(entry.value) {
                return Some(entry);
            }
        }
        None
    }

    pub fn num_matching(&self, predicate: impl Fn(&T) -> bool) -> usize {
        self.entries_matching(predicate).count()
    }

    pub fn entries_matching(
        &self,
        mut predicate: impl FnMut(&T) -> bool,
    ) -> impl Iterator<Item = GridEntry<&T>> {
        self.entries().filter(move |entry| predicate(entry.value))
    }

    pub fn entries(&self) -> impl Iterator<Item = GridEntry<&T>> {
        self.data.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, value)| GridEntry {
                point: IndexPoint::from((x, y)).to_grid_point(&self.top_left),
                value,
            })
        })
    }

    pub fn astar_distance_othogonal(
        &self,
        start: &GridPoint,
        end: &GridPoint,
        traversable: impl Fn(&T) -> bool,
    ) -> Option<u64> {
        let successors = |point: &GridPoint| {
            point
                .orthogonal_neighbors()
                .into_iter()
                .filter_map(|neighbor| {
                    // if we're off the map or not traversable, we can't move there
                    if !self.is_within_bounds(&neighbor) || !traversable(self.must_get(&neighbor)) {
                        return None;
                    }
                    Some((neighbor, 1))
                })
        };
        let heuristic = |point: &GridPoint| start.manhattan_distance_to(point);
        let success = |point: &GridPoint| point == end;

        pathfinding::directed::astar::astar(start, successors, heuristic, success)
            .map(|(_, distance)| distance)
    }

    /// Compute the shortest distance between two points using A*
    ///
    /// This assumes that the cache was used for previous calls to `astar_distance_othogonal_with_cache`
    /// using the same parameters except the starting point
    pub fn astar_distance_othogonal_with_cache(
        &self,
        start: &GridPoint,
        end: &GridPoint,
        traversable: impl Fn(&T) -> bool,
        cache: &mut Grid<Option<u64>>,
    ) -> Option<u64> {
        let successors = |point: &GridPoint| {
            point
                .orthogonal_neighbors()
                .into_iter()
                .filter_map(|neighbor| {
                    // if we're off the map or not traversable, we can't move there
                    if !self.is_within_bounds(&neighbor) || !traversable(self.must_get(&neighbor)) {
                        return None;
                    }

                    if let Some(cached_cost) = cache.get(&neighbor).unwrap_or(&None) {
                        Some((end.clone(), *cached_cost + 1))
                    } else {
                        Some((neighbor, 1))
                    }
                })
        };
        let heuristic = |point: &GridPoint| start.manhattan_distance_to(point);
        let success = |point: &GridPoint| point == end;

        let result = pathfinding::directed::astar::astar(start, successors, heuristic, success);

        match result {
            Some((path, distance)) => {
                for (i, point) in path.into_iter().enumerate() {
                    cache
                        .set(&point, Some(distance - i as u64))
                        .expect("failed to set cache");
                }
                Some(distance)
            }
            None => None,
        }
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
            top_left: GridPoint { x: 0, y: 0 },
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
struct IndexPoint {
    x: usize,
    y: usize,
}

impl IndexPoint {
    fn to_grid_point(&self, top_left: &GridPoint) -> GridPoint {
        GridPoint {
            x: (self.x as i64) + top_left.x,
            y: (self.y as i64) + top_left.y,
        }
    }
}

impl From<(usize, usize)> for IndexPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPoint {
    pub x: i64,
    pub y: i64,
}

impl GridPoint {
    fn to_index_point(&self, top_left: &GridPoint) -> IndexPoint {
        IndexPoint {
            x: (self.x - top_left.x) as usize,
            y: (self.y - top_left.y) as usize,
        }
    }

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

    pub fn manhattan_distance_to(&self, other: &GridPoint) -> u64 {
        (self.x - other.x).abs() as u64 + (self.y - other.y).abs() as u64
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
    South,
    East,
    West,
}

impl CardinalDirection {
    pub fn all() -> [Self; 4] {
        [
            CardinalDirection::North,
            CardinalDirection::South,
            CardinalDirection::East,
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

    pub fn relative_direction_to(&self, other: &CardinalDirection) -> RelativeDirection {
        match (self, other) {
            (CardinalDirection::North, CardinalDirection::North) => RelativeDirection::Forward,
            (CardinalDirection::North, CardinalDirection::South) => RelativeDirection::Backward,
            (CardinalDirection::North, CardinalDirection::East) => RelativeDirection::Right,
            (CardinalDirection::North, CardinalDirection::West) => RelativeDirection::Left,
            (CardinalDirection::South, CardinalDirection::North) => RelativeDirection::Backward,
            (CardinalDirection::South, CardinalDirection::South) => RelativeDirection::Forward,
            (CardinalDirection::South, CardinalDirection::East) => RelativeDirection::Left,
            (CardinalDirection::South, CardinalDirection::West) => RelativeDirection::Right,
            (CardinalDirection::East, CardinalDirection::North) => RelativeDirection::Left,
            (CardinalDirection::East, CardinalDirection::South) => RelativeDirection::Right,
            (CardinalDirection::East, CardinalDirection::East) => RelativeDirection::Forward,
            (CardinalDirection::East, CardinalDirection::West) => RelativeDirection::Backward,
            (CardinalDirection::West, CardinalDirection::North) => RelativeDirection::Right,
            (CardinalDirection::West, CardinalDirection::South) => RelativeDirection::Left,
            (CardinalDirection::West, CardinalDirection::East) => RelativeDirection::Backward,
            (CardinalDirection::West, CardinalDirection::West) => RelativeDirection::Forward,
        }
    }

    pub fn halfway_to(&self, other: &CardinalDirection) -> Result<OrdinalDirection, String> {
        match (self, other) {
            (CardinalDirection::North, CardinalDirection::East)
            | (CardinalDirection::East, CardinalDirection::North) => {
                Ok(OrdinalDirection::NorthEast)
            }
            (CardinalDirection::South, CardinalDirection::East)
            | (CardinalDirection::East, CardinalDirection::South) => {
                Ok(OrdinalDirection::SouthEast)
            }
            (CardinalDirection::South, CardinalDirection::West)
            | (CardinalDirection::West, CardinalDirection::South) => {
                Ok(OrdinalDirection::SouthWest)
            }
            (CardinalDirection::North, CardinalDirection::West)
            | (CardinalDirection::West, CardinalDirection::North) => {
                Ok(OrdinalDirection::NorthWest)
            }
            _ => Err(format!(
                "cardinal directions {} and {} are opposite",
                self, other
            )),
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

impl std::fmt::Display for CardinalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardinalDirection::North => write!(f, "north"),
            CardinalDirection::South => write!(f, "south"),
            CardinalDirection::East => write!(f, "east"),
            CardinalDirection::West => write!(f, "west"),
        }
    }
}

pub enum OrdinalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl OrdinalDirection {
    pub fn all() -> [Self; 4] {
        [
            OrdinalDirection::NorthEast,
            OrdinalDirection::SouthEast,
            OrdinalDirection::SouthWest,
            OrdinalDirection::NorthWest,
        ]
    }

    pub fn opposite(&self) -> Self {
        match self {
            OrdinalDirection::NorthEast => OrdinalDirection::SouthWest,
            OrdinalDirection::SouthEast => OrdinalDirection::NorthWest,
            OrdinalDirection::SouthWest => OrdinalDirection::NorthEast,
            OrdinalDirection::NorthWest => OrdinalDirection::SouthEast,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winding {
    Left,
    Right,
}

impl From<Winding> for RelativeDirection {
    fn from(value: Winding) -> Self {
        match value {
            Winding::Left => RelativeDirection::Left,
            Winding::Right => RelativeDirection::Right,
        }
    }
}

impl std::fmt::Display for Winding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Winding::Left => write!(f, "left"),
            Winding::Right => write!(f, "right"),
        }
    }
}
