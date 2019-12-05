use std::convert::TryInto;

const MAX_PARAMETERS: usize = 3;

type ParameterModes = [i64; MAX_PARAMETERS];
type Instruction = (i64, ParameterModes);

/// Parse a single instruction into an opcode and the parameter modes.
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

/// Get the value of a parameter, depending on the parameter mode.
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

/// Jump operators take two parameters (either position or immediate mode),
/// the second of which is an address to jump to.
fn get_jump_op_params(pc: usize, mem: &[i64], modes: &ParameterModes) -> (i64, usize) {
    let a = get_parameter_value(1, pc, mem, modes);
    let b = get_parameter_value(2, pc, mem, modes).try_into().unwrap();

    (a, b)
}

/// Ternary operators take two parameters (either position or immediate mode)
/// and write their output to the third parameter in position mode.
fn get_ternary_op_params(pc: usize, mem: &[i64], modes: &ParameterModes) -> (i64, i64, usize) {
    let a = get_parameter_value(1, pc, mem, modes);
    let b = get_parameter_value(2, pc, mem, modes);
    let c = mem[pc + 3].try_into().unwrap();

    (a, b, c)
}

/// Print a memory dump to stderr.
fn dump_memory(pc: usize, mem: &[i64]) {
    const SIZE: usize = 20;
    const PAD: usize = 7;

    eprintln!("{}", "-".repeat(SIZE * PAD));
    eprintln!("MEMORY DUMP");
    eprintln!("{}", "-".repeat(SIZE * PAD));

    for (start_addr, chunk) in mem.chunks(SIZE).enumerate().map(|(i, chunk)| (SIZE * i, chunk)) {
        for (i, _) in chunk.iter().enumerate() {
            eprint!("{: >7}", start_addr + i);
        }
        eprintln!();
        for val in chunk {
            eprint!("{: >7}", val);
        }
        eprintln!();
        if pc >= start_addr && pc < start_addr + SIZE {
            eprint!("{}", " ".repeat((pc - start_addr) * PAD + 1));
            eprintln!("{}", "^".repeat(PAD - 1));
        } else {
            eprintln!();
        }
    }

    eprintln!("{}", "-".repeat(SIZE * PAD));
}

/// Interpret an Intcode program stored in `mem`, using `inputs` to simulate input,
/// and return a vector of all output values.
pub fn interpret(mem: &mut [i64], inputs: &[String]) -> Vec<String> {
    let mut pc = 0;

    let mut input_index = 0;
    let mut outputs = Vec::new();

    loop {
        let (opcode, modes) = parse_instruction(mem[pc]);
        match opcode {
            // add
            1 | 2 => {
                let (a, b, dest) = get_ternary_op_params(pc, mem, &modes);

                match opcode {
                    1 => mem[dest] = a + b,
                    2 => mem[dest] = a * b,
                    // other opcodes are filtered by the outer match, so this is fine
                    _ => unreachable!(),
                }

                pc += 4;
            }
            // input
            3 => {
                let dest: usize = mem[pc + 1].try_into().unwrap();

                let input = &inputs[input_index];
                input_index += 1;

                let value = input.trim().parse()
                    .expect(&format!("Not a number: {}", input));

                mem[dest] = value;

                pc += 2;
            }
            // output
            4 => {
                let value = get_parameter_value(1, pc, mem, &modes);
                outputs.push(format!("{}", value));

                pc += 2;
            }
            // jump-if-true, jump-if-false
            5 | 6 => {
                let (value, dest) = get_jump_op_params(pc, mem, &modes);

                let condition = match opcode {
                    5 => value != 0,
                    6 => value == 0,
                    // other opcodes are filtered by the outer match, so this is fine
                    _ => unreachable!(),
                };

                if condition {
                    pc = dest;
                } else {
                    pc += 3;
                }
            }
            // less than, equals
            7 | 8 => {
                let (a, b, dest) = get_ternary_op_params(pc, mem, &modes);

                let condition = match opcode {
                    7 => a < b,
                    8 => a == b,
                    // other opcodes are filtered by the outer match, so this is fine
                    _ => unreachable!(),
                };

                mem[dest] = if condition { 1 } else { 0 };

                pc += 4;
            }
            // halt
            99 => break,
            _ => {
                dump_memory(pc, mem);
                panic!("Unknown opcode: {}", opcode);
            }
        }
    }

    outputs
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parameter_modes() {
        let mut mem = [1002,4,3,4,33];
        let _ = interpret(&mut mem, &[]);
        assert_eq!(
            mem,
            [1002,4,3,4,99]
        );
    }

    #[test]
    fn test_io() {
        let mem = [3,5,4,5,99,0];

        let output = interpret(&mut mem.clone(), &["700".into()]);
        assert_eq!(output, ["700"]);

        let mem = [3,5,104,5,99,0];

        let output = interpret(&mut mem.clone(), &["77".into()]);
        assert_eq!(output, ["5"]);
    }

    #[test]
    fn test_jumps_position_mode() {
        let mem = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        let output = interpret(&mut mem.clone(), &["0".into()]);
        assert_eq!(output, ["0"]);

        let output = interpret(&mut mem.clone(), &["1".into()]);
        assert_eq!(output, ["1"]);
    }

    #[test]
    fn test_jumps_immediate_mode() {
        let mem = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

        let output = interpret(&mut mem.clone(), &["0".into()]);
        assert_eq!(output, ["0"]);

        let output = interpret(&mut mem.clone(), &["1".into()]);
        assert_eq!(output, ["1"]);
    }

    #[test]
    fn test_equal_position_mode() {
        let mem = [3,9,8,9,10,9,4,9,99,-1,8];
        
        let output = interpret(&mut mem.clone(), &["8".into()]);
        assert_eq!(output, ["1"]);

        let output = interpret(&mut mem.clone(), &["4".into()]);
        assert_eq!(output, ["0"]);

    }
    
    #[test]
    fn test_lessthan_position_mode() {
        let mem = [3,9,7,9,10,9,4,9,99,-1,8];

        let output = interpret(&mut mem.clone(), &["7".into()]);
        assert_eq!(output, ["1"]);

        let output = interpret(&mut mem.clone(), &["9".into()]);
        assert_eq!(output, ["0"]);
    }

    #[test]
    fn test_equal_immediate_mode() {
        let mem = [3,3,1108,-1,8,3,4,3,99];

        let output = interpret(&mut mem.clone(), &["8".into()]);
        assert_eq!(output, ["1"]);

        let output = interpret(&mut mem.clone(), &["4".into()]);
        assert_eq!(output, ["0"]);
    }

    #[test]
    fn test_lessthan_immediate_mode() {
        let mem = [3,3,1107,-1,8,3,4,3,99];

        let output = interpret(&mut mem.clone(), &["7".into()]);
        assert_eq!(output, ["1"]);

        let output = interpret(&mut mem.clone(), &["9".into()]);
        assert_eq!(output, ["0"]);
    }

    #[test]
    fn test_io_comparison_jumps() {
        let mem = [
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        ];

        let output = interpret(&mut mem.clone(), &["7".into()]);
        assert_eq!(output, ["999"]);

        let output = interpret(&mut mem.clone(), &["8".into()]);
        assert_eq!(output, ["1000"]);

        let output = interpret(&mut mem.clone(), &["9".into()]);
        assert_eq!(output, ["1001"]);
    }
}