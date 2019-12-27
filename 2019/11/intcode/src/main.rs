use intcode;

struct ConsoleInputProvider { }

impl intcode::InputProvider for ConsoleInputProvider {
    fn get_input(&self) -> i64 {
        0
    }
}

struct ConsoleOutputSink { }

impl intcode::OutputSink for ConsoleOutputSink {
    fn print_output(&mut self, value: i64) {
        println!("{}", value);
    }
}

fn main() {
    let input_provider = ConsoleInputProvider { };
    let mut output_sink = ConsoleOutputSink {  };
    let mut computer = intcode::IntCode::new(&input_provider, &mut output_sink);
    let mut input = vec![1, 0, 0, 0, 99];
    computer.run_to_completion(&mut input);

    println!("{:?}", input);
}