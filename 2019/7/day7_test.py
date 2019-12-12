import intcode
import unittest

class AmpInputProvider(intcode.InputProvider):
    def __init__(self, input, setting):
        self.input = input
        self.setting = setting
        self._first = True

    def get_input(self):
        if self._first:
            self._first = False
            return self.setting
        else:
            return self.input

class AmpOutputSink(intcode.OutputSink):
    def print_output(self, output):
        self.output = output

class Day7():
    def __init__(self, program):
        self.program = program

    def part1(self):
        maximum = 0
        maximum_settings = []
        for a1 in range(0, 5):
            for a2 in range(0, 5):
                for a3 in range(0, 5):
                    for a4 in range(0, 5):
                        for a5 in range(0, 5):
                            settings = [a1, a2, a3, a4, a5]
                            if self.is_valid(settings):
                                current = self.try_settings(settings)
                                print(f"Tried '{settings}', got '{current}'")
                                if current > maximum:
                                    maximum = current
                                    maximum_settings = settings
        print(f"Found max '{maximum}' at '{maximum_settings}'")
        return maximum, maximum_settings

    def try_settings(self, settings):
        input = 0
        for setting in settings:
            inputProvider = AmpInputProvider(input, setting)
            outputSink = AmpOutputSink()
            computer = intcode.IntCode(inputProvider, outputSink)
            p = list(self.program)
            computer.run_to_completion(p)
            print(f"inputs: ({input}, {setting}), output: {outputSink.output}")
            input = outputSink.output
        return input

    def is_valid(self, settings):
        for s in range(0, 5):
            i = settings.index(settings[s])
            if i != s:
                return False

            try:
                settings.index(settings[s], s + 1)
            except ValueError as identifier:
                # Expected, we didn't find it
                pass
            else:
                return False

        return True

class Day7Part1Tests(unittest.TestCase):
    def verify(self, expected_output, expected_sequence, program):
        output, sequence = Day7(program).part1()
        self.assertEqual(expected_output, output)
        self.assertSequenceEqual(expected_sequence, sequence)

    def test_1(self):
        self.verify(43210, [4,3,2,1,0], [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0])

    def test_2(self):
        self.verify(54321, [0,1,2,3,4], [3,23,3,24,1002,24,10,24,1002,23,-1,23, 101,5,23,23,1,24,23,23,4,23,99,0,0])

    def test_3(self):
        self.verify(65210, [1,0,4,3,2], [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33, 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0])



def main():
    #unittest.main()
    with open("input.txt") as f:
        content = f.read()
    program = list(map(int, content.split(",")))
    print(program)
    output, sequence = Day7(program).part1()
    print(f"Day 7, Part 1 result: {output}")

if __name__ == "__main__":
    main()
