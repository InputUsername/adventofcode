//! This crate provides a fully-working, fully-compliant
//! Intcode interpreter for Advent of Code 2019.

use std::convert::{From, TryInto};
use std::path::Path;
use std::fs;

const MAX_PARAMETERS: usize = 3;

#[repr(u8)]
#[derive(Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn new(mode: i64) -> Self {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            n => panic!("Unknown parameter mode: {}", n),
        }
    }
}

type ParameterModes = [ParameterMode; MAX_PARAMETERS];

type Instruction = (i64, ParameterModes);

/// Represents the result of running a single instruction.
pub enum InterpretStep {
    /// The executed instruction caused input to be read.
    Input,
    /// The executed instruction caused output.
    Output(i64),
    /// The executed instruction caused the program to halt.
    Halt,
    /// The executed instruction did not perform input or output and did not halt the program.
    Nothing,
}

/// Represents an Intcode computer containing memory, a program counter
/// and a relative base.
#[derive(Clone)]
pub struct Computer {
    pub memory: Vec<i64>,
    pc: usize,
    relative_base: isize,
}

impl From<&[i64]> for Computer {
    fn from(slice: &[i64]) -> Self {
        Self {
            memory: slice.to_vec(),
            pc: 0,
            relative_base: 0,
        }
    }
}

impl Computer {
    /// Construct a new empty computer.
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            pc: 0,
            relative_base: 0,
        }
    }

    /// Construct a computer from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let memory = fs::read_to_string(path)
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Self {
            memory,
            pc: 0,
            relative_base: 0,
        }
    }

    /// Print a memory dump to stderr.
    fn dump_memory(&self) {
        const SIZE: usize = 20;
        const PAD: usize = 7;

        eprintln!("{}", "-".repeat(SIZE * PAD));
        eprintln!("MEMORY DUMP");
        eprintln!("{}", "-".repeat(SIZE * PAD));

        for (start_addr, chunk) in self.memory.chunks(SIZE).enumerate().map(|(i, chunk)| (SIZE * i, chunk)) {
            for (i, _) in chunk.iter().enumerate() {
                eprint!("{: >7}", start_addr + i);
            }
            eprintln!();
            for val in chunk {
                eprint!("{: >7}", val);
            }
            eprintln!();
            if self.pc >= start_addr && self.pc < start_addr + SIZE {
                eprint!("{}", " ".repeat((self.pc - start_addr) * PAD + 1));
                eprintln!("{}", "^".repeat(PAD - 1));
            } else {
                eprintln!();
            }
        }

        eprintln!("{}", "-".repeat(SIZE * PAD));
    }

    /// Parse the current instruction at the program counter.
    fn parse_instruction(&self) -> Instruction {
        let instruction = self.memory[self.pc];
        let opcode = instruction % 100;
        let mut modes_mask = instruction / 100;
        let mut modes = [ParameterMode::Position; MAX_PARAMETERS];
        for i in 0..MAX_PARAMETERS {
            modes[i] = ParameterMode::new(modes_mask % 10);
            modes_mask /= 10;
        }
        (opcode, modes)
    }

    /// Read a parameter's value, respecting the parameter modes.
    fn get_parameter_value(&self, index: usize, modes: &ParameterModes) -> i64 {
        let address: usize = match modes[index] {
            ParameterMode::Position => self.memory[self.pc + 1 + index].try_into().unwrap(),
            ParameterMode::Immediate => self.pc + 1 + index,
            ParameterMode::Relative => {
                let offset: isize = self.memory[self.pc + 1 + index].try_into().unwrap();
                (self.relative_base + offset).try_into().unwrap()
            },
        };
        if address >= self.memory.len() {
            return 0;
        }
        self.memory[address]
    }

    /// Write a value to a destination parameter, respecting the parameter modes.
    fn write(&mut self, index: usize, modes: &ParameterModes, value: i64) {
        let address: usize = match modes[index] {
            ParameterMode::Position => self.memory[self.pc + 1 + index].try_into().unwrap(),
            ParameterMode::Immediate => panic!("Destination parameters cannot be in immediate mode"),
            ParameterMode::Relative => {
                let offset: isize = self.memory[self.pc + 1 + index].try_into().unwrap();
                (self.relative_base + offset).try_into().unwrap()
            }
        };
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
        self.memory[address] = value;
    }

    /// Get the parameters for a binary operation, respecting the parameter modes.
    fn get_binary_op_parameters(&self, modes: &ParameterModes) -> (i64, i64) {
        let a = self.get_parameter_value(0, modes);
        let b = self.get_parameter_value(1, modes);

        (a, b)
    }

    /// Get the parameters for a jump operation:
    /// a condition value and a jump address.
    fn get_jump_op_parameters(&self, modes: &ParameterModes) -> (i64, usize) {
        let value = self.get_parameter_value(0, modes);
        let location = self.get_parameter_value(1, modes).try_into().unwrap();

        (value, location)
    }

    /// Interpret one instruction and return its side effect.
    pub fn step(&mut self, input: Option<i64>) -> InterpretStep {
        let (opcode, modes) = self.parse_instruction();

        match opcode {
            // add, multiply
            1 | 2 => {
                let (a, b) = self.get_binary_op_parameters(&modes);

                let value = match opcode {
                    1 => a + b,
                    2 => a * b,
                    // other opcodes are filtered by the outer match, so this is fine
                    _ => unreachable!(),
                };

                self.write(2, &modes, value);

                self.pc += 4;
            }
            // input
            3 => {
                self.write(0, &modes, input.unwrap());
                
                self.pc += 2;

                return InterpretStep::Input;
            }
            // output
            4 => {
                let value = self.get_parameter_value(0, &modes);

                self.pc += 2;

                return InterpretStep::Output(value);
            }
            // jump-if-true, jump-if-false
            5 | 6 => {
                let (value, location) = self.get_jump_op_parameters(&modes);

                let condition = match opcode {
                    5 => value != 0,
                    6 => value == 0,
                    // other opcodes are filtered by the outer match, so this is fine
                    _ => unreachable!(),
                };

                if condition {
                    self.pc = location;
                } else {
                    self.pc += 3;
                }
            }
            // less than, equals
            7 | 8 => {
                let (a, b) = self.get_binary_op_parameters(&modes);

                let comparison = match opcode {
                    7 => a < b,
                    8 => a == b,
                    // other opcodes are filtered by the outer match, so this is fine
                    _ => unreachable!(),
                };

                self.write(2, &modes, if comparison { 1 } else { 0 });

                self.pc += 4;
            }
            // relative base offset
            9 => {
                let value: isize = self.get_parameter_value(0, &modes).try_into().unwrap();

                self.relative_base += value;

                self.pc += 2;
            }
            // halt
            99 => return InterpretStep::Halt,
            _ => {
                self.dump_memory();
                panic!("Unknown opcode: {}", opcode);
            }
        }

        InterpretStep::Nothing
    }

    /// Interpret an Intcode program.
    pub fn run(&mut self, inputs: &[i64]) -> Vec<i64> {
        let mut input_index = 0;
        let mut outputs = Vec::new();

        loop {
            match self.step(inputs.get(input_index).copied()) {
                InterpretStep::Input => input_index += 1,
                InterpretStep::Output(output) => outputs.push(output),
                InterpretStep::Halt => break,
                InterpretStep::Nothing => {}
            }
        }

        outputs
    }

    /// Check if the current instruction is an input instruction.
    pub fn wants_input(&self) -> bool {
        let (opcode, _) = self.parse_instruction();
        opcode == 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parameter_modes() {
        let mem = [1002,4,3,4,33];
        let mut cpu = Computer::from(&mem[..]);

        let _ = cpu.run(&[]);
        assert_eq!(
            cpu.memory,
            [1002,4,3,4,99]
        );
    }

    #[test]
    fn test_io() {
        let mem = [3,5,4,5,99,0];
        let mut cpu = Computer::from(&mem[..]);

        let output = cpu.run(&[700]);
        assert_eq!(output, [700]);

        let mem = [3,5,104,5,99,0];
        let mut cpu = Computer::from(&mem[..]);

        let output = cpu.run(&[77]);
        assert_eq!(output, [5]);
    }

    #[test]
    fn test_jumps_position_mode() {
        let mem = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let cpu = Computer::from(&mem[..]);

        let output = cpu.clone().run(&[0]);
        assert_eq!(output, [0]);

        let output = cpu.clone().run(&[1]);
        assert_eq!(output, [1]);
    }

    #[test]
    fn test_jumps_immediate_mode() {
        let mem = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let cpu = Computer::from(&mem[..]);

        let output = cpu.clone().run(&[0]);
        assert_eq!(output, [0]);

        let output = cpu.clone().run(&[1]);
        assert_eq!(output, [1]);
    }

    #[test]
    fn test_equal_position_mode() {
        let mem = [3,9,8,9,10,9,4,9,99,-1,8];
        let cpu = Computer::from(&mem[..]);
        
        let output = cpu.clone().run(&[8]);
        assert_eq!(output, [1]);

        let output = cpu.clone().run(&[4]);
        assert_eq!(output, [0]);

    }
    
    #[test]
    fn test_lessthan_position_mode() {
        let mem = [3,9,7,9,10,9,4,9,99,-1,8];
        let cpu = Computer::from(&mem[..]);

        let output = cpu.clone().run(&[7]);
        assert_eq!(output, [1]);

        let output = cpu.clone().run(&[9]);
        assert_eq!(output, [0]);
    }

    #[test]
    fn test_equal_immediate_mode() {
        let mem = [3,3,1108,-1,8,3,4,3,99];
        let cpu = Computer::from(&mem[..]);

        let output = cpu.clone().run(&[8]);
        assert_eq!(output, [1]);

        let output = cpu.clone().run(&[4]);
        assert_eq!(output, [0]);
    }

    #[test]
    fn test_lessthan_immediate_mode() {
        let mem = [3,3,1107,-1,8,3,4,3,99];
        let cpu = Computer::from(&mem[..]);

        let output = cpu.clone().run(&[7]);
        assert_eq!(output, [1]);

        let output = cpu.clone().run(&[9]);
        assert_eq!(output, [0]);
    }

    #[test]
    fn test_io_comparison_jumps() {
        let mem = [
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        ];
        let cpu = Computer::from(&mem[..]);

        let output = cpu.clone().run(&[7]);
        assert_eq!(output, [999]);

        let output = cpu.clone().run(&[8]);
        assert_eq!(output, [1000]);

        let output = cpu.clone().run(&[9]);
        assert_eq!(output, [1001]);
    }

    #[test]
    fn test_relative_mode_quine() {
        let mem = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut cpu = Computer::from(&mem[..]);

        let output = cpu.run(&[]);
        assert_eq!(
            output,
            [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
        );
    }

    #[test]
    fn test_large_numbers() {
        let mem = [1102,34915192,34915192,7,4,7,99,0];
        let mut cpu = Computer::from(&mem[..]);

        let _ = cpu.run(&[]);

        let mem = [104,1125899906842624,99];
        let mut cpu = Computer::from(&mem[..]);

        let output = cpu.run(&[]);
        assert_eq!(
            output,
            [1125899906842624]
        );
    }
}