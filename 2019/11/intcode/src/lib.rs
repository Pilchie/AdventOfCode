struct IntCode<'a> {
    input_provider: TestInputProvider,
    output_sink: &'a mut TestOutputSink,
}

impl<'a> IntCode<'a> {
    fn run_to_completion(&self, input: &mut Vec<i64>) {

    }
}

struct TestOutputSink {
    output: Vec<i64>
}

struct TestInputProvider {
    value: i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify(input: &mut Vec<i64>, expected: Vec<i64>) {
        let computer = IntCode {
            input_provider: TestInputProvider { value: 0 },
            output_sink: &mut TestOutputSink { output: Vec::new() },
        };
        computer.run_to_completion(input);
        if input.len() != expected.len() {
            panic!("Input and output lengths didn't match, expected: {}, actual {}", expected.len(), input.len());
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
        verify(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_5(){
        verify(&mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_6() {
        verify(&mut vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_7() {
        verify(&mut vec![1101,100,-1,4,0], vec![1101, 100, -1, 4, 99]);
    }

    fn verify_input_output(program: &mut Vec<i64>, input: i64, expected_output: i64) {
        let mut output_sink = TestOutputSink { output: Vec::new() };
        let computer = IntCode {
            input_provider: TestInputProvider { value: input },
            output_sink: &mut output_sink,
        };
        computer.run_to_completion(program);
        assert_eq!(expected_output, output_sink.output[0])
    }

    #[test]
    fn test_equals_8_position_yes() {
        verify_input_output(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 8, 1);
    }

    #[test]
    fn test_equals_8_position_no() {
        verify_input_output(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 7, 0);
    }

    #[test]
    fn test_less_than_8_position_yes() {
        verify_input_output(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 7, 1);
    }

    #[test]
    fn test_less_than_8_position_no() {
        verify_input_output(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 8, 0);
    }

    #[test]
    fn test_equals_8_immediate_yes() {
        verify_input_output(&mut vec![3,3,1108,-1,8,3,4,3,99], 8, 1);
    }

    #[test]
    fn test_equals_8_immediate_no() {
        verify_input_output(&mut vec![3,3,1108,-1,8,3,4,3,99], 7, 0);
    }

    #[test]
    fn test_less_than_8_immediate_yes() {
        verify_input_output(&mut vec![3,3,1107,-1,8,3,4,3,99], 7, 1);
    }

    #[test]
    fn test_less_than_8_immediate_no() {
        verify_input_output(&mut vec![3,3,1107,-1,8,3,4,3,99], 8, 0);
    }

    #[test]
    fn test_jump_with_zero_input_position() {
        verify_input_output(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0, 0);
    }

    #[test]
    fn test_jump_with_nonzero_input_position() {
        verify_input_output(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 25, 1);
    }

    #[test]
    fn test_jump_with_zero_input_immediate() {
        verify_input_output(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0, 0);
    }

    #[test]
    fn test_jump_with_nonzero_input_immediate() {
        verify_input_output(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 25, 1);
    }

    #[test]
    fn test_larger_example_input_below_eight() {
        verify_input_output(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 3, 999);
    }

    #[test]
    fn test_larger_example_input_equal_to_eight() {
        verify_input_output(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8, 1000);
    }

    #[test]
    fn test_larger_example_input_over_eight() {
        verify_input_output(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 29, 1001);
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
        let mut input = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut output_sink = TestOutputSink { output: Vec::new() };
        let computer = IntCode {
            input_provider: TestInputProvider { value: 0 }, 
            output_sink: &mut output_sink,
        };
        computer.run_to_completion(&mut input);
        for (i, &item) in input.iter().enumerate() {
            assert_eq!(item, output_sink.output[i])
        }
    }

    #[test]
    fn test_day9_2() {
        let mut input = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut output_sink = TestOutputSink { output: Vec::new() };
        let computer = IntCode {
            input_provider: TestInputProvider { value: 0 },
            output_sink: &mut output_sink,
        };
        computer.run_to_completion(&mut input);
        let str_output = format!("{}", output_sink.output[0]);
        assert_eq!(16, str_output.len());
    }

    #[test]
    fn test_day9_3() {
        let mut input = vec![104,1125899906842624,99];
        let mut output_sink = TestOutputSink { output: Vec::new() };
        let computer = IntCode {
            input_provider: TestInputProvider { value: 0 },
            output_sink: &mut output_sink,
        };
        computer.run_to_completion(&mut input);
        assert_eq!(input[1], output_sink.output[0]);
    }
}
