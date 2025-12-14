use adventage::{day, part1demo, part2demo};
use good_lp::{highs, Expression, ProblemVariables, Solution, SolverModel, variable, Variable};
use std::collections::{HashSet, VecDeque};

day!(2025, 10);
part1demo!(
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    7
);
part2demo!(
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    33
);

#[derive(Debug)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    joltage: Vec<usize>,
}

type TInput = Vec<Machine>;

fn parse_machine(machine: &str) -> Machine {
    let mut lights = 0;
    let mut buttons = vec![];
    let mut joltage = vec![];

    for part in machine.split(" ") {
        let opener = &part[0..1];
        let value = &part[1..part.len() - 1];

        match opener {
            "[" => {
                lights = value
                    .chars()
                    .rev()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .fold(0, |v, bit| (v << 1) + bit);
            }
            "(" => {
                buttons.push(
                    value
                        .split(",")
                        .map(|v| 1 << v.parse::<usize>().unwrap())
                        .sum(),
                );
            }
            "{" => {
                joltage = value.split(",").map(|v| v.parse().unwrap()).collect();
            }
            _ => panic!(),
        }
    }

    Machine {
        lights,
        buttons,
        joltage,
    }
}

fn parse(input: &str) -> TInput {
    input.lines().map(parse_machine).collect()
}

fn activate(machine: &Machine) -> usize {
    let mut to_explore = VecDeque::from([(0, 0)]);
    let mut seen = HashSet::new();

    while let Some((current, steps)) = to_explore.pop_front() {
        if current == machine.lights {
            return steps;
        }

        if !seen.insert(current) {
            continue;
        }

        for button in &machine.buttons {
            let next = current ^ button;
            if !seen.contains(&next) {
                to_explore.push_back((next, steps + 1));
            }
        }
    }

    panic!();
}

fn part1(machines: &TInput) -> usize {
    machines.iter().map(activate).sum()
}

fn jolt(machine: &Machine) -> usize {
  let mut problem = ProblemVariables::new();
  let variables: Vec<Variable> = problem.add_vector(variable().integer().min(0), machine.buttons.len());
  let objective: Expression = variables.iter().sum();
  let mut model = problem.minimise(objective).using(highs);

  let max_light = machine.buttons.iter().map(|u| u.ilog2()).max().unwrap() as usize;
  for lidx in 0..=max_light {
      let constraint: Expression = variables.iter()
          .enumerate()
          .filter_map(|(bidx, var)| 
            if machine.buttons[bidx] & (1 << lidx) != 0 {
                Some(var)
            } else {
                None
            }).sum();

      model = model.with(constraint.eq(machine.joltage[lidx] as u32));
  }
  
  let solution = model.solve().unwrap();
  variables.iter()
      .map(|v| solution.value(*v).round() as usize)
      .sum()
}

fn part2(machines: &TInput) -> usize {
    machines
        .iter()
        .map(jolt)
        .sum()
}
