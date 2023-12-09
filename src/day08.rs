use std::{char, collections::HashMap, fs};

use num::Integer;

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day08_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let map = Map::parse_from_string(&input);

    let steps = map.get_steps_to_end();

    Ok(steps.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input =
        fs::read_to_string(INPUT_FILE).map_err(|err| AdventError::Other(err.to_string()))?;

    let map = Map::parse_from_string(&input);

    let steps = map.get_steps_to_all_ends_lcm();

    Ok(steps.to_string())
}

struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<Node, NodeFork>,
}

impl Map {
    fn parse_from_string(input: &str) -> Self {
        let mut lines = input.lines();

        // parse directions
        let directions: Vec<Direction> = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Unknown direction: {}", c),
            })
            .collect();

        // consume blank line
        lines.next();

        // parse node map
        let mut nodes = HashMap::new();
        for line in lines {
            let (node_key, left_right) = line.split_once(" = ").unwrap();

            let node_key = Node::new(node_key);
            let node_fork = NodeFork::parse_from_string(left_right);

            nodes.insert(node_key, node_fork);
        }

        Self { directions, nodes }
    }

    fn get_steps_to_end(&self) -> u64 {
        let end_node = Node::new("ZZZ");

        let mut steps = 0;
        let mut current_node = Node::new("AAA");
        let mut dir_index = 0;
        loop {
            if current_node == end_node {
                break;
            }
            let dir = &self.directions[dir_index % self.directions.len()];
            current_node = match dir {
                Direction::Left => self.nodes[&current_node].left,
                Direction::Right => self.nodes[&current_node].right,
            };
            dir_index += 1;
            steps += 1;
        }

        steps
    }

    fn get_steps_to_all_ends_naive(&self) -> u64 {
        let mut steps = 0;
        let mut current_nodes: Vec<Node> = self
            .nodes
            .keys()
            .filter(|node| node.id.last() == Some(&'A'))
            .copied()
            .collect();

        let mut dir_index = 0;
        loop {
            if current_nodes
                .iter()
                .all(|node| node.id.last() == Some(&'Z'))
            {
                break;
            }
            let dir = &self.directions[dir_index % self.directions.len()];
            for node in current_nodes.iter_mut() {
                *node = match dir {
                    Direction::Left => self.nodes[&node].left,
                    Direction::Right => self.nodes[&node].right,
                };
            }
            dir_index += 1;
            steps += 1;
        }

        steps
    }

    fn get_steps_to_all_ends_lcm(&self) -> u64 {
        let mut steps = 0;
        let mut current_nodes: Vec<Node> = self
            .nodes
            .keys()
            .filter(|node| node.id.last() == Some(&'A'))
            .copied()
            .collect();

        // pre-initialize all the steps for starting node
        let mut steps_per_start = vec![None; current_nodes.len()];

        let mut dir_index = 0;
        loop {
            if steps_per_start.iter().all(|steps| steps.is_some()) {
                break;
            }

            let dir = &self.directions[dir_index % self.directions.len()];
            for (i, node) in current_nodes.iter_mut().enumerate() {
                if node.id.last() == Some(&'Z') && steps_per_start[i].is_none() {
                    steps_per_start[i] = Some(steps);
                }
                *node = match dir {
                    Direction::Left => self.nodes[&node].left,
                    Direction::Right => self.nodes[&node].right,
                };
            }
            dir_index += 1;
            steps += 1;
        }

        let total_steps = steps_per_start
            .into_iter()
            .reduce(|a, b| Some(a.unwrap().lcm(&b.unwrap())))
            .unwrap()
            .unwrap();

        total_steps
    }
}

enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    id: [char; 3],
}

impl Node {
    fn new(id: &str) -> Self {
        Self {
            id: [
                id.chars().nth(0).unwrap(),
                id.chars().nth(1).unwrap(),
                id.chars().nth(2).unwrap(),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct NodeFork {
    left: Node,
    right: Node,
}

impl NodeFork {
    fn parse_from_string(input: &str) -> Self {
        let (left, right) = input[1..input.len() - 1].split_once(", ").unwrap();

        Self {
            left: Node::new(left),
            right: Node::new(right),
        }
    }
}
