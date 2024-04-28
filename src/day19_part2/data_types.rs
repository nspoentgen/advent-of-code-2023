use RuleResult::*;
use RelationalType::*;
use PartType::*;

#[derive(Clone)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>
}

#[derive(Clone)]
pub struct Rule {
    pub variant: Option<PartType>,
    pub operator: Option<RelationalType>,
    pub rule_threshold: Option<u64>,
    pub true_result: RuleResult,
}

impl Rule {
    pub fn invert_rule(rule: &Rule) -> Rule {
        let inverted_operator = match rule.operator {
            Some(GreaterThan) => Some(LessThan),
            Some(LessThan) => Some(GreaterThan),
            None => None
        };

        return Rule {
            variant: rule.variant.clone(),
            operator: inverted_operator,
            rule_threshold: rule.rule_threshold.clone(),
            true_result: rule.true_result.clone()
        };
    }

    pub fn evaluate(&self, input_parts: &AggregatePart) -> Vec<(RuleResult, AggregatePart)> {
        //Short circuit for automatic result
        if self.variant.is_none() {
            return vec![(self.true_result.clone(), (*input_parts).clone())];
        }

        let mut lower_bound_parts = (*input_parts).clone();
        let mut upper_bound_parts = (*input_parts).clone();

        let (lower_bound_arguments, upper_bound_arguments): ((&mut u64, &mut u64), (&mut u64, &mut u64)) = match self.variant {
            Some(ExtremelyCool) => ((&mut lower_bound_parts.x_lower_bound, &mut lower_bound_parts.x_upper_bound), (&mut upper_bound_parts.x_lower_bound, &mut upper_bound_parts.x_upper_bound)),
            Some(Musical) => ((&mut lower_bound_parts.m_lower_bound, &mut lower_bound_parts.m_upper_bound), (&mut upper_bound_parts.m_lower_bound, &mut upper_bound_parts.m_upper_bound)),
            Some(Aerodynamic) => ((&mut lower_bound_parts.a_lower_bound, &mut lower_bound_parts.a_upper_bound), (&mut upper_bound_parts.a_lower_bound, &mut upper_bound_parts.a_upper_bound)),
            Some(Shiny) => ((&mut lower_bound_parts.s_lower_bound, &mut lower_bound_parts.s_upper_bound), (&mut upper_bound_parts.s_lower_bound, &mut upper_bound_parts.s_upper_bound)),
            None => panic!("Cannot evaluate rule since variant is None")
        };

        let mut lower_bound_parts_optional = None;
        let mut upper_bound_parts_optional = None;

        match self.operator.unwrap() {
            LessThan => {
                if *lower_bound_arguments.0 <= self.rule_threshold.unwrap() && *lower_bound_arguments.1 <= self.rule_threshold.unwrap() {
                    lower_bound_parts_optional = Some(lower_bound_parts);
                } else if *lower_bound_arguments.0 <= self.rule_threshold.unwrap() && *lower_bound_arguments.1 >= self.rule_threshold.unwrap() {
                    *lower_bound_arguments.1 = self.rule_threshold.unwrap();
                    lower_bound_parts_optional = Some(lower_bound_parts);
                }
            },
            GreaterThan => {
                if *upper_bound_arguments.0 <= self.rule_threshold.unwrap() && *upper_bound_arguments.1 >= self.rule_threshold.unwrap() {
                    *upper_bound_arguments.0 = self.rule_threshold.unwrap();
                    upper_bound_parts_optional = Some(upper_bound_parts);
                } else {
                    upper_bound_parts_optional = Some(upper_bound_parts);
                }
            }
        }

        let mut results = Vec::<(RuleResult, AggregatePart)>::new();
        if let Some(lower_bound_parts_value) = lower_bound_parts_optional {
            results.push((self.true_result.clone(), lower_bound_parts_value));
        } else {
            results.push((Reject, AggregatePart::empty_part()))
        }

        if let Some(upper_bound_parts_value) = upper_bound_parts_optional {
            results.push((self.true_result.clone(), upper_bound_parts_value));
        } else {
            results.push((Reject, AggregatePart::empty_part()))
        }

        return results;
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

#[derive(Clone)]
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

    pub fn empty_part() -> Self {
        return AggregatePart {
            x_lower_bound: 0u64,
            x_upper_bound: 0u64,
            m_lower_bound: 0u64,
            m_upper_bound: 0u64,
            a_lower_bound: 0u64,
            a_upper_bound: 0u64,
            s_lower_bound: 0u64,
            s_upper_bound: 0u64
        };
    }

    pub fn all_parts() -> Self {
        //Bound are open so add/subtract 1 to min/max part numbers
        return AggregatePart {
            x_lower_bound: Self::MIN_PART_NUMBER - 1,
            x_upper_bound: Self::MAX_PART_NUMBER + 1,
            m_lower_bound: Self::MIN_PART_NUMBER - 1,
            m_upper_bound: Self::MAX_PART_NUMBER + 1,
            a_lower_bound: Self::MIN_PART_NUMBER - 1,
            a_upper_bound: Self::MAX_PART_NUMBER + 1,
            s_lower_bound: Self::MIN_PART_NUMBER - 1,
            s_upper_bound: Self::MAX_PART_NUMBER + 1,
        };
    }

    pub fn get_parts_combinations(&self) -> u64 {
        let bound_range = |lb: u64, ub: u64| if ub - lb <= 2 { 0 } else { (ub - 1) - (lb + 1) + 1 };
        let bound_pairs = [(self.x_lower_bound, self.x_upper_bound), (self.m_lower_bound, self.m_upper_bound),
            (self.a_lower_bound, self.a_upper_bound), (self.s_lower_bound, self.s_upper_bound)];

        return bound_pairs
            .iter()
            .map(|x| bound_range(x.0, x.1))
            .filter(|x| *x > 0)
            .product::<u64>();
    }
}
