use std::fs;

use crate::{
    utils::{Grid, GridPoint},
    AdventError, ExclusivePart,
};

const INPUT_FILE: &str = "./resources/day13_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let patterns = input
        .split("\n\n")
        .map(|pattern_str| Pattern::parse(pattern_str))
        .collect::<Result<Vec<Pattern>, String>>()?;

    let mut lines_of_symmetry = Vec::new();
    for (i, patterns) in patterns.iter().enumerate() {
        let pattern_los = patterns.find_lines_of_symmetry();
        if let Some(line_of_symmetry) = pattern_los.get(0) {
            lines_of_symmetry.push(*line_of_symmetry);
        } else {
            return Err(format!("no line of symmetry found for patterns #{}", i).into());
        }
    }

    let mut summary_num = 0;
    for line_of_symmetry in lines_of_symmetry {
        summary_num += match line_of_symmetry {
            LineOfSymmetry::Horizontal(y) => 100 * y,
            LineOfSymmetry::Vertical(x) => x,
        }
    }

    Ok(summary_num.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let patterns = input
        .split("\n\n")
        .map(|pattern_str| Pattern::parse(pattern_str))
        .collect::<Result<Vec<Pattern>, String>>()?;

    let mut lines_of_symmetry = Vec::new();
    for (i, patterns) in patterns.iter().enumerate() {
        let pattern_los = patterns.find_lines_of_symmetry_with_smudge();
        if let Some(line_of_symmetry) = pattern_los.get(0) {
            lines_of_symmetry.push(*line_of_symmetry);
        } else {
            return Err(format!("no smudged line of symmetry found for patterns #{}", i).into());
        }
    }

    let mut summary_num = 0;
    for line_of_symmetry in lines_of_symmetry {
        summary_num += match line_of_symmetry {
            LineOfSymmetry::Horizontal(y) => 100 * y,
            LineOfSymmetry::Vertical(x) => x,
        }
    }

    Ok(summary_num.to_string())
}

struct Pattern {
    terrain: Grid<Terrain>,
}

impl Pattern {
    fn from_grid(terrain: Grid<Terrain>) -> Self {
        Self { terrain }
    }

    fn parse(pattern_str: &str) -> Result<Pattern, String> {
        let height = pattern_str.lines().count();
        let width = pattern_str.lines().next().unwrap().chars().count();

        let mut terrain = Grid::new_empty(width, height);
        for (y, line) in pattern_str.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                terrain.set(
                    &GridPoint {
                        x: x as i64,
                        y: y as i64,
                    },
                    Terrain::parse(c)?,
                )?;
            }
        }

        Ok(Pattern { terrain })
    }

    fn find_lines_of_symmetry(&self) -> Vec<LineOfSymmetry> {
        let mut lines_of_symmetry = Vec::new();

        let mut enumerated_rows = self.terrain.rows_iter().enumerate();
        let first_row = enumerated_rows.next().unwrap().1.collect::<Vec<_>>();
        let mut previous_rows = vec![first_row];
        for (y, row) in enumerated_rows {
            if self.is_vertically_symmetric_from(y, previous_rows.clone()) {
                lines_of_symmetry.push(LineOfSymmetry::Horizontal(y));
            }
            previous_rows.push(row.collect());
        }

        let mut enumerated_columns = self.terrain.columns_iter().enumerate();
        let first_column = enumerated_columns.next().unwrap().1.collect::<Vec<_>>();
        let mut previous_columns = vec![first_column];
        for (x, column) in enumerated_columns {
            if self.is_horizontally_symmetric_from(x, previous_columns.clone()) {
                lines_of_symmetry.push(LineOfSymmetry::Vertical(x));
            }
            previous_columns.push(column.collect());
        }

        lines_of_symmetry
    }

    fn find_lines_of_symmetry_with_smudge(&self) -> Vec<LineOfSymmetry> {
        let existing_line_of_symmetry = *self
            .find_lines_of_symmetry()
            .get(0)
            .expect("couldn't find single exclusive existing line of symmetry");

        // println!("existing line of symmetry: {:?}", existing_line_of_symmetry);
        let mut lines_of_symmetry = Vec::new();
        for x in 0..self.terrain.width() {
            for y in 0..self.terrain.height() {
                let terrain = self
                    .terrain
                    .get(&GridPoint {
                        x: x as i64,
                        y: y as i64,
                    })
                    .expect(format!("couldn't get terrain at {}, {}", x, y).as_str());

                let mut potential_smudge_grid = self.terrain.clone();
                potential_smudge_grid
                    .set(
                        &GridPoint {
                            x: x as i64,
                            y: y as i64,
                        },
                        terrain.opposite(),
                    )
                    .expect(format!("couldn't set terrain at {}, {}", x, y).as_str());

                let new_pattern = Pattern::from_grid(potential_smudge_grid);

                let new_pattern_lines_of_symmetry = new_pattern.find_lines_of_symmetry();
                // if x == 4 && y == 0 {
                //     println!("outputting for x=0, y=4");
                //     println!("new pattern: \n{}", new_pattern);
                //     println!("new line of symmetry: {:?}", new_pattern_line_of_symmetry);
                //     println!("old line of symmetry: {:?}", existing_line_of_symmetry);
                // }
                for new_los in new_pattern_lines_of_symmetry {
                    if new_los != existing_line_of_symmetry {
                        lines_of_symmetry.push(new_los);
                    }
                }
            }
        }

        lines_of_symmetry
    }

    fn is_vertically_symmetric_from(
        &self,
        y: usize,
        mut previous_rows: Vec<Vec<&Terrain>>,
    ) -> bool {
        for row in self.terrain.rows_iter().skip(y) {
            let last_matching_row: Vec<Terrain> = match previous_rows.pop() {
                Some(row) => row.iter().map(|x| **x).collect(),
                None => break,
            };

            let true_row: Vec<Terrain> = row.copied().collect();

            if true_row != last_matching_row {
                return false;
            }
        }
        true
    }

    fn is_horizontally_symmetric_from(
        &self,
        x: usize,
        mut previous_columns: Vec<Vec<&Terrain>>,
    ) -> bool {
        for column in self.terrain.columns_iter().skip(x) {
            let last_matching_column: Vec<Terrain> = match previous_columns.pop() {
                Some(column) => column.iter().map(|x| **x).collect(),
                None => break,
            };

            let true_row: Vec<Terrain> = column.copied().collect();

            if true_row != last_matching_column {
                return false;
            }
            // println!(
            //     "matched column {:?} against last ({:?})",
            //     true_row, last_matching_column
            // );
        }
        // println!("found a symmetric column at x = {}", x);
        true
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.terrain.rows_iter().enumerate() {
            for terrain in row {
                write!(f, "{}", terrain)?;
            }
            if y != self.terrain.height() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineOfSymmetry {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    #[default]
    Ash,
    Rocks,
}

impl Terrain {
    fn parse(c: char) -> Result<Terrain, String> {
        match c {
            '.' => Ok(Terrain::Ash),
            '#' => Ok(Terrain::Rocks),
            _ => Err(format!("Invalid terrain character: {}", c)),
        }
    }

    fn opposite(&self) -> Terrain {
        match self {
            Terrain::Ash => Terrain::Rocks,
            Terrain::Rocks => Terrain::Ash,
        }
    }
}

impl std::fmt::Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Ash => write!(f, "."),
            Terrain::Rocks => write!(f, "#"),
        }
    }
}
