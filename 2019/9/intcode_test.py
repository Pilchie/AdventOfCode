import unittest
import intcode

class TestInputProvider(intcode.InputProvider):
    def __init__(self, value):
        self.value = value

    def get_input(self):
        return self.value

class TestOutputSink(intcode.OutputSink):
    def __init__(self):
        self.output = []

    def print_output(self, output):
        self.output.append(output)

class IntCodeTests(unittest.TestCase):
    def verify(self, input, expected):
        computer = intcode.IntCode(intcode.InputProvider, TestOutputSink)
        computer.run_to_completion(input)
        self.assertSequenceEqual(expected, input)

    def test_1(self):
        self.verify([1, 0, 0, 0, 99], [2, 0, 0, 0, 99])

    def test_2(self):
        self.verify([2, 3, 0, 3, 99], [2, 3, 0, 6, 99])

    def test_3(self):
        self.verify([2, 4, 4, 5, 99, 0], [2, 4, 4, 5, 99, 9801])

    def test_4(self):
        self.verify([1, 1, 1, 4, 99, 5, 6, 0, 99], [30, 1, 1, 4, 2, 5, 6, 0, 99])

    def test_5(self):
        self.verify([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])

    def test_6(self):
        self.verify([1002, 4, 3, 4, 33], [1002, 4, 3, 4, 99])

    def test_7(self):
        self.verify([1101,100,-1,4,0], [1101, 100, -1, 4, 99])

class IntCodeInputOutputTests(unittest.TestCase):
    def verifyInputOutput(self, program, input, expected_output):
        output_sink = TestOutputSink()
        computer = intcode.IntCode(TestInputProvider(input), output_sink)
        computer.run_to_completion(program)
        self.assertEqual(expected_output, output_sink.output[0])

    def test_Equals8PositionYes(self):
        self.verifyInputOutput([3,9,8,9,10,9,4,9,99,-1,8], 8, 1)

    def test_Equals8PositionNo(self):
        self.verifyInputOutput([3,9,8,9,10,9,4,9,99,-1,8], 7, 0)

    def test_LessThan8PositionYes(self):
        self.verifyInputOutput([3,9,7,9,10,9,4,9,99,-1,8], 7, 1)

    def test_LessThan8PositionNo(self):
        self.verifyInputOutput([3,9,7,9,10,9,4,9,99,-1,8], 8, 0)

    def test_Equals8ImmediateYes(self):
        self.verifyInputOutput([3,3,1108,-1,8,3,4,3,99], 8, 1)

    def test_Equals8ImmediateNo(self):
        self.verifyInputOutput([3,3,1108,-1,8,3,4,3,99], 7, 0)

    def test_LessThan8ImmediateYes(self):
        self.verifyInputOutput([3,3,1107,-1,8,3,4,3,99], 7, 1)

    def test_LessThan8ImmediateNo(self):
        self.verifyInputOutput([3,3,1107,-1,8,3,4,3,99], 8, 0)

    def test_JumpWithZeroInputPosition(self):
        self.verifyInputOutput([3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0, 0)

    def test_JumpWithNonZeroInputPosition(self):
        self.verifyInputOutput([3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 25, 1)

    def test_JumpWithZeroInputImmediate(self):
        self.verifyInputOutput([3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0, 0)

    def test_JumpWithNonZeroInputImmediate(self):
        self.verifyInputOutput([3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 25, 1)

    def test_LargerExampleInputBelowEight(self):
        self.verifyInputOutput([3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 3, 999)

    def test_LargerExampleInputEqualToEight(self):
        self.verifyInputOutput([3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8, 1000)

    def test_LargerExampleInputOverEight(self):
        self.verifyInputOutput([3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 29, 1001)

    def test_8(self):
        self.verifyInputOutput([109, -1, 4, 1, 99], 0, -1)

    def test_9(self):
        self.verifyInputOutput([109, -1, 104, 1, 99], 0, 1)

    def test_10(self):
        self.verifyInputOutput([109, -1, 204, 1, 99], 0, 109)

    def test_11(self):
        self.verifyInputOutput([109, 1, 9, 2, 204, -6, 99], 0, 204)

    def test_12(self):
        self.verifyInputOutput([109, 1, 109, 9, 204, -6, 99], 0, 204)

    def test_13(self):
        self.verifyInputOutput([109, 1, 209, -1, 204, -106, 99], 0, 204)

    def test_14(self):
        for i in range(0, 25):
            self.verifyInputOutput([109, 1, 3, 3, 204, 2, 99], i, i)

    def test_15(self):
        for i in range(0, 25):
            self.verifyInputOutput([109, 1, 203, 2, 204, 2, 99], i, i)

class IntCodeDay9Tests(unittest.TestCase):
    def __init__(self, methodName):
        super().__init__(methodName)

    def test_1(self):
        input = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
        output_sink = TestOutputSink()
        computer = intcode.IntCode(TestInputProvider(0), output_sink)
        computer.run_to_completion(input)
        self.assertSequenceEqual(input, output_sink.output)

    def test_2(self):
        input = [1102,34915192,34915192,7,4,7,99,0]
        output_sink = TestOutputSink()
        computer = intcode.IntCode(TestInputProvider(0), output_sink)
        computer.run_to_completion(input)
        strOutput = str(output_sink.output[0])
        self.assertEqual(16, len(strOutput))

    def test_3(self):
        input = [104,1125899906842624,99]
        output_sink = TestOutputSink()
        computer = intcode.IntCode(TestInputProvider(0), output_sink)
        computer.run_to_completion(input)
        self.assertEqual(input[1], output_sink.output[0])

def main():
    unittest.main()

if __name__ == "__main__":
    main()