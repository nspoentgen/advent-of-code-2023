use RuleResult::*;
use RelationalType::*;
use PartType::*;

#[derive(Clone, Debug)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub variant: Option<PartType>,
    pub operator: Option<RelationalType>,
    pub rule_threshold: Option<u64>,
    pub true_result: RuleResult,
}

impl Rule {
    pub fn evaluate(&self, input_parts: &AggregatePart) -> Vec<(RuleResult, AggregatePart)> {
        //Final rule corner case. Automatically use true result.
        if self.variant.is_none() {
            return vec![(self.true_result.clone(), (*input_parts).clone())];
        }

        //Nominal case
        let (lower_bound, upper_bound) = match self.variant {
            Some(ExtremelyCool) => (input_parts.x_lower_bound, input_parts.x_upper_bound),
            Some(Musical) => (input_parts.m_lower_bound, input_parts.m_upper_bound),
            Some(Aerodynamic) => (input_parts.a_lower_bound, input_parts.a_upper_bound),
            Some(Shiny) => (input_parts.s_lower_bound, input_parts.s_upper_bound),
            None => panic!("Cannot evaluate rule since variant is None")
        };

        let mut results = Vec::<(RuleResult, AggregatePart)>::new();
        match self.operator.unwrap() {
            GreaterThan => {
                if lower_bound <= self.rule_threshold.unwrap() && upper_bound <= self.rule_threshold.unwrap() {
                    results.push((NextRule, input_parts.clone()));
                } else if lower_bound <= self.rule_threshold.unwrap() && upper_bound > self.rule_threshold.unwrap() {
                    let mut failing_parts = input_parts.clone();
                    failing_parts.update_bounds(self.variant.unwrap(), lower_bound, self.rule_threshold.unwrap());
                    results.push((NextRule, failing_parts));

                    let mut passing_parts = input_parts.clone();
                    passing_parts.update_bounds(self.variant.unwrap(), self.rule_threshold.unwrap() + 1, upper_bound);
                    results.push((self.true_result.clone(), passing_parts));
                } else {
                    results.push((self.true_result.clone(), input_parts.clone()));
                }
            },
            LessThan => {
                if lower_bound <= self.rule_threshold.unwrap() && upper_bound <= self.rule_threshold.unwrap() {
                    results.push((self.true_result.clone(), input_parts.clone()));
                } else if lower_bound <= self.rule_threshold.unwrap() && upper_bound > self.rule_threshold.unwrap() {
                    let mut passing_parts = input_parts.clone();
                    passing_parts.update_bounds(self.variant.unwrap(), lower_bound, self.rule_threshold.unwrap() - 1);
                    results.push((self.true_result.clone(), passing_parts));

                    let mut failing_parts = input_parts.clone();
                    failing_parts.update_bounds(self.variant.unwrap(), self.rule_threshold.unwrap(), upper_bound);
                    results.push((NextRule, failing_parts));
                } else {
                    results.push((NextRule, input_parts.clone()));
                }
            }
        }

        return results;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PartType {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny
}

#[derive(Copy, Clone, Debug)]
pub enum RelationalType {
    LessThan,
    GreaterThan
}

#[derive(Clone, PartialEq, Debug)]
pub enum RuleResult {
    Accept,
    Reject,
    NextRule,
    GoToWorkflow(String)
}

#[derive(Clone, Debug)]
pub struct AggregatePart {

    pub x_lower_bound: u64,
    pub x_upper_bound: u64,
    pub m_lower_bound: u64,
    pub m_upper_bound: u64,
    pub a_lower_bound: u64,
    pub a_upper_bound: u64,
    pub s_lower_bound: u64,
    pub s_upper_bound: u64
}

impl AggregatePart {
    pub const MIN_PART_NUMBER: u64 = 1;
    pub const MAX_PART_NUMBER: u64 = 4000;

    pub fn update_bounds(&mut self, part_type: PartType, lower_bound: u64, upper_bound: u64) {
        match part_type {
            ExtremelyCool => {
                self.x_lower_bound = lower_bound;
                self.x_upper_bound = upper_bound;
            },
            Musical => {
                self.m_lower_bound = lower_bound;
                self.m_upper_bound = upper_bound;
            },
            Aerodynamic => {
                self.a_lower_bound = lower_bound;
                self.a_upper_bound = upper_bound;
            },
            Shiny => {
                self.s_lower_bound = lower_bound;
                self.s_upper_bound = upper_bound;
            },
        }
    }

    pub fn all_parts() -> Self {
        //Bound are open so add/subtract 1 to min/max part numbers
        return AggregatePart {
            x_lower_bound: Self::MIN_PART_NUMBER,
            x_upper_bound: Self::MAX_PART_NUMBER,
            m_lower_bound: Self::MIN_PART_NUMBER,
            m_upper_bound: Self::MAX_PART_NUMBER,
            a_lower_bound: Self::MIN_PART_NUMBER,
            a_upper_bound: Self::MAX_PART_NUMBER,
            s_lower_bound: Self::MIN_PART_NUMBER,
            s_upper_bound: Self::MAX_PART_NUMBER,
        };
    }

    pub fn get_parts_combinations(&self) -> u64 {
        let bound_pairs = [(self.x_lower_bound, self.x_upper_bound), (self.m_lower_bound, self.m_upper_bound),
            (self.a_lower_bound, self.a_upper_bound), (self.s_lower_bound, self.s_upper_bound)];

        return bound_pairs
            .iter()
            .map(|(lb, ub)| ub - lb + 1)
            .filter(|x| *x > 0)
            .product::<u64>();
    }
}
