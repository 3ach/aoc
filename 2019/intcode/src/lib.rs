
pub type Program = Vec<i32>;
#[derive(Debug)]

pub struct Execution {
    ip: usize,
    program: Program,
    input: Vec<i32>,
    output: Vec<i32>,
    halted: bool,
}

fn jump_if_true(first: i32, second: i32, execution: &mut Execution) {
    //println!("\t JUMP to {} IF [{}]", second, first);

    execution.ip = if first != 0 {
        let second: usize = second as usize;
        assert!(second < execution.program.len());
        second
    } else {
        execution.ip + 3
    };
}

fn jump_if_false(first: i32, second: i32, execution: &mut Execution) {
    //println!("\t JUMP to {} UNLESS [{}]", second, first);

    execution.ip = if first == 0 {
        let second: usize = second as usize;
        assert!(second < execution.program.len());
        second
    } else {
        execution.ip + 3
    };
}

fn less_than(first: i32, second: i32, third: i32, execution: &mut Execution) {
    //println!("\t {} < {} -> [{}]", first, second, third);
    let third: usize = third as usize;
    assert!(third < execution.program.len());

    if first < second {
        execution.program[third] = 1;
    } else {
        execution.program[third] = 0;
    }

    execution.ip += 4;
}

fn equals(first: i32, second: i32, third: i32, execution: &mut Execution) {
    //println!("\t {} == {} -> [{}]", first, second, third);
    let third: usize = third as usize;
    assert!(third < execution.program.len());

    if first == second {
        execution.program[third] = 1;
    } else {
        execution.program[third] = 0;
    }

    execution.ip += 4;
}

fn add(first: i32, second: i32, third: i32, execution: &mut Execution) {
    //println!("\t {} + {} -> [{}]", first, second, third);
    let third: usize = third as usize;
    assert!(third < execution.program.len());
    execution.program[third] = first + second;
    execution.ip += 4;
}

fn multiply(first: i32, second: i32, third: i32, execution: &mut Execution) {
    //println!("\t {} * {} -> [{}]", first, second, third);
    let third: usize = third as usize;
    assert!(third < execution.program.len());
    execution.program[third] = first * second;
    execution.ip += 4;
}

fn input(param: i32, execution: &mut Execution) -> bool {
    let address: usize = param as usize;
    assert!(address < execution.program.len());

    if execution.input.is_empty() {
        //println!("Suspending.");
        return true;
    }


    let input = execution.input.remove(0);
    //println!("\t {} IN -> [{}]", input, param);
    execution.program[address] = input;
    execution.ip += 2;
    false
}

fn output(param: i32, execution: &mut Execution) {
    //println!("\t [{}] -> OUT", param);
    execution.output.push(param);
    execution.ip += 2;
}

fn two_parameter_with_result(
    execution: &mut Execution,
    instruction: i32,
    operation: fn(i32, i32, i32, &mut Execution),
) -> bool {
    let program = &execution.program;
    let ip = execution.ip;

    let first = match (instruction % 1000) / 100 {
        0 => program[program[ip + 1] as usize],
        1 => program[ip + 1],
        _ => panic!(),
    };

    let second = match (instruction % 10000) / 1000 {
        0 => program[program[ip + 2] as usize],
        1 => program[ip + 2],
        _ => panic!(),
    };

    let third = program[ip + 3];

    operation(first, second, third, execution);
    false
}

fn two_parameter_no_result(
    execution: &mut Execution,
    instruction: i32,
    operation: fn(i32, i32, &mut Execution),
) -> bool {
    let ip = execution.ip;
    let program = &execution.program;

    let first = match (instruction % 1000) / 100 {
        0 => program[program[ip + 1] as usize],
        1 => program[ip + 1],
        _ => panic!(),
    };

    let second = match (instruction % 10000) / 1000 {
        0 => program[program[ip + 2] as usize],
        1 => program[ip + 2],
        _ => panic!(),
    };

    operation(first, second, execution);
    false
}

fn one_parameter_no_result(
    execution: &mut Execution,
    instruction: i32,
    operation: fn(i32, &mut Execution),
) -> bool {
    let ip = execution.ip;
    let program = &execution.program;

    let first = match (instruction % 1000) / 100 {
        0 => program[program[ip + 1] as usize],
        1 => program[ip + 1],
        _ => panic!(),
    };

    operation(first, execution);
    false
}

fn result(
    execution: &mut Execution,
    _instruction: i32,
    operation: fn(i32, &mut Execution) -> bool,
) -> bool {
    let param = execution.program[execution.ip + 1];

    operation(param, execution)
}

fn interp(mut execution: &mut Execution) {
    loop {
        assert!(execution.ip < execution.program.len());
        let instruction = execution.program[execution.ip];

        let suspend = match instruction % 100 {
            1 => two_parameter_with_result(&mut execution, instruction, add),
            2 => two_parameter_with_result(&mut execution, instruction, multiply),
            3 => result(&mut execution, instruction, input),
            4 => one_parameter_no_result(&mut execution, instruction, output),
            5 => two_parameter_no_result(&mut execution, instruction, jump_if_true),
            6 => two_parameter_no_result(&mut execution, instruction, jump_if_false),
            7 => two_parameter_with_result(&mut execution, instruction, less_than),
            8 => two_parameter_with_result(&mut execution, instruction, equals),
            99 => { execution.halted = true; return },
            _ => panic!("Unsupported instruction {}", instruction),
        };

        if suspend {
            return
        }
    }
}

pub fn enter<'a, 'b>(execution: &'a mut Execution, input: &'b [i32]) -> (bool, Vec<i32>, &'a mut Execution) {
    assert!(!execution.halted);
    execution.input.extend_from_slice(input);
    interp(execution);

    (execution.halted, execution.output.drain(0..).collect(), execution)
}

pub fn init(program: &Program, input: &[i32]) -> Execution {
    Execution { ip: 0, program: program.clone(), input: Vec::from(input), output: vec![], halted: false }
}

pub fn run(program: &Program, input: &[i32]) -> Vec<i32> {
    let mut execution = init(program, input);
    interp(&mut execution);
    execution.output
}
