use std::collections::HashMap;

advent_of_code::solution!(19);

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
    
    fn step_through_workflow<'a>(&'a self, workflow: &'a Workflow) -> &Step {
        // go through each rule in the workflow, and check if it passes
        // if none pass, return the default step
        for rule in &workflow.rules {
            let variable = match rule.condition.variable {
                PartIdent::X => self.x,
                PartIdent::M => self.m,
                PartIdent::A => self.a,
                PartIdent::S => self.s,
            };
            
            let constant = rule.condition.constant;
            
            let passes = match rule.condition.operator {
                Operator::LessThan => variable < constant,
                Operator::GreaterThan => variable > constant,
            };
            
            if passes {
                return &rule.next_step;
            }
        }
        
        &workflow.default
    }
    
    fn get_accepted(&self, start_name: &str, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut workflow = workflows.get(start_name).unwrap();
        
        loop {
            match self.step_through_workflow(workflow) {
                Step::Workflow(workflow_name) => {
                    workflow = workflows.get(workflow_name).unwrap();
                },
                Step::Accept => {
                    return true;
                },
                Step::Reject => {
                    return false;
                },
            }
        }
    }
}

enum Step<'a> {
    Workflow(&'a str),
    Accept,
    Reject,
}

enum Operator {
    LessThan,
    GreaterThan,
}

enum PartIdent {
    X,
    M,
    A,
    S,
}

// conditions compare a variable of (x, m, a, or s) and a constant, and can be one of (<, >)
struct Condition {
    variable: PartIdent,
    constant: u32,
    operator: Operator,
}

struct Rule<'a> {
    condition: Condition,
    next_step: Step<'a>,
}

struct Workflow<'a, 'b> {
    rules: Vec<Rule<'a>>,
    default: Step<'b>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    
    // split into two on empty line
    // first part is rules
    // second part is part ratings
    // example:
    // ```
    // hdj{m>838:A,pv}
    //
    // {x=787,m=2655,a=1222,s=2876}
    // ```
    
    let mut two_parts = lines.split(|line| line.is_empty());
    let workflow_lines = two_parts.next().unwrap();
    let part_ratings_lines = two_parts.next().unwrap();
    
    // rules hashmap
    let mut workflows: HashMap<&str, Workflow> = HashMap::new();
    
    // parse rules
    for line in workflow_lines {
        let line = &line[..line.len() - 1];
        let mut parts = line.split("{");
        let workflow_name = parts.next().unwrap();
        let mut workflow_rules = parts.next().unwrap().split(',');

        let mut workflow = Workflow {
            rules: Vec::new(),
            default: match workflow_rules.next_back().unwrap() {
                "A" => Step::Accept,
                "R" => Step::Reject,
                default_workflow_name => Step::Workflow(default_workflow_name),
            },
        };

        for rules in workflow_rules {
            let mut parts = rules.split(":");
            let rule_condition_str = parts.next().unwrap();
            let next_workflow_name = parts.next().unwrap();
            
            let rule_variable = match rule_condition_str.chars().nth(0).unwrap() {
                'x' => PartIdent::X,
                'm' => PartIdent::M,
                'a' => PartIdent::A,
                's' => PartIdent::S,
                _ => panic!("invalid variable"),
            };
            
            let rule_operator = match rule_condition_str.chars().nth(1).unwrap() {
                '<' => Operator::LessThan,
                '>' => Operator::GreaterThan,
                _ => panic!("invalid operator"),
            };
            
            let rule_constant = rule_condition_str[2..].parse::<u32>().unwrap();
            
            let rule_condition = Condition {
                variable: rule_variable,
                operator: rule_operator,
                constant: rule_constant,
            };

            let rule_next_step = match next_workflow_name {
                "A" => Step::Accept,
                "R" => Step::Reject,
                _ => Step::Workflow(next_workflow_name),
            };

            let rule = Rule {
                condition: rule_condition,
                next_step: rule_next_step,
            };
            
            workflow.rules.push(rule);
        }
        
        workflows.insert(workflow_name, workflow);
    }
    
    let part_successes = part_ratings_lines.iter()
        .map(|line| {
            let line = &line[1..line.len() - 1];
            let mut ratings_raw = line.split(",");
            Part {
                x: ratings_raw.next().unwrap().split('=').last().unwrap().parse().unwrap(),
                m: ratings_raw.next().unwrap().split('=').last().unwrap().parse().unwrap(),
                a: ratings_raw.next().unwrap().split('=').last().unwrap().parse().unwrap(),
                s: ratings_raw.next().unwrap().split('=').last().unwrap().parse().unwrap(),
            }
        })
        .filter_map(|part| {
            if part.get_accepted("in", &workflows) {
                Some(part.sum())
            } else {
                None
            }
        });
    
    Some(part_successes.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let _lines = input.lines().collect::<Vec<&str>>();
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
