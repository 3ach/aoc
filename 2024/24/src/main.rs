use adventage::{day, part1demo};
use std::collections::HashMap;

day!(2024, 24);
part1demo!(
    "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
    4
);
part1demo!(
    "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
    2024
);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate {
    left: String,
    right: String,
    output: String,
    operation: Operation,
}

type TInput = (Vec<(String, bool)>, Vec<Gate>);

fn parse(input: &str) -> TInput {
    let (initial, gates) = input.split_once("\n\n").unwrap();
    let initial = initial
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            (String::from(name), value == "1")
        })
        .collect();

    let gates = gates
        .lines()
        .map(|gate| {
            let parts: Vec<&str> = gate.split_whitespace().collect();
            let operation = match parts[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("Unknown operation!"),
            };
            Gate {
                left: String::from(parts[0]),
                right: String::from(parts[2]),
                output: String::from(parts[4]),
                operation: operation,
            }
        })
        .collect();

    (initial, gates)
}

fn dependencies(gates: &Vec<Gate>) -> HashMap<String, Vec<Gate>> {
    let mut deps = HashMap::new();
    for gate in gates {
        deps.entry(gate.left.clone())
            .or_insert(vec![])
            .push(gate.clone());
        deps.entry(gate.right.clone())
            .or_insert(vec![])
            .push(gate.clone());
    }
    deps
}

fn run(gate: &Gate, left: bool, right: bool) -> bool {
    match &gate.operation {
        Operation::And => left & right,
        Operation::Or => left | right,
        Operation::Xor => left ^ right,
    }
}
fn simulate(gates: &Vec<Gate>, inputs: &Vec<(String, bool)>) -> u64 {
    let deps = dependencies(gates);
    let mut signals = HashMap::new();
    let mut changed = inputs.clone();

    while let Some((signal, value)) = changed.pop() {
        if signals.insert(signal.clone(), value).is_some() {
            panic!("Tried to change an existing signal {signal}");
        }

        if let Some(dependents) = deps.get(&signal) {
            for dependent in dependents {
                if signals.contains_key(&dependent.left)
                    && signals.contains_key(&dependent.right)
                    && !signals.contains_key(&dependent.output)
                {
                    let left = *signals.get(&dependent.left).unwrap();
                    let right = *signals.get(&dependent.right).unwrap();
                    let output = run(dependent, left, right);
                    changed.push((dependent.output.clone(), output));
                }
            }
        }
    }

    signals
        .iter()
        .filter(|(name, _)| name.starts_with("z") && name[1..].parse::<usize>().is_ok())
        .fold(0, |z, (name, value)| {
            let bit_position: usize = name[1..].parse().unwrap();
            let value = if *value { 1 } else { 0 };
            z | (value << bit_position)
        })
}

fn part1((initial, gates): &TInput) -> u64 {
    simulate(gates, initial)
}

fn find<'a>(gates: &'a Vec<Gate>, left: &str, right: &str, op: Operation) -> Option<&'a Gate> {
    let candidates = gates
        .iter()
        .filter(|g| (g.left == left && g.right == right) || (g.right == left && g.left == right))
        .filter(|g| g.operation == op)
        .collect::<Vec<&Gate>>();
    if candidates.len() != 1 {
        None
    } else {
        Some(candidates[0])
    }
}

fn part2((_, gates): &TInput) -> String {
    let max_input = gates
        .iter()
        .map(|gate| vec![&gate.left, &gate.right])
        .flatten()
        .map(|signal| signal[1..].parse::<usize>().unwrap_or(0))
        .max()
        .unwrap();
    let mut carry: Option<String> = None;
    let mut swaps = vec![];
    for bit in 0..=max_input {
        let x_in = format!("x{bit:02}");
        let y_in = format!("y{bit:02}");
        let z_out = format!("z{bit:02}");
        let xor = find(gates, &x_in, &y_in, Operation::Xor).unwrap();
        let mut and = find(gates, &x_in, &y_in, Operation::And).unwrap();

        let carrier = if let Some(ref carry_in) = carry {
            let xor_with_carry = find(gates, &xor.output, carry_in, Operation::Xor);
            let and_with_carry = find(gates, &xor.output, carry_in, Operation::And);

            let (xor_with_carry, and_with_carry) =
                if xor_with_carry.is_some() && and_with_carry.is_some() {
                    (xor_with_carry.unwrap(), and_with_carry.unwrap())
                } else if xor_with_carry.is_none() && and_with_carry.is_none() {
                    let out = (
                        find(gates, &and.output, carry_in, Operation::Xor).unwrap(),
                        find(gates, &and.output, carry_in, Operation::And).unwrap(),
                    );
                    swaps.push((and.output.clone(), xor.output.clone()));
                    and = xor;
                    out
                } else {
                    panic!("{xor_with_carry:?}, {and_with_carry:?}");
                };

            if xor_with_carry.output == z_out {
                find(gates, &and.output, &and_with_carry.output, Operation::Or).unwrap()
            } else if and_with_carry.output == z_out {
                swaps.push((xor_with_carry.output.clone(), and_with_carry.output.clone()));
                find(gates, &and.output, &xor_with_carry.output, Operation::Or).unwrap()
            } else if and.output == z_out {
                swaps.push((and.output.clone(), xor_with_carry.output.clone()));
                find(
                    gates,
                    &and_with_carry.output,
                    &xor_with_carry.output,
                    Operation::Or,
                )
                .unwrap()
            } else {
                let wrong_carry =
                    find(gates, &and.output, &and_with_carry.output, Operation::Or).unwrap();
                if wrong_carry.output == z_out {
                    swaps.push((xor_with_carry.output.clone(), wrong_carry.output.clone()));
                    xor_with_carry
                } else {
                    panic!()
                }
            }
        } else {
            if xor.output != "z00" {
                panic!("Must have a carry-in for all digits > 0");
            }
            and
        };

        carry = Some(carrier.output.clone())
    }

    let mut swaps: Vec<String> = swaps
        .iter()
        .map(|s| vec![s.0.clone(), s.1.clone()])
        .flatten()
        .collect();
    swaps.sort();

    swaps.join(",")
}
