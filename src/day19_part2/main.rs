use std::path::Path;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::discriminant;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use regex::Regex;

use crate::data_types::{Rule, Workflow, RuleResult, AggregatePart};
use crate::data_types::PartType::{Aerodynamic, ExtremelyCool, Musical, Shiny};
use crate::data_types::RelationalType::{GreaterThan, LessThan};
use crate::data_types::RuleResult::{Accept, GoToWorkflow, NextRule, Reject};

mod data_types;

fn main () {
    let path = Path::new("src/day19_part1/input.txt");
    let workflows = parse_data(&path);
    let accepted_parts = get_all_accepted_parts(&workflows);


    let total_sum = accepted_parts.iter().map(|x| x.get_parts_combinations()).sum::<u64>();

    println!("Ratings sum = {}", total_sum.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> HashMap<String, Workflow> {
    let file = File::open(&path).unwrap();
    let mut workflows = HashMap::<String, Workflow>::new();
    let lines = BufReader::new(file).lines().flatten().collect_vec();
    let mut line_index = 0usize;

    while lines[line_index].len() > 0 {
        let workflow = parse_workflow_definition(&lines[line_index]);
        workflows.insert(workflow.name.clone(), workflow);
        line_index += 1
    }

    return workflows;
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

fn get_all_accepted_parts(all_workflows: &HashMap<String, Workflow>) -> Vec<AggregatePart> {
    const STARTING_WORKFLOW_NAME: &'static str = "in";
    let initial_workflow = all_workflows[STARTING_WORKFLOW_NAME].clone();
    let initial_rule_index = 0usize;
    let initial_parts = AggregatePart::all_parts();
    let mut all_passing_parts = Vec::<AggregatePart>::new();

    let mut evaluation_queue = VecDeque::<(RuleResult, Workflow, usize, AggregatePart)>::new();
    evaluation_queue.extend(evaluate_rule(initial_workflow.clone(), initial_rule_index, initial_parts.clone(), all_workflows));

    while evaluation_queue.len() > 0 {
        let status = evaluation_queue.pop_front().unwrap();

        if discriminant(&status.0) == discriminant(&Accept) {
            all_passing_parts.push(status.3);
        } else if discriminant(&status.0) == discriminant(&GoToWorkflow("".to_string())) || discriminant(&status.0) == discriminant(&NextRule) {
            evaluation_queue.extend(evaluate_rule(status.1.clone(), status.2, status.3.clone(), all_workflows));
        }
    }

    return all_passing_parts;
}

fn evaluate_rule(workflow: Workflow, rule_index: usize, input_parts: AggregatePart,
                 all_workflows: &HashMap<String, Workflow>) -> Vec<(RuleResult, Workflow, usize, AggregatePart)>
{
    let rule = workflow.rules[rule_index].clone();
    let evaluation_results = rule.evaluate(&input_parts);
    let mut statuses = Vec::<(RuleResult, Workflow, usize, AggregatePart)>::new();

    for result in evaluation_results {
        match &result.0 {
            GoToWorkflow(workflow_name) => {
                statuses.push((result.0.clone(), all_workflows[workflow_name].clone(), 0usize, result.1))
            },
            _ => {
                statuses.push((result.0.clone(), workflow.clone(), rule_index + 1, result.1))
            },
        }
    }

    return statuses;
}
