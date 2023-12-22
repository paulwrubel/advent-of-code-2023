use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    fmt, fs,
};

use itertools::Itertools;

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day22_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut bricks = Bricks::parse(&input)?;

    bricks.settle()?;

    let removable_bricks = bricks.find_removable()?;

    Ok(removable_bricks.len().to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut bricks = Bricks::parse(&input)?;

    bricks.settle()?;

    let chain_reaction_counts = bricks.find_chain_reaction_counts()?;

    let mut total_chain_reaction_count = 0;
    for (_a, chain_reaction_count) in chain_reaction_counts {
        total_chain_reaction_count += chain_reaction_count;
    }

    Ok(total_chain_reaction_count.to_string())
}

struct Bricks {
    bricks: Vec<Brick>,
}

impl Bricks {
    fn parse(input: &str) -> Result<Bricks, String> {
        let mut bricks = Vec::new();
        for line in input.lines() {
            bricks.push(Brick::parse(line)?);
        }

        Ok(Bricks { bricks })
    }

    fn settle(&mut self) -> Result<(), String> {
        // try to settle each bricks, which are all assumed to not be settled
        // we keep track of the indices of bricks that need to be settled instead of
        // the bricks themselves because we need to be able to change them at any point mutably

        // sort the bricks by z. this ensures that bricks nearer to the ground are settled first, which
        // likely will have knock-off effects for bricks above it in the queue.
        //
        // it is not necessary to sort this for correctness, but it makes the function resolve faster
        let mut sorted_bricks = self.bricks.iter().enumerate().collect_vec();
        sorted_bricks.sort_by(|(_, a), (_, b)| {
            // sort ascending by z
            match a.a.z().cmp(&b.a.z()) {
                cmp::Ordering::Less => cmp::Ordering::Less,
                cmp::Ordering::Greater => cmp::Ordering::Greater,
                cmp::Ordering::Equal => a.b.z().cmp(&b.b.z()),
            }
        });

        // extract the indices of the bricks that need to be settled and put them in a queue
        let mut indices_to_settle: VecDeque<usize> =
            VecDeque::from_iter(sorted_bricks.iter().map(|(i, _)| *i));

        // get a brick index from the front of the queue
        // this order is semi-sorted so that we settle bricks that are closer to the ground first
        while let Some(index) = indices_to_settle.pop_front() {
            // try to drop the brick and get which bricks, if any, are supporting it
            let supporting_bricks = self.drop_brick_by_index_and_get_supporting_bricks(index)?;

            // check if this brick is settled or not
            //
            // a brick is settled when:
            // - it has no supporting bricks, meaning it is supported by the ground, or
            // - all supporting bricks are settled
            let mut is_settled = true;
            for supporting_brick in supporting_bricks.iter() {
                if !supporting_brick.is_settled {
                    is_settled = false;
                }
            }

            // get a mutable reference to the brick
            let brick = &mut self.bricks[index];

            // update this brick's settled status
            brick.is_settled = is_settled;

            // finally, if this brick is not settled...
            if !brick.is_settled {
                // ...we need to add it to the back of the queue
                indices_to_settle.push_back(index);
            }
        }

        Ok(())
    }

    fn drop_brick_by_index_and_get_supporting_bricks(
        &mut self,
        index: usize,
    ) -> Result<Vec<Brick>, String> {
        loop {
            let brick = &self.bricks[index];
            // check if this brick is on the ground.
            // if so, it's settled, supported by no other bricks, and can't be dropped
            if brick.a.z() == 1 || brick.b.z() == 1 {
                return Ok(Vec::new());
            }

            // find bricks which are directly below this brick
            let mut supporting_bricks = Vec::new();
            let below_points = brick.points_below().collect_vec();
            for other_brick in self.bricks.iter() {
                for below_point in below_points.iter() {
                    if other_brick.contains_point(below_point) {
                        supporting_bricks.push(other_brick.clone());
                    }
                }
            }

            if supporting_bricks.is_empty() {
                // drop brick one level
                self.bricks[index].drop(1);
                // we will do another pass to try to drop again
            } else {
                return Ok(supporting_bricks);
            }
        }
    }

    fn get_support_maps(&self) -> (HashMap<&Brick, Vec<&Brick>>, HashMap<&Brick, Vec<&Brick>>) {
        let mut supporting = HashMap::new();
        let mut supported_by = HashMap::new();
        for a in &self.bricks {
            let mut supporting_a = Vec::new();
            let mut supported_by_a = Vec::new();
            for b in &self.bricks {
                if b.is_directly_supporting(a) {
                    supporting_a.push(b);
                } else if b.is_directly_supported_by(a) {
                    supported_by_a.push(b);
                }
            }
            supporting.insert(a, supporting_a);
            supported_by.insert(a, supported_by_a);
        }

        (supporting, supported_by)
    }

    fn find_chain_reaction_counts(&self) -> Result<HashMap<&Brick, usize>, String> {
        // get support maps
        let (supporting, supported_by) = self.get_support_maps();

        let mut chain_reaction_counts = HashMap::new();
        for a in &self.bricks {
            let chain_reaction_count =
                self.total_dependent_bricks_for(a, &supporting, &supported_by, &mut HashSet::new());
            chain_reaction_counts.insert(a, chain_reaction_count);
        }
        Ok(chain_reaction_counts)
    }

    fn total_dependent_bricks_for<'a, 'b>(
        &self,
        a: &'b Brick,
        supporting: &HashMap<&'a Brick, Vec<&'a Brick>>,
        supported_by: &HashMap<&'a Brick, Vec<&'a Brick>>,
        resolved: &mut HashSet<&'b Brick>,
    ) -> usize
    where
        'a: 'b,
    {
        if resolved.contains(a) {
            return 0;
        }
        resolved.insert(a);

        let mut dependencies = 0;
        let mut will_collapse = Vec::new();
        for b in supported_by[a].iter().copied() {
            // if a is the only brick supporting b, b will definitely collapse
            if supporting[b].len() == 1 {
                will_collapse.push(b);
                continue;
            }

            // otherwise, b can only collapse if ALL of it supports are already marked as collapsing
            let mut can_still_collapse = true;
            for c in supporting[b].iter().copied() {
                if !resolved.contains(c) {
                    can_still_collapse = false;
                }
            }

            // if so, mark b as collapsing
            if can_still_collapse {
                will_collapse.push(b);
            }
        }

        // for each depending brick, add its dependencies and itself to our count
        for collapsing_brick in will_collapse {
            dependencies += 1 + self.total_dependent_bricks_for(
                collapsing_brick,
                supporting,
                supported_by,
                resolved,
            );
        }
        dependencies
    }

    fn find_removable(&self) -> Result<Vec<&Brick>, String> {
        // get support maps
        let (supporting, supported_by) = self.get_support_maps();

        let mut removable = Vec::new();
        for a in &self.bricks {
            // if a brick is not supporting any other bricks, it is removable
            if supported_by[a].is_empty() {
                removable.push(a);
            } else {
                // otherwise, a brick is removable ONLY IF all the bricks it supports are
                // also supported by other bricks
                let mut is_removable = true;
                for b in supported_by[a].iter() {
                    if supporting[b].len() == 1 {
                        // only supported by a! this brick is not removable
                        is_removable = false;
                        break;
                    }
                }
                if is_removable {
                    removable.push(a);
                }
            }
        }

        Ok(removable)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    a: BlockPoint,
    b: BlockPoint,
    is_settled: bool,
}

impl Brick {
    fn parse(line: &str) -> Result<Brick, String> {
        let (p1, p2) = line
            .split_once('~')
            .ok_or(format!("invalid line: {}", line))?;

        let p1_components: Vec<i64> = p1
            .split(',')
            .map(|s| {
                s.parse::<i64>()
                    .map_err(|err| format!("failed to parse {} as i64: {}", s, err))
            })
            .try_collect()?;

        let p2_components: Vec<i64> = p2
            .split(',')
            .map(|s| {
                s.parse::<i64>()
                    .map_err(|err| format!("failed to parse {} as i64: {}", s, err))
            })
            .try_collect()?;

        let p1: BlockPoint = p1_components.into();
        let p2: BlockPoint = p2_components.into();

        let min = p1.min_components(&p2);
        let max = p2.max_components(&p2);

        Ok(Brick {
            a: min,
            b: max,
            is_settled: false,
        })
    }

    fn drop(&mut self, levels: i64) {
        self.a.0[2] -= levels;
        self.b.0[2] -= levels;
    }

    fn points_above(&self) -> impl Iterator<Item = BlockPoint> + '_ {
        self.encompassing_points().filter_map(|p| {
            let above = p.above();
            if !self.contains_point(&above) {
                Some(above)
            } else {
                None
            }
        })
    }

    fn points_below(&self) -> impl Iterator<Item = BlockPoint> + '_ {
        self.encompassing_points().filter_map(|p| {
            let below = p.below();
            if !self.contains_point(&below) {
                Some(below)
            } else {
                None
            }
        })
    }

    fn encompassing_points(&self) -> impl Iterator<Item = BlockPoint> {
        let mut points = Vec::new();

        // only one of these loops will have more than one iteration
        // so the efficiency is not nearly as bad as it appears here
        for x in self.a.x()..=self.b.x() {
            for y in self.a.y()..=self.b.y() {
                for z in self.a.z()..=self.b.z() {
                    points.push(BlockPoint::new(x, y, z));
                }
            }
        }
        points.into_iter()
    }

    fn contains_point(&self, p: &BlockPoint) -> bool {
        self.a.x() <= p.x()
            && p.x() <= self.b.x()
            && self.a.y() <= p.y()
            && p.y() <= self.b.y()
            && self.a.z() <= p.z()
            && p.z() <= self.b.z()
    }

    fn is_directly_supporting(&self, other: &Brick) -> bool {
        other
            .encompassing_points()
            .any(|p| self.points_above().contains(&p))
    }

    fn is_directly_supported_by(&self, other: &Brick) -> bool {
        other.is_directly_supporting(self)
    }
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}~{}", self.a, self.b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BlockPoint([i64; 3]);

impl BlockPoint {
    fn new(x: i64, y: i64, z: i64) -> BlockPoint {
        BlockPoint([x, y, z])
    }

    fn x(&self) -> i64 {
        self.0[0]
    }

    fn y(&self) -> i64 {
        self.0[1]
    }

    fn z(&self) -> i64 {
        self.0[2]
    }

    fn min_components(&self, other: &BlockPoint) -> BlockPoint {
        BlockPoint([
            self.0[0].min(other.0[0]),
            self.0[1].min(other.0[1]),
            self.0[2].min(other.0[2]),
        ])
    }

    fn max_components(&self, other: &BlockPoint) -> BlockPoint {
        BlockPoint([
            self.0[0].max(other.0[0]),
            self.0[1].max(other.0[1]),
            self.0[2].max(other.0[2]),
        ])
    }

    fn above(&self) -> BlockPoint {
        BlockPoint([self.0[0], self.0[1], self.0[2] + 1])
    }

    fn below(&self) -> BlockPoint {
        BlockPoint([self.0[0], self.0[1], self.0[2] - 1])
    }
}

impl From<Vec<i64>> for BlockPoint {
    fn from(v: Vec<i64>) -> Self {
        BlockPoint([v[0], v[1], v[2]])
    }
}

impl fmt::Display for BlockPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}
