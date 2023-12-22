use std::{
    cmp,
    collections::{BinaryHeap, HashMap},
    fs,
};

use crate::{
    utils::{CardinalDirection, Grid, GridPoint},
    AdventError, ExclusivePart,
};

const INPUT_FILE: &str = "./resources/day17_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let debug = false;

    let city_map = CityMap::parse(&input)?;

    let starting_point = (0, 0);
    let ending_point = (city_map.map.width() - 1, city_map.map.height() - 1);

    // let ending_point = (5, 5);

    let straight_line_limits = (1, 3);

    let optimal_path = city_map.find_optimal_path(
        starting_point.into(),
        ending_point.into(),
        straight_line_limits,
    )?;

    if debug {
        println!(
            "path: \n{}",
            optimal_path.get_string_mapped_onto(&city_map.map, false)?
        );
    }

    let path_heat_loss = optimal_path.cost;

    Ok(path_heat_loss.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let debug = false;

    let city_map = CityMap::parse(&input)?;

    let starting_point = (0, 0);
    let ending_point = (city_map.map.width() - 1, city_map.map.height() - 1);

    // let ending_point = (5, 5);

    let straight_line_limits = (4, 10);

    let optimal_path = city_map.find_optimal_path(
        starting_point.into(),
        ending_point.into(),
        straight_line_limits,
    )?;

    if debug {
        println!(
            "path: \n{}",
            optimal_path.get_string_mapped_onto(&city_map.map, false)?
        );
    }

    let path_heat_loss = optimal_path.cost;

    Ok(path_heat_loss.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CityMap {
    map: Grid<u64>,
}

impl CityMap {
    fn parse(input: &str) -> Result<Self, String> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let mut map = Grid::new_empty(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.set(&(x, y).into(), c.to_digit(10).unwrap() as u64)?;
            }
        }

        Ok(CityMap { map })
    }

    fn find_optimal_path(
        &self,
        starting_point: GridPoint,
        ending_point: GridPoint,
        straight_line_limits: (i64, i64),
    ) -> Result<Path, String> {
        // this may contain duplicate nodes (with different costs), but that's ok!
        // since a `pop` will always return the node with the lowest cost, we will
        // practically only incur a mild memory cost, not a cpu cost
        let mut frontier = BinaryHeap::new();
        let starting_node = Node {
            location: starting_point,
            direction_to_here: None,
        };
        frontier.push(NodeWithCost {
            node: starting_node,
            cost: 0,
        });

        // this is a complete maps of all "discovered" nodes.
        // the key is the node, and the value is the optimal (so far) cost of getting to that node
        let mut best_paths = HashMap::new();
        best_paths.insert(
            starting_node,
            Path {
                cost: 0,
                nodes: vec![starting_node.location],
            },
        );

        while let Some(node_with_cost) = frontier.pop() {
            // node_with_cost is guaranteed to be the node with the MINIMUM cost

            let NodeWithCost { node, cost } = node_with_cost;

            // if we've found the ending point, we can return the path,
            // since we know that the optimal path will be the shortest
            if node.location == ending_point {
                return Ok(Path {
                    cost,
                    nodes: best_paths.get(&node).unwrap().nodes.clone(),
                });
            }

            if let Some(best_path) = best_paths.get(&node) {
                // if this node has already been discovered with a lower cost,
                // we can skip it
                if cost > best_path.cost {
                    continue;
                }
            }

            // valid cardinal directions we can travel in from here
            let valid_directions = if let Some(direction) = node.direction_to_here {
                vec![direction.turn_left(), direction.turn_right()]
            } else {
                CardinalDirection::all().to_vec()
            };

            let mut potential_destinations_with_direction = Vec::new();
            for direction in valid_directions {
                for distance in straight_line_limits.0..=straight_line_limits.1 {
                    let potential_destination = node
                        .location
                        .neighbor_in_direction_distance(direction, distance);
                    if self.map.is_within_bounds(&potential_destination) {
                        potential_destinations_with_direction
                            .push((potential_destination, direction));
                    }
                }
            }

            // println!("destinations: {:?}", potential_destinations_with_direction);

            for (destination, direction) in potential_destinations_with_direction {
                // start with the final destination cost, since we know we'll incur at least that
                let mut cost_increase = *self.map.get(&destination).unwrap();
                for intermediate_location in node
                    .location
                    .points_between_orthogonal_exclusive(&destination)
                {
                    // add the cost of each node we traverse on the way there
                    cost_increase += *self.map.get(&intermediate_location).unwrap();
                }

                // the final cost is the cost to get here + the cost to get there
                let new_cost = cost + cost_increase;

                let new_node = Node {
                    location: destination,
                    direction_to_here: Some(direction),
                };

                let new_node_with_cost = NodeWithCost {
                    node: new_node.clone(),
                    cost: new_cost,
                };

                let current_path = best_paths.get(&node).unwrap();
                let mut new_nodes = current_path.nodes.clone();
                new_nodes.push(destination);
                let new_path = Path {
                    cost: new_cost,
                    nodes: new_nodes,
                };

                let should_insert: bool = match best_paths.get(&new_node) {
                    Some(prev_best_path) => {
                        if new_cost < prev_best_path.cost {
                            true
                        } else {
                            false
                        }
                    }
                    None => true,
                };

                if should_insert {
                    // println!("inserting new node: {:?}", new_node_with_cost);
                    frontier.push(new_node_with_cost);
                    best_paths.insert(new_node, new_path);
                }
            }
        }

        let mut best_path: Option<&Path> = None;
        for (node, path) in &best_paths {
            if node.location == ending_point
                && (best_path.is_none() || path.cost < best_path.unwrap().cost)
            {
                best_path = Some(path);
            }
        }

        if let Some(best_path) = best_path {
            Ok(best_path.clone())
        } else {
            Err("No path found".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    nodes: Vec<GridPoint>,
    cost: u64,
}

impl Path {
    fn get_string_mapped_onto(
        &self,
        grid: &Grid<u64>,
        show_non_traversed: bool,
    ) -> Result<String, String> {
        let mut new_grid = Grid::new_empty(grid.width(), grid.height());
        for entry in grid.entries() {
            new_grid.set(
                &entry.point,
                if show_non_traversed {
                    entry.value.to_string()
                } else {
                    ".".to_string()
                },
            )?;
        }
        new_grid.set(&self.nodes[0], "S".to_string())?;
        for i in 1..self.nodes.len() {
            let this_node = self.nodes[i];
            let prev_node = self.nodes[i - 1];
            let mut points_to_set = prev_node.points_between_orthogonal_exclusive(&this_node);
            points_to_set.push(this_node);
            let cardinal_char: char = prev_node.direction_to_orthogonal(&this_node)?.into();
            for point in points_to_set {
                new_grid.set(&point, cardinal_char.to_string())?;
            }
        }

        Ok(new_grid.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    location: GridPoint,
    direction_to_here: Option<CardinalDirection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeWithCost {
    node: Node,
    cost: u64,
}

impl cmp::PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // we reverse the ordering to get a min heap
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CostAndPreviousNode {
    cost: u64,
    parent: Option<Node>,
}
