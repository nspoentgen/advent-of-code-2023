use RuleResult::*;
use RelationalType::*;
use PartType::*;

pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>
}

pub struct Rule {
    pub variant: Option<PartType>,
    pub operator: Option<RelationalType>,
    pub rule_threshold: Option<u64>,
    pub true_result: RuleResult,
}

impl Rule {
    pub fn evaluate(&self, part: &Part) -> RuleResult {
        //Short circuit for automatic result
        if self.variant.is_none() {
            return self.true_result.clone();
        }

        //Evaluate normal conditions
        let argument = match self.variant {
            Some(ExtremelyCool) => part.x,
            Some(Musical) => part.m,
            Some(Aerodynamic) => part.a,
            Some(Shiny) => part.s,
            None => panic!("Cannot evaluate rule since variant is None")
        };

        let boolean_result = match self.operator {
            Some(LessThan) => argument < self.rule_threshold.unwrap(),
            Some(GreaterThan) => argument > self.rule_threshold.unwrap(),
            None => panic!("Cannot evaluate rule since threshold is None")
        };

        return match boolean_result {
            true => self.true_result.clone(),
            false => NextRule
        }
    }
}

#[derive(Copy, Clone)]
pub enum PartType {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny
}

#[derive(Copy, Clone)]
pub enum RelationalType {
    LessThan,
    GreaterThan
}

#[derive(Clone, PartialEq)]
pub enum RuleResult {
    Accept,
    Reject,
    NextRule,
    GoToWorkflow(String)
}

pub struct Part {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64
}

impl Part {
    pub fn get_ratings_sum(&self) -> u64 {
        return self.x + self.m + self.a + self.s;
    }
}
