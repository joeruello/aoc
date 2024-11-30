use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};

fn main() {
    let input: String = common::AocInput::fetch(2023, 2).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();

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
                        Rule(Condition::Always, rule.parse().unwrap())
                    }
                })
                .collect();

            (name.to_string(), rules)
        })
        .collect();

    let initial = HashMap::from([
        (Field::X, Range(1, 4000)),
        (Field::M, Range(1, 4000)),
        (Field::A, Range(1, 4000)),
        (Field::S, Range(1, 4000)),
    ]);

    let mut queue = VecDeque::from([(initial, workflows.get("in").unwrap())]);
    let mut accepted = vec![];

    while let Some((part, workflow)) = queue.pop_front() {
        let mut remaining = part;
        for Rule(condition, action) in workflow {
            match condition {
                Condition::GreaterThan { field, value } | Condition::LessThan { field, value } => {
                    let (matching, not_matching) =
                        if matches!(condition, Condition::GreaterThan { .. }) {
                            remaining.get(field).unwrap().split_gt(*value)
                        } else {
                            remaining.get(field).unwrap().split_le(*value)
                        };

                    match (matching, action) {
                        (Some(matching), Action::Workflow(workflow)) => {
                            let mut next = remaining.clone();
                            next.entry(*field).and_modify(|r| *r = matching);
                            queue.push_front((next, workflows.get(workflow).unwrap()))
                        }
                        (Some(matching), Action::Approve) => {
                            let mut next = remaining.clone();
                            next.entry(*field).and_modify(|r| *r = matching);
                            accepted.push(next)
                        }
                        _ => {}
                    }

                    if let Some(not_matching) = not_matching {
                        remaining.entry(*field).and_modify(|r| *r = not_matching);
                    }
                }
                Condition::Always => {
                    match action {
                        Action::Workflow(workflow) => {
                            queue.push_front((remaining.clone(), workflows.get(workflow).unwrap()))
                        }
                        Action::Approve => accepted.push(remaining.clone()),
                        _ => {}
                    }
                    break;
                }
            }
        }
    }

    accepted
        .into_iter()
        .map(|part| part.values().map(|r| r.size()).product::<usize>())
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Range(usize, usize);

impl Range {
    fn split_gt(&self, value: usize) -> (Option<Range>, Option<Range>) {
        if self.0 > value {
            (Some(*self), None)
        } else if self.1 < value {
            (None, Some(*self))
        } else {
            (Some(Range(value + 1, self.1)), Some(Range(self.0, value)))
        }
    }

    fn split_le(&self, value: usize) -> (Option<Range>, Option<Range>) {
        if self.1 < value {
            (Some(*self), None)
        } else if self.0 > value {
            (None, Some(*self))
        } else {
            (Some(Range(self.0, value - 1)), Some(Range(value, self.1)))
        }
    }

    fn size(&self) -> usize {
        (self.1 + 1) - self.0
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 167409079868000);
    }
}
