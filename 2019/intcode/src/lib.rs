use std::convert::TryInto;

const MAX_PARAMETERS: usize = 3;

type ParameterModes = [i64; MAX_PARAMETERS];
type Instruction = (i64, ParameterModes);

fn parse_instruction(instruction: i64) -> Instruction {
    let opcode = instruction % 100;
    let mut modes_mask = instruction / 100;
    let mut modes = [0; MAX_PARAMETERS];
    for i in 0..MAX_PARAMETERS {
        modes[i] = modes_mask % 10;
        modes_mask /= 10;
    }
    (opcode, modes)
}

fn get_parameter_value(index: usize, pc: usize, mem: &[i64], modes: &ParameterModes) -> i64 {
    match modes[index - 1] {
        // Position mode
        0 => {
            let addr: usize = mem[pc + index].try_into().unwrap();
            mem[addr]
        },

        // Immediate mode
        1 => mem[pc + index],

        // Other modes don't exist
        _ => unreachable!(),
    }
}

fn get_binary_op_params(pc: usize, mem: &[i64], modes: &ParameterModes) -> (i64, i64, usize) {
    let a = get_parameter_value(1, pc, mem, modes);
    let b = get_parameter_value(2, pc, mem, modes);
    let dest = mem[pc + 3].try_into().unwrap();

    (a, b, dest)
}

pub fn interpret(mem: &mut [i64]) {
    let mut pc = 0;
    loop {
        let (opcode, modes) = parse_instruction(mem[pc]);
        match opcode {
            1 => {
                let (a, b, dest) = get_binary_op_params(pc, mem, &modes);
                mem[dest] = a + b;
                pc += 4;
            }
            2 => {
                let (a, b, dest) = get_binary_op_params(pc, mem, &modes);
                mem[dest] = a * b;
                pc += 4;
            }
            99 => break,
            _ => panic!("Uknown opcode: {}", opcode),
        }
    }
}