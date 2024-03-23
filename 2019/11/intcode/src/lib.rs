use std::collections::HashMap;
use std::convert::TryInto;

enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

impl ParameterMode {
    fn get(parameter_modes: i64, position: u32) -> ParameterMode {
        let pos = position - 1;
        let mask = i64::pow(10, pos);
        let modes = parameter_modes / mask;
        let modes = modes % 10;
        if modes == 0 {
            return ParameterMode::Position;
        } else if modes == 1 {
            return ParameterMode::Immediate;
        } else if modes == 2 {
            return ParameterMode::Relative;
        } else {
            panic!("Unexpected parameter mode");
        }
    }
}

pub struct IntCode<'a> {
    input_output_system: &'a mut dyn InputOutputSystem,
    relative_base: i64,
    program_counter: i64,
}

impl<'a> IntCode<'a> {
    pub fn new(input_output_system: &'a mut dyn InputOutputSystem) -> IntCode<'a> {
        IntCode {
            input_output_system,
            relative_base: 0,
            program_counter: 0,
        }
    }

    pub fn run_to_completion(&mut self, input: &mut Vec<i64>) {
        let mut memory = Memory::new(input);
        loop {
            self.program_counter = self.execute_next_instruction(&mut memory);
            if self.program_counter < 0 {
                return;
            }
        }
    }

    fn execute_next_instruction(&mut self, memory: &mut Memory) -> i64 {
        let instruction = self.parse_instruction(memory);
        instruction.execute(memory, self)
    }

    fn parse_instruction(&self, memory: &mut Memory) -> Instruction {
        let opcode_and_parameter_modes = memory.get_at(self.program_counter);
        let opcode = opcode_and_parameter_modes % 100;
        let parameter_modes = opcode_and_parameter_modes / 100;
        match opcode {
            99 => Instruction::Stop,
            1 => Instruction::Add(BinaryOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            2 => Instruction::Multiply(BinaryOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            3 => Instruction::Input(UnaryOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            4 => Instruction::Output(UnaryOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            5 => Instruction::JumpIfTrue(JumpOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            6 => Instruction::JumpIfFalse(JumpOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            7 => Instruction::LessThan(BinaryOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            8 => Instruction::EqualTo(BinaryOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            9 => Instruction::AdjustRelativeBase(UnaryOperator::load(
                memory,
                self.program_counter,
                parameter_modes,
            )),
            _ => panic!("Unexpected instruction"),
        }
    }
}

struct Parameter {
    value: i64,
    mode: ParameterMode,
}

impl Parameter {
    fn load(&self, memory: &mut Memory, relative_base: i64) -> i64 {
        match self.mode {
            ParameterMode::Immediate => self.value,
            ParameterMode::Position => memory.get_at(self.value),
            ParameterMode::Relative => memory.get_at(relative_base + self.value),
        }
    }

    fn store(&self, memory: &mut Memory, value: i64, relative_base: i64) {
        match self.mode {
            ParameterMode::Position => memory.set_at(self.value, value),
            ParameterMode::Relative => memory.set_at(relative_base + self.value, value),
            ParameterMode::Immediate => panic!("Can't store to immediate mode parameter"),
        }
    }
}

struct UnaryOperator {
    position: Parameter,
}

impl UnaryOperator {
    fn load(memory: &mut Memory, program_counter: i64, parameter_modes: i64) -> UnaryOperator {
        UnaryOperator {
            position: Parameter {
                value: memory.get_at(program_counter + 1),
                mode: ParameterMode::get(parameter_modes, 1),
            },
        }
    }
}

struct BinaryOperator {
    param1: Parameter,
    param2: Parameter,
    output: Parameter,
}

impl BinaryOperator {
    fn load(memory: &mut Memory, program_counter: i64, parameter_modes: i64) -> BinaryOperator {
        BinaryOperator {
            param1: Parameter {
                value: memory.get_at(program_counter + 1),
                mode: ParameterMode::get(parameter_modes, 1),
            },
            param2: Parameter {
                value: memory.get_at(program_counter + 2),
                mode: ParameterMode::get(parameter_modes, 2),
            },
            output: Parameter {
                value: memory.get_at(program_counter + 3),
                mode: ParameterMode::get(parameter_modes, 3),
            },
        }
    }

    fn execute<T>(&self, computer: &IntCode, memory: &mut Memory, operation: T) -> i64
    where
        T: Fn(i64, i64) -> i64,
    {
        let input1 = self.param1.load(memory, computer.relative_base);
        let input2 = self.param2.load(memory, computer.relative_base);
        let result = operation(input1, input2);
        self.output.store(memory, result, computer.relative_base);
        computer.program_counter + 4
    }
}

struct JumpOperator {
    value: Parameter,
    dest: Parameter,
}

impl JumpOperator {
    fn load(memory: &mut Memory, program_counter: i64, parameter_modes: i64) -> JumpOperator {
        JumpOperator {
            value: Parameter {
                value: memory.get_at(program_counter + 1),
                mode: ParameterMode::get(parameter_modes, 1),
            },
            dest: Parameter {
                value: memory.get_at(program_counter + 2),
                mode: ParameterMode::get(parameter_modes, 2),
            },
        }
    }

    fn execute<T>(&self, computer: &IntCode, memory: &mut Memory, compare: T) -> i64
    where
        T: Fn(i64) -> bool,
    {
        let value = self.value.load(memory, computer.relative_base);
        let dest = self.dest.load(memory, computer.relative_base);
        if compare(value) {
            dest
        } else {
            computer.program_counter + 3
        }
    }
}

enum Instruction {
    Stop,
    Add(BinaryOperator),
    Multiply(BinaryOperator),
    Input(UnaryOperator),
    Output(UnaryOperator),
    JumpIfTrue(JumpOperator),
    JumpIfFalse(JumpOperator),
    LessThan(BinaryOperator),
    EqualTo(BinaryOperator),
    AdjustRelativeBase(UnaryOperator),
}

impl Instruction {
    fn execute(&self, memory: &mut Memory, computer: &mut IntCode) -> i64 {
        match self {
            Instruction::Stop => -1,
            Instruction::Add(binary_operator) => {
                binary_operator.execute(computer, memory, |a, b| a + b)
            }
            Instruction::Multiply(binary_operator) => {
                binary_operator.execute(computer, memory, |a, b| a * b)
            }
            Instruction::Input(unary_operator) => {
                let value = computer.input_output_system.get_input();
                unary_operator
                    .position
                    .store(memory, value, computer.relative_base);
                computer.program_counter + 2
            }
            Instruction::Output(unary_operator) => {
                let output = unary_operator.position.load(memory, computer.relative_base);
                computer.input_output_system.print_output(output);
                computer.program_counter + 2
            }
            Instruction::JumpIfTrue(jump_operator) => {
                jump_operator.execute(computer, memory, |a| a != 0)
            }
            Instruction::JumpIfFalse(jump_operator) => {
                jump_operator.execute(computer, memory, |a| a == 0)
            }
            Instruction::LessThan(binary_operator) => {
                binary_operator.execute(computer, memory, |a, b| if a < b { 1 } else { 0 })
            }
            Instruction::EqualTo(binary_operator) => {
                binary_operator.execute(computer, memory, |a, b| if a == b { 1 } else { 0 })
            }
            Instruction::AdjustRelativeBase(unary_operator) => {
                let value = unary_operator.position.load(memory, computer.relative_base);
                computer.relative_base += value;
                computer.program_counter + 2
            }
        }
    }
}

struct Memory<'a> {
    memory: &'a mut Vec<i64>,
    other_values: HashMap<usize, i64>,
}

impl<'a> Memory<'a> {
    fn new(memory: &'a mut Vec<i64>) -> Memory {
        Memory {
            memory,
            other_values: HashMap::new(),
        }
    }
    fn get_at(&mut self, address: i64) -> i64 {
        let addr = address.try_into().unwrap();
        if addr < self.memory.len() {
            return self.memory[addr];
        } else {
            let value = self.other_values.entry(addr).or_insert(0);
            return *value;
        }
    }

    fn set_at(&mut self, address: i64, value: i64) {
        let addr = address.try_into().unwrap();
        if addr < self.memory.len() {
            self.memory[addr] = value;
        } else {
            self.other_values.insert(addr, value);
        }
    }
}

pub trait InputOutputSystem {
    fn print_output(&mut self, value: i64);
    fn get_input(&mut self) -> i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestInputOutputSystem {
        input_value: i64,
        output: Vec<i64>,
    }

    impl InputOutputSystem for TestInputOutputSystem {
        fn get_input(&mut self) -> i64 {
            self.input_value
        }

        fn print_output(&mut self, value: i64) {
            self.output.push(value);
        }
    }

    fn verify(input: &mut Vec<i64>, expected: Vec<i64>) {
        let mut input_output_system = TestInputOutputSystem {
            input_value: 0,
            output: Vec::new(),
        };
        let mut computer = IntCode::new(&mut input_output_system);
        computer.run_to_completion(input);
        if input.len() != expected.len() {
            panic!(
                "Input and output lengths didn't match, expected: {}, actual {}",
                expected.len(),
                input.len()
            );
        }
        for (i, &item) in input.iter().enumerate() {
            assert_eq!(expected[i], item);
        }
    }

    #[test]
    fn test_1() {
        verify(&mut vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_2() {
        verify(&mut vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_3() {
        verify(&mut vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_4() {
        verify(
            &mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn test_5() {
        verify(
            &mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn test_6() {
        verify(&mut vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_7() {
        verify(&mut vec![1101, 100, -1, 4, 0], vec![1101, 100, -1, 4, 99]);
    }

    fn verify_input_output(program: &mut Vec<i64>, input: i64, expected_output: i64) {
        let mut input_output_system = TestInputOutputSystem {
            input_value: input,
            output: Vec::new(),
        };
        let mut computer = IntCode::new(&mut input_output_system);
        computer.run_to_completion(program);
        assert_eq!(expected_output, input_output_system.output[0])
    }

    #[test]
    fn test_equals_8_position_yes() {
        verify_input_output(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8, 1);
    }

    #[test]
    fn test_equals_8_position_no() {
        verify_input_output(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7, 0);
    }

    #[test]
    fn test_less_than_8_position_yes() {
        verify_input_output(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7, 1);
    }

    #[test]
    fn test_less_than_8_position_no() {
        verify_input_output(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8, 0);
    }

    #[test]
    fn test_equals_8_immediate_yes() {
        verify_input_output(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, 1);
    }

    #[test]
    fn test_equals_8_immediate_no() {
        verify_input_output(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 7, 0);
    }

    #[test]
    fn test_less_than_8_immediate_yes() {
        verify_input_output(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7, 1);
    }

    #[test]
    fn test_less_than_8_immediate_no() {
        verify_input_output(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8, 0);
    }

    #[test]
    fn test_jump_with_zero_input_position() {
        verify_input_output(
            &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            0,
            0,
        );
    }

    #[test]
    fn test_jump_with_nonzero_input_position() {
        verify_input_output(
            &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            25,
            1,
        );
    }

    #[test]
    fn test_jump_with_zero_input_immediate() {
        verify_input_output(
            &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            0,
            0,
        );
    }

    #[test]
    fn test_jump_with_nonzero_input_immediate() {
        verify_input_output(
            &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            25,
            1,
        );
    }

    #[test]
    fn test_larger_example_input_below_eight() {
        verify_input_output(
            &mut vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            3,
            999,
        );
    }

    #[test]
    fn test_larger_example_input_equal_to_eight() {
        verify_input_output(
            &mut vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            8,
            1000,
        );
    }

    #[test]
    fn test_larger_example_input_over_eight() {
        verify_input_output(
            &mut vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            29,
            1001,
        );
    }

    #[test]
    fn test_8() {
        verify_input_output(&mut vec![109, -1, 4, 1, 99], 0, -1);
    }

    #[test]
    fn test_9() {
        verify_input_output(&mut vec![109, -1, 104, 1, 99], 0, 1);
    }

    #[test]
    fn test_10() {
        verify_input_output(&mut vec![109, -1, 204, 1, 99], 0, 109);
    }

    #[test]
    fn test_11() {
        verify_input_output(&mut vec![109, 1, 9, 2, 204, -6, 99], 0, 204);
    }

    #[test]
    fn test_12() {
        verify_input_output(&mut vec![109, 1, 109, 9, 204, -6, 99], 0, 204);
    }

    #[test]
    fn test_13() {
        verify_input_output(&mut vec![109, 1, 209, -1, 204, -106, 99], 0, 204);
    }

    #[test]
    fn test_14() {
        for i in 0..25 {
            verify_input_output(&mut vec![109, 1, 3, 3, 204, 2, 99], i, i);
        }
    }

    #[test]
    fn test_15() {
        for i in 0..25 {
            verify_input_output(&mut vec![109, 1, 203, 2, 204, 2, 99], i, i);
        }
    }

    #[test]
    fn test_day9_1() {
        let mut input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut input_output_system = TestInputOutputSystem {
            input_value: 0,
            output: Vec::new(),
        };
        let mut computer = IntCode::new(&mut input_output_system);
        computer.run_to_completion(&mut input);
        for (i, &item) in input.iter().enumerate() {
            assert_eq!(item, input_output_system.output[i])
        }
    }

    #[test]
    fn test_day9_2() {
        let mut input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut input_output_system = TestInputOutputSystem {
            input_value: 0,
            output: Vec::new(),
        };
        let mut computer = IntCode::new(&mut input_output_system);
        computer.run_to_completion(&mut input);
        let str_output = format!("{}", input_output_system.output[0]);
        assert_eq!(16, str_output.len());
    }

    #[test]
    fn test_day9_3() {
        let mut input = vec![104, 1125899906842624, 99];
        let mut input_output_system = TestInputOutputSystem {
            input_value: 0,
            output: Vec::new(),
        };
        let mut computer = IntCode::new(&mut input_output_system);
        computer.run_to_completion(&mut input);
        assert_eq!(input[1], input_output_system.output[0]);
    }
}
