use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use regex::Regex;

use crate::data_types::{Rule, Workflow, Part, RuleResult};
use crate::data_types::PartType::{Aerodynamic, ExtremelyCool, Musical, Shiny};
use crate::data_types::RelationalType::{GreaterThan, LessThan};
use crate::data_types::RuleResult::{Accept, GoToWorkflow, NextRule, Reject};

mod data_types;

fn main () {
    let path = Path::new("src/day19_part1/input.txt");
    let (workflows, parts) = parse_data(&path);
    let ratings_sum = parts
        .iter()
        .filter(|&part| part_accepted(part, &workflows))
        .map(|part| part.get_ratings_sum())
        .sum::<u64>();
    println!("Ratings sum = {}", ratings_sum.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> (HashMap<String, Workflow>, Vec<Part>) {
    let file = File::open(&path).unwrap();
    let mut workflows = HashMap::<String, Workflow>::new();
    let mut parts = Vec::<Part>::new();
    let mut in_workflow_definitions = true;

    for line in BufReader::new(file).lines().flatten() {
        if line.len() > 0 {
            if in_workflow_definitions {
                let workflow = parse_workflow_definition(&line);
                workflows.insert(workflow.name.clone(), workflow);
            } else {
                parts.push(parse_part_definition(&line));
            }
        } else {
            in_workflow_definitions = false;
        }
    }

    return (workflows, parts);
}

fn parse_workflow_definition(line: &String) -> Workflow {
    let re = Regex::new(r"([a-z]+)\{(.+)\}").unwrap();
    let captures = re.captures(line).unwrap();

    let mut rules = Vec::<Rule>::new();
    for rule_definition in captures.get(2).unwrap().as_str().split(",") {
        rules.push(parse_rule_definition(rule_definition));
    }

    return Workflow {
        name: captures.get(1).unwrap().as_str().to_string(),
        rules
    };
}

fn parse_rule_definition(rule_definition: &str) -> Rule {
    let rule_split_result = rule_definition.split(":").collect_vec();

    let rule = match rule_split_result.len() > 1 {
        true => {
            let operator = match rule_split_result[0].contains("<") {
                true => Some(LessThan),
                false => Some(GreaterThan)
            };

            let variant = match rule_split_result[0].chars().nth(0).unwrap() {
                'x' => Some(ExtremelyCool),
                'm' => Some(Musical),
                'a' => Some(Aerodynamic),
                's' => Some(Shiny),
                _ => panic!("Invalid variant code")
            };

            let rule_threshold = Some(rule_split_result[0][2..].to_string().parse().unwrap());
            let true_result_code = rule_split_result[1].to_string();

            Rule {
                variant,
                operator,
                rule_threshold,
                true_result: true_result_code_to_value(&true_result_code)
            }
        },
        false => {
            Rule {
                variant: None,
                operator: None,
                rule_threshold: None,
                true_result: true_result_code_to_value(rule_definition),
            }
        }
    };

    return rule;
}

fn true_result_code_to_value(value: &str) -> RuleResult {
    return if value == "A" {
        Accept
    } else if value == "R" {
        Reject
    } else {
        GoToWorkflow(value.to_string())
    };
}

fn parse_part_definition(part_definition: &String) -> Part {
    let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let captures = re.captures(part_definition).unwrap();

    return Part {
        x: captures.get(1).unwrap().as_str().parse().unwrap(),
        m: captures.get(2).unwrap().as_str().parse().unwrap(),
        a: captures.get(3).unwrap().as_str().parse().unwrap(),
        s: captures.get(4).unwrap().as_str().parse().unwrap(),
    }
}

fn part_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    const STARTING_WORKFLOW_NAME: &'static str = "in";
    let mut status = None;
    let mut workflow = &workflows[STARTING_WORKFLOW_NAME];
    let mut rule_index = 0usize;

    while status.is_none() || (status != Some(Accept) && status != Some(Reject)) {
        let rule = &workflow.rules[rule_index];
        status = Some(rule.evaluate(&part));

        match &status {
            Some(NextRule) => {
                rule_index += 1;
            },
            Some(GoToWorkflow(workflow_name)) => {
                workflow = &workflows[workflow_name];
                rule_index = 0usize;
            },
            _ => {}
        }
    }

    return status.unwrap() == Accept;
}
