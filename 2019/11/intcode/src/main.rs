use intcode;

fn main() {
    let input_provider = intcode::InputProvider { value: 0 };
    let mut output_sink = intcode::OutputSink { output: Vec::new() };
    let mut computer = intcode::IntCode::new(input_provider, &mut output_sink);
    let mut input = vec![1, 0, 0, 0, 99];
    computer.run_to_completion(&mut input);

    println!("{:?}", input);
}