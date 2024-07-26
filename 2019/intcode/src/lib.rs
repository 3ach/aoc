
pub type Program = Vec<i64>;

#[derive(Debug, Clone)]
pub struct Execution {
    ip: usize,
    base: i64,
    program: Program,
    input: Vec<i64>,
    output: Vec<i64>,
    halted: bool,
}

fn get_memory(execution: &mut Execution, address: usize) -> i64 {
    let address = address as usize;

    if execution.program.len() <= address {
        execution.program.resize(address + 1, 0);
    }

    execution.program[address]
}

fn set_memory(execution: &mut Execution, address: usize, value: i64) {
    let address = address as usize;

    if execution.program.len() <= address {
        execution.program.resize(address + 1, 0);
    }

    execution.program[address] = value;
}

fn jump_if_true(first: i64, second: i64, execution: &mut Execution) {
    //println!("\t JUMP to {} IF [{}]", second, first);

    execution.ip = if first != 0 {
        let second: usize = second as usize;
        assert!(second < execution.program.len());
        second
    } else {
        execution.ip + 3
    };
}

fn jump_if_false(first: i64, second: i64, execution: &mut Execution) {
    //println!("\t JUMP to {} UNLESS [{}]", second, first);

    execution.ip = if first == 0 {
        let second: usize = second as usize;
        assert!(second < execution.program.len());
        second
    } else {
        execution.ip + 3
    };
}

fn less_than(first: i64, second: i64, third: i64, execution: &mut Execution) {
    //println!("\t {} < {} -> [{}]", first, second, third);
    if first < second {
        set_memory(execution, third as usize, 1);
    } else {
        set_memory(execution, third as usize, 0);
    }

    execution.ip += 4;
}

fn equals(first: i64, second: i64, third: i64, execution: &mut Execution) {
    //println!("\t {} == {} -> [{}]", first, second, third);

    if first == second {
        set_memory(execution, third as usize, 1);
    } else {
        set_memory(execution, third as usize, 0);
    }

    execution.ip += 4;
}

fn add(first: i64, second: i64, third: i64, execution: &mut Execution) {
    //println!("\t {} + {} -> [{}]", first, second, third);

    set_memory(execution, third as usize, first + second);
    execution.ip += 4;
}

fn multiply(first: i64, second: i64, third: i64, execution: &mut Execution) {
    //println!("\t {} * {} -> [{}]", first, second, third);

    set_memory(execution, third as usize, first * second);
    execution.ip += 4;
}

fn input(param: i64, execution: &mut Execution) -> bool {
    if execution.input.is_empty() {
        //println!("Suspending.");
        return true;
    }

    let input = execution.input.remove(0);
    //println!("\t {} IN -> [{}]", input, param);
    set_memory(execution, param as usize, input);
    execution.ip += 2;
    false
}

fn output(param: i64, execution: &mut Execution) {
    //println!("\t [{}] -> OUT", param);
    execution.output.push(param);
    execution.ip += 2;
}

fn base(param: i64, execution: &mut Execution) {
    //println!("\t SET BASE TO {}", param);
    execution.base += param;
    execution.ip += 2;
}

fn two_parameter_with_result(
    execution: &mut Execution,
    instruction: i64,
    operation: fn(i64, i64, i64, &mut Execution),
) -> bool {
    let ip = execution.ip;
    //print!("\t{:?}, ", &execution.program[ip..=ip + 3]); 

    let first_param = get_memory(execution, ip + 1);
    let first = match (instruction % 1000) / 100 {
        0 => get_memory(execution, first_param as usize),
        1 => first_param,
        2 => get_memory(execution, (first_param  + execution.base) as usize),
        _ => panic!(),
    };

    let second_param = get_memory(execution, ip + 2);
    let second = match (instruction % 10000) / 1000 {
        0 => get_memory(execution, second_param as usize),
        1 => second_param,
        2 => get_memory(execution, (second_param + execution.base) as usize),
        _ => panic!(),
    };

    let third  = match (instruction % 100000) / 10000 {
        0 => get_memory(execution, ip + 3),
        1 => panic!("Immediate mode not supported for results"),
        2 => get_memory(execution, ip + 3) + execution.base,
        _ => panic!(),
    };

    //println!("interpreted as {} with args {} {} {}", instruction % 1000, first, second, third); 

    operation(first, second, third, execution);
    false
}

fn two_parameter_no_result(
    execution: &mut Execution,
    instruction: i64,
    operation: fn(i64, i64, &mut Execution),
) -> bool {
    let ip = execution.ip;

    let first_param = get_memory(execution, ip + 1);
    let first = match (instruction % 1000) / 100 {
        0 => get_memory(execution, first_param as usize),
        1 => first_param,
        2 => get_memory(execution, (first_param + execution.base) as usize),
        _ => panic!(),
    };

    let second_param = get_memory(execution, ip + 2);
    let second = match (instruction % 10000) / 1000 {
        0 => get_memory(execution, second_param as usize),
        1 => second_param,
        2 => get_memory(execution, (second_param + execution.base) as usize),
        _ => panic!(),
    };

    operation(first, second, execution);
    false
}

fn one_parameter_no_result(
    execution: &mut Execution,
    instruction: i64,
    operation: fn(i64, &mut Execution),
) -> bool {
    let ip = execution.ip;

    let first_param = get_memory(execution, ip + 1);
    let first = match (instruction % 1000) / 100 {
        0 => get_memory(execution, first_param as usize),
        1 => first_param,
        2 => get_memory(execution, (first_param + execution.base) as usize),
        _ => panic!(),
    };

    //println!("\t{:?}, interpreted as {} with arg {}", &execution.program[ip..=ip + 1], instruction % 1000, first); 

    operation(first, execution);
    false
}

fn result(
    execution: &mut Execution,
    instruction: i64,
    operation: fn(i64, &mut Execution) -> bool,
) -> bool {
    let ip = execution.ip;

    let first_param = get_memory(execution, ip + 1);
    let param = match (instruction % 1000) / 100 {
        0 => first_param,
        1 => panic!("Immediate mode not supported for result"),
        2 => first_param + execution.base,
        _ => panic!(),
    };

    //println!("\t{:?}, interpreted as {} with arg {}", &execution.program[ip..=ip + 1], instruction % 1000, param); 

    operation(param, execution)
}

fn interp(mut execution: &mut Execution) {
    loop {
        assert!(execution.ip < execution.program.len());
        let instruction = get_memory(execution, execution.ip);

        let suspend = match instruction % 100 {
            1 => two_parameter_with_result(&mut execution, instruction, add),
            2 => two_parameter_with_result(&mut execution, instruction, multiply),
            3 => result(&mut execution, instruction, input),
            4 => one_parameter_no_result(&mut execution, instruction, output),
            5 => two_parameter_no_result(&mut execution, instruction, jump_if_true),
            6 => two_parameter_no_result(&mut execution, instruction, jump_if_false),
            7 => two_parameter_with_result(&mut execution, instruction, less_than),
            8 => two_parameter_with_result(&mut execution, instruction, equals),
            9 => one_parameter_no_result(&mut execution, instruction, base),
            99 => { execution.halted = true; return },
            _ => panic!("Unsupported instruction {}", instruction),
        };

        if suspend {
            return
        }
    }
}

pub fn enter<'a, 'b>(execution: &'a mut Execution, input: &'b [i64]) -> (bool, Vec<i64>) {
    assert!(!execution.halted);
    execution.input.extend_from_slice(input);
    interp(execution);

    (execution.halted, execution.output.drain(0..).collect())
}

pub fn init(program: &Program, input: &[i64]) -> Execution {
    Execution { ip: 0, base: 0, program: program.clone(), input: Vec::from(input), output: vec![], halted: false }
}

pub fn run(program: &Program, input: &[i64]) -> Vec<i64> {
    let mut execution = init(program, input);
    interp(&mut execution);
    execution.output
}
