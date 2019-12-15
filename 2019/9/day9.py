import intcode

def main():
    with open("input.txt") as f:
        content = f.read()
    program = list(map(lambda x: int(x), content.split(",")))

    computer = intcode.IntCode(intcode.InputProvider(), intcode.OutputSink())
    computer.run_to_completion(program)

if __name__ == "__main__":
    main()