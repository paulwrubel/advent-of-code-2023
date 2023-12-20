use std::{cmp, collections::HashMap, fs};

use itertools::Itertools;

use crate::{
    utils::{Interval, SortedDisjointIntervalList},
    AdventError, ExclusivePart,
};

const INPUT_FILE: &str = "./resources/day19_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();

    let workflows: Workflows = Workflows::parse(workflows_str)?;

    let parts: Vec<Part> = parts_str
        .lines()
        .map(|line| Part::parse(line))
        .try_collect()?;

    let accepted_parts = workflows.accepted_from(&parts)?;

    let mut sum_of_ratings = 0;
    for part in accepted_parts {
        sum_of_ratings += part.sum_of_ratings();
    }

    Ok(sum_of_ratings.to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let (workflows_str, _) = input.split_once("\n\n").unwrap();

    let workflows: Workflows = Workflows::parse(workflows_str)?;

    // let parts: Vec<Part> = parts_str
    //     .lines()
    //     .map(|line| Part::parse(line))
    //     .try_collect()?;

    let accepted_part_ranges = workflows.sweep_accepted_ranges()?;

    let mut total_options = 0;
    for part_range in accepted_part_ranges {
        let options_count = part_range.combination_counts();
        // println!("{}", options_count);
        total_options += options_count;
    }

    Ok(total_options.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflows {
    workflows: HashMap<String, Rules>,
}

impl Workflows {
    fn parse(input: &str) -> Result<Workflows, String> {
        let mut workflows = HashMap::new();
        for line in input.lines() {
            let (name, rules_str) = line.split_once("{").unwrap();
            let rules_str = &rules_str[..rules_str.len() - 1];

            let rules = Rules::parse(rules_str)?;

            workflows.insert(name.to_string(), rules);
        }

        Ok(Workflows { workflows })
    }

    fn accepted_from(&self, parts: &[Part]) -> Result<Vec<Part>, String> {
        let mut accepted_parts = Vec::new();
        for part in parts {
            if self.process(part)? == FinalRuleResult::Accept {
                accepted_parts.push(part.clone());
            }
        }
        Ok(accepted_parts)
    }

    fn sweep_accepted_ranges(&self) -> Result<Vec<RangedPart>, String> {
        let starting_part = RangedPart::new_all();

        let mut to_resolve = HashMap::new();
        to_resolve.insert("in".to_string(), vec![starting_part]);

        let mut accepted = Vec::new();
        let mut rejected = Vec::new();

        while !to_resolve.is_empty() {
            let mut to_resolve_next = HashMap::new();
            for (workflow_name, parts) in to_resolve.into_iter() {
                let rules = self.workflows.get(&workflow_name).unwrap();

                for part in parts {
                    let result = rules.process_range(&part)?;

                    accepted.extend(result.accepted);
                    rejected.extend(result.rejected);

                    for next in result.to_resolve {
                        to_resolve_next
                            .entry(next.workflow_name)
                            .and_modify(|v: &mut Vec<RangedPart>| v.push(next.part.clone()))
                            .or_insert(vec![next.part.clone()]);
                    }
                }
            }
            to_resolve = to_resolve_next;
        }

        Ok(accepted)
    }

    fn process(&self, part: &Part) -> Result<FinalRuleResult, String> {
        let mut active_workflow = "in";
        loop {
            if let Some(rules) = self.workflows.get(active_workflow) {
                match rules.process(part)? {
                    RuleResult::Finalize(result) => {
                        return Ok(result.clone());
                    }
                    RuleResult::SendToWorkflow(workflow) => {
                        active_workflow = &workflow;
                    }
                }
            } else {
                return Err(format!("invalid workflow: {}", active_workflow));
            }
        }
    }
}

struct RulesProcessRangeResult {
    accepted: Vec<RangedPart>,
    rejected: Vec<RangedPart>,
    to_resolve: Vec<Next>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Next {
    workflow_name: String,
    part: RangedPart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn parse(input: &str) -> Result<Rules, String> {
        let mut rules = Vec::new();
        for rule_str in input.split(',') {
            rules.push(Rule::parse(rule_str)?);
        }
        Ok(Rules { rules })
    }

    fn process(&self, part: &Part) -> Result<&RuleResult, String> {
        for rule in &self.rules {
            if rule.condition.is_none() || part.meets_condition(&rule.condition.unwrap()) {
                return Ok(&rule.destination);
            }
        }
        Err(format!("somehow, no rule matched for {:?}", part))
    }

    fn process_range(&self, part: &RangedPart) -> Result<RulesProcessRangeResult, String> {
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();
        let mut to_resolve = Vec::new();

        let mut current_part = part.clone();
        for rule in &self.rules {
            let (passed, failed) = if rule.condition.is_none() {
                (current_part.clone(), None)
            } else {
                let (p, f) = current_part.split_range(&rule.condition.unwrap());
                (p, Some(f))
            };

            match &rule.destination {
                RuleResult::Finalize(FinalRuleResult::Accept) => {
                    accepted.push(passed);
                }
                RuleResult::Finalize(FinalRuleResult::Reject) => {
                    rejected.push(passed);
                }
                RuleResult::SendToWorkflow(workflow) => to_resolve.push(Next {
                    workflow_name: workflow.clone(),
                    part: passed,
                }),
            }

            if let Some(failed) = failed {
                current_part = failed;
            } else {
                break;
            }
        }

        Ok(RulesProcessRangeResult {
            accepted,
            rejected,
            to_resolve,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    condition: Option<Condition>,
    destination: RuleResult,
}

impl Rule {
    fn parse(input: &str) -> Result<Self, String> {
        match input.split_once(':') {
            Some((condition, destination)) => {
                let condition = Condition::parse(condition)?;
                let destination = RuleResult::parse(destination)?;

                Ok(Rule {
                    condition: Some(condition),
                    destination,
                })
            }
            None => {
                let destination = RuleResult::parse(input)?;

                Ok(Rule {
                    condition: None,
                    destination,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Condition {
    category: PartCategory,
    ordering: cmp::Ordering,
    value: i64,
}

impl Condition {
    fn parse(input: &str) -> Result<Condition, String> {
        if input.len() < 3 {
            return Err(format!("invalid condition (too short): {}", input));
        }

        let mut chars = input.chars();

        let category = PartCategory::parse(chars.next().unwrap())?;

        let ordering = match chars.next().unwrap() {
            '>' => cmp::Ordering::Greater,
            '<' => cmp::Ordering::Less,
            _ => {
                return Err(format!("invalid condition (bad comparator): {}", input));
            }
        };

        let value = input[2..]
            .parse::<i64>()
            .map_err(|err| format!("invalid condition (bad value): {}: {}", input, err))?;

        Ok(Condition {
            category,
            ordering,
            value,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FinalRuleResult {
    Accept,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleResult {
    Finalize(FinalRuleResult),
    SendToWorkflow(String),
}

impl RuleResult {
    fn parse(input: &str) -> Result<RuleResult, String> {
        match input {
            "A" => Ok(RuleResult::Finalize(FinalRuleResult::Accept)),
            "R" => Ok(RuleResult::Finalize(FinalRuleResult::Reject)),
            name => Ok(RuleResult::SendToWorkflow(name.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PartCategory {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl PartCategory {
    fn parse(input: char) -> Result<PartCategory, String> {
        match input {
            'x' => Ok(PartCategory::ExtremelyCoolLooking),
            'm' => Ok(PartCategory::Musical),
            'a' => Ok(PartCategory::Aerodynamic),
            's' => Ok(PartCategory::Shiny),
            _ => Err(format!("invalid part category: {}", input)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    ratings: [i64; 4],
}

impl Part {
    fn parse(input: &str) -> Result<Part, String> {
        let mut ratings = [0i64; 4];

        // trim off the curly braces
        let input = &input[1..input.len() - 1];

        let rating_strs = input.split(',').collect_vec();
        if rating_strs.len() != 4 {
            return Err(format!("invalid part (bad number of ratings): {}", input));
        }

        for (i, rating_str) in rating_strs.into_iter().enumerate() {
            let chars = rating_str.chars().collect_vec();
            if chars.len() < 3 {
                return Err(format!("invalid part (rating too short): {}", input));
            }
            ratings[i] = rating_str[2..]
                .parse()
                .map_err(|err| format!("invalid part (bad rating): {}: {}", input, err))?;
        }

        Ok(Part { ratings })
    }

    fn meets_condition(&self, condition: &Condition) -> bool {
        let Condition {
            category,
            ordering,
            value,
        } = condition;

        let category_index = match category {
            PartCategory::ExtremelyCoolLooking => 0,
            PartCategory::Musical => 1,
            PartCategory::Aerodynamic => 2,
            PartCategory::Shiny => 3,
        };

        self.meets_ordering_for_index(category_index, ordering, *value)
    }

    fn meets_ordering_for_index(
        &self,
        index: usize,
        ordering: &cmp::Ordering,
        condition_value: i64,
    ) -> bool {
        let our_value = self.ratings[index];
        match ordering {
            cmp::Ordering::Greater => our_value > condition_value,
            cmp::Ordering::Less => our_value < condition_value,
            cmp::Ordering::Equal => our_value == condition_value,
        }
    }

    fn sum_of_ratings(&self) -> i64 {
        self.ratings.iter().sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangedPart {
    ranges: [SortedDisjointIntervalList<i64>; 4],
}

impl RangedPart {
    fn new_all() -> RangedPart {
        let ranges = [
            SortedDisjointIntervalList::new(vec![Interval::new(1..4001)]),
            SortedDisjointIntervalList::new(vec![Interval::new(1..4001)]),
            SortedDisjointIntervalList::new(vec![Interval::new(1..4001)]),
            SortedDisjointIntervalList::new(vec![Interval::new(1..4001)]),
        ];
        RangedPart { ranges }
    }

    fn combination_counts(&self) -> u64 {
        let mut combinations = 1;
        for range in &self.ranges {
            combinations *= range.span();
        }
        combinations
    }

    fn split_range(&self, condition: &Condition) -> (RangedPart, RangedPart) {
        let condition_index = match condition.category {
            PartCategory::ExtremelyCoolLooking => 0,
            PartCategory::Musical => 1,
            PartCategory::Aerodynamic => 2,
            PartCategory::Shiny => 3,
        };

        let relevant_interval_list = &self.ranges[condition_index];

        let passing_interval_list = match condition.ordering {
            cmp::Ordering::Greater => {
                SortedDisjointIntervalList::new(vec![Interval::new(condition.value + 1..4001)])
            }
            cmp::Ordering::Less => {
                SortedDisjointIntervalList::new(vec![Interval::new(1..condition.value)])
            }
            cmp::Ordering::Equal => SortedDisjointIntervalList::new(vec![Interval::new(
                condition.value..condition.value + 1,
            )]),
        };

        let passing = relevant_interval_list.intersect(&passing_interval_list);
        let failing = relevant_interval_list.subtract(&passing);

        let mut passing_ranges = self.ranges.clone();
        passing_ranges[condition_index] = passing;

        let mut failing_ranges = self.ranges.clone();
        failing_ranges[condition_index] = failing;

        (
            RangedPart {
                ranges: passing_ranges,
            },
            RangedPart {
                ranges: failing_ranges,
            },
        )
    }
}
