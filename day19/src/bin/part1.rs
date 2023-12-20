use std::{collections::HashMap, error::Error, str::FromStr, vec};

#[derive(Debug)]
struct Rule(Condition, Action);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl FromStr for Field {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Unknow field {s}"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    GreaterThan { field: Field, value: usize },
    LessThan { field: Field, value: usize },
    Always,
}

#[derive(Debug, Clone)]
enum Action {
    Workflow(String),
    Reject,
    Approve,
}

impl FromStr for Action {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Approve,
            "R" => Self::Reject,
            workflow => Self::Workflow(workflow.to_string()),
        })
    }
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Output: {}", process(input));
}

fn process(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Vec<Rule>> = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();

            let rules = rest[0..rest.len() - 1]
                .split(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let mut chars = rule.chars();
                        let field = chars.next().unwrap();
                        let operator = chars.next().unwrap();
                        let (value, result) = &rule[2..].split_once(':').unwrap();

                        match operator {
                            '>' => Rule(
                                Condition::GreaterThan {
                                    field: field.to_string().parse().unwrap(),
                                    value: value.parse().unwrap(),
                                },
                                result.parse().unwrap(),
                            ),
                            '<' => Rule(
                                Condition::LessThan {
                                    field: field.to_string().parse().unwrap(),
                                    value: value.parse().unwrap(),
                                },
                                result.parse().unwrap(),
                            ),
                            _ => unreachable!(),
                        }
                    } else {
                        dbg!(rule);
                        Rule(Condition::Always, rule.parse().unwrap())
                    }
                })
                .collect();

            (name.to_string(), rules)
        })
        .collect();


    let parts: Vec<HashMap<Field, usize>> = parts
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(',')
                .map(|field| {
                    let (field, value) = field.split_once('=').unwrap();
                    (field.parse().unwrap(), value.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut queue: Vec<(&Part, &Workflow)> = parts
        .iter()
        .map(|part| (part, workflows.get("in").unwrap()))
        .collect();


    dbg!(queue.len());
    let mut accepted = vec![];
    while let Some((part, workflow)) = queue.pop() {
        for Rule(condition, action) in workflow {
            match condition {
                Condition::GreaterThan { field, value } => {
                    if part.get(field).unwrap() > value {
                        match action {
                            Action::Workflow(workflow) => {
                                print!(" -> {workflow}");
                                queue.push((part, workflows.get(workflow).unwrap()));
                            }
                            Action::Reject => {
                                println!(" -> R");
                            }
                            Action::Approve =>  {
                                println!(" -> A");
                                accepted.push(part);
                            }
                        }
                        break;
                    }
                }
                Condition::LessThan { field, value } => {
                    if part.get(field).unwrap() < value {
                        match action {
                            Action::Workflow(workflow) => {
                                print!(" -> {workflow}");
                                queue.push((part, workflows.get(workflow).unwrap()));
                            }
                            Action::Reject => {
                                println!(" -> R");
                            }
                            Action::Approve =>  {
                                println!(" -> A");
                                accepted.push(part);
                            }
                        }
                        break;
                    }
                }
                Condition::Always => match action {
                    Action::Workflow(workflow) => {
                        print!(" -> {workflow}");
                        queue.push((part, workflows.get(workflow).unwrap()));
                    }
                    Action::Reject => {
                        println!(" -> R");
                    }
                    Action::Approve =>  {
                        println!(" -> A");
                        accepted.push(part)
                    }
                },
            }
        }
    }

    accepted.into_iter().flat_map(|part| part.values()).sum()
}

type Part = HashMap<Field, usize>;
type Workflow = Vec<Rule>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 19114);
    }
}
