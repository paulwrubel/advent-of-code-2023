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

    pub fn is_within_bounds(&self, point: GridPoint) -> bool {
        self.is_within_bounds_x(point.x) && self.is_within_bounds_y(point.y)
    }

    pub fn is_within_bounds_x(&self, x: i64) -> bool {
        x >= self.top_left.x && x < self.top_left.x + self.width as i64
    }

    pub fn is_within_bounds_y(&self, y: i64) -> bool {
        y >= self.top_left.y && y < self.top_left.y + self.height as i64
    }

    pub fn get(&self, point: GridPoint) -> Option<&T> {
        if self.is_within_bounds(point) {
            let ipoint = point.to_index_point(&self.top_left);
            self.data.get(ipoint.y).and_then(|row| row.get(ipoint.x))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: GridPoint) -> Option<&mut T> {
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
    pub fn must_get(&self, point: GridPoint) -> &T {
        let ipoint = point.to_index_point(&self.top_left);
        &self.data[ipoint.y][ipoint.x]
    }

    pub fn set(&mut self, point: GridPoint, value: T) -> Result<(), String> {
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
    pub fn set_expand(&mut self, point: GridPoint, value: T) -> Result<(), String> {
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

    pub fn flood<FV, FF>(
        &mut self,
        point: GridPoint,
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
            self.flood(neighbor, value, is_floodable)?;
        }

        Ok(())
    }

    pub fn entries(&self) -> impl Iterator<Item = GridEntry<&T>> {
        self.data.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, value)| GridEntry {
                point: IndexPoint::from((x, y)).to_grid_point(&self.top_left),
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
