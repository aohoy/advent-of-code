use std::{cmp, collections::HashMap, ops::RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending, one_of},
    combinator::{complete, opt},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple, Tuple},
    IResult,
};

advent_of_code::solution!(19);

struct Parts {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Action {
    Accept,
    Reject,
    Workflow(String),
}

impl TryFrom<&str> for Action {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Action::Accept),
            "R" => Ok(Action::Reject),
            _ => Ok(Action::Workflow(value.to_string())),
        }
    }
}

enum ConditionRangeResult {
    Next(Action),
    Reject,
    Split((PartsRange, Action), PartsRange),
}

#[derive(Debug, Clone)]
enum Condition {
    Test(char, cmp::Ordering, u64, Action),
    Target(Action),
}

impl Condition {
    fn apply(&self, part: &Parts) -> Option<Action> {
        match self {
            Condition::Test(field, op, num, action) => {
                let f = match field {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => unreachable!(),
                };
                (f.cmp(num) == *op).then_some(action.clone())
            }
            Condition::Target(action) => Some(action.clone()),
        }
    }

    fn test_range(&self, part: &PartsRange) -> ConditionRangeResult {
        match self {
            Condition::Test(field, op, num, action) => {
                let f = match field {
                    'x' => &part.x,
                    'm' => &part.m,
                    'a' => &part.a,
                    's' => &part.s,
                    _ => unreachable!(),
                };

                if f.contains(num) {
                    let (mut part_low, mut part_high) = (part.clone(), part.clone());
                    match op {
                        cmp::Ordering::Greater => {
                            let (low, high) = (*f.start()..=*num, *num + 1..=*f.end());
                            part_low.set(*field, low);
                            part_high.set(*field, high);
                            ConditionRangeResult::Split((part_high, action.clone()), part_low)
                        }
                        cmp::Ordering::Less => {
                            let (low, high) = (*f.start()..=*num - 1, *num..=*f.end());
                            part_low.set(*field, low);
                            part_high.set(*field, high);
                            ConditionRangeResult::Split((part_low, action.clone()), part_high)
                        }
                        _ => unreachable!(),
                    }
                } else {
                    match op {
                        cmp::Ordering::Greater if f.start() > num => {
                            ConditionRangeResult::Next(action.clone())
                        }
                        cmp::Ordering::Less if f.end() < num => {
                            ConditionRangeResult::Next(action.clone())
                        }
                        _ => ConditionRangeResult::Reject,
                    }
                }
            }
            Condition::Target(action) => ConditionRangeResult::Next(action.clone()),
        }
    }
}

type Workflow = Box<dyn Fn(&Parts) -> Action>;
struct Workflows(HashMap<Action, (Workflow, Vec<Condition>)>);

impl Workflows {
    fn new() -> Self {
        Workflows(HashMap::new())
    }

    fn run(&self, part: &Parts) -> bool {
        let mut action = Action::Workflow("in".to_string());
        while let Action::Workflow(_) = action {
            let (action_run, _) = self.0.get(&action).unwrap();
            action = action_run(part);
        }
        match action {
            Action::Accept => true,
            Action::Reject => false,
            _ => unreachable!(),
        }
    }
}

fn get_workflow(conditions: Vec<Condition>) -> Workflow {
    Box::new(move |parts| {
        conditions
            .iter()
            .find_map(|cond| cond.apply(parts))
            .expect("incorrect workflow")
    })
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, (name, op, value, action)) = tuple((
        one_of("xmas"),
        one_of("<>"),
        complete::u64,
        preceded(tag(":"), alpha1),
    ))(input)?;
    let ord = match op {
        '<' => cmp::Ordering::Less,
        '>' => cmp::Ordering::Greater,
        _ => unreachable!(),
    };
    Ok((
        input,
        Condition::Test(name, ord, value, Action::try_from(action).unwrap()),
    ))
}

fn parse_workflow_inside(input: &str) -> IResult<&str, (Workflow, Vec<Condition>)> {
    let (input, (mut conditions, last_action)) =
        separated_pair(separated_list1(tag(","), parse_condition), tag(","), alpha1)(input)?;
    conditions.push(Condition::Target(Action::try_from(last_action).unwrap()));
    Ok((input, (get_workflow(conditions.clone()), conditions)))
}

fn parse_workflow(input: &str) -> IResult<&str, (Action, (Workflow, Vec<Condition>))> {
    let (input, (action, workflow)) =
        tuple((alpha1, delimited(tag("{"), parse_workflow_inside, tag("}"))))(input)?;
    Ok((input, (Action::try_from(action).unwrap(), workflow)))
}

fn parse_workflows(input: &str) -> IResult<&str, Workflows> {
    fold_many1(
        terminated(parse_workflow, line_ending),
        Workflows::new,
        |mut acc: Workflows, (name, func)| {
            acc.0.insert(name, func);
            acc
        },
    )(input)
}

fn parse_part(input: &str) -> IResult<&str, Parts> {
    let (input, (x, m, a, s)) = delimited(
        tag("{"),
        tuple((
            delimited(tag("x="), complete::u64, opt(tag(","))),
            delimited(tag("m="), complete::u64, opt(tag(","))),
            delimited(tag("a="), complete::u64, opt(tag(","))),
            delimited(tag("s="), complete::u64, opt(tag(","))),
        )),
        tag("}"),
    )(input)?;

    Ok((input, Parts { x, m, a, s }))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Parts>> {
    separated_list1(line_ending, parse_part)(input)
}

fn parse(input: &str) -> IResult<&str, (Workflows, Vec<Parts>)> {
    separated_pair(parse_workflows, line_ending, parse_parts)(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (workflows, parts)) = parse(input).unwrap();
    let res = parts
        .iter()
        .filter(|p| workflows.run(p))
        .flat_map(|p| [p.x, p.m, p.a, p.s].into_iter())
        .sum::<u64>();
    Some(res)
}

#[derive(Debug, Clone)]
struct PartsRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

impl PartsRange {
    fn set(&mut self, field: char, value: RangeInclusive<u64>) {
        match field {
            'x' => self.x = value,
            'm' => self.m = value,
            'a' => self.a = value,
            's' => self.s = value,
            _ => unreachable!(),
        }
    }
}

fn process(part: PartsRange, workflows: &Workflows, target: &Action) -> u64 {
    match target {
        Action::Workflow(_) => {
            let current_workflow = workflows.0.get(target).expect("incorrect target");
            let mut curr = part;
            let mut sum = 0;
            for cond in current_workflow.1.iter() {
                match cond.test_range(&curr) {
                    ConditionRangeResult::Next(action) => {
                        sum += process(curr.clone(), workflows, &action);
                        break;
                    }
                    ConditionRangeResult::Split((passed, action), failed) => {
                        sum += process(passed, workflows, &action);
                        curr = failed;
                    }
                    ConditionRangeResult::Reject => {}
                }
            }
            sum
        }
        Action::Accept => {
            println!("{:?}", part);
            (part.x.end() - part.x.start() + 1)
                * (part.m.end() - part.m.start() + 1)
                * (part.a.end() - part.a.start() + 1)
                * (part.s.end() - part.s.start() + 1)
        }
        Action::Reject => 0,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (workflows, _)) = parse(input).unwrap();
    let part = PartsRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };
    let res = process(part, &workflows, &Action::Workflow("in".to_string()));
    Some(res)
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
        assert_eq!(result, Some(167409079868000));
    }
}
