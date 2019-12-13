import intcode
import threading
import time
import unittest

class AmpConnection(intcode.InputProvider, intcode.OutputSink):
    def __init__(self, name):
        self.queue = []
        self.name = name
        self.lock = threading.Lock()

    def do_in_lock(self, fn):
        try:
            self.lock.acquire()
            return fn()
        finally:
            self.lock.release()

    def get_input(self):
        print(f"{self.name} in get_input")
        while self.do_in_lock(lambda: len(self.queue) == 0):
            time.sleep(1/1000)

        res = self.do_in_lock(lambda: self.queue.pop(0))
        print(f"{self.name} returning: {res}")
        return res

    def print_output(self, output):
        print(f"{self.name} storing: {output}")
        self.do_in_lock(lambda: self.queue.append(output))

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

    def part2(self):
        return self.find_max(5, 10, False)

    def part1(self):
        return self.find_max(0, 5, True)

    def find_max(self, min, max, part1):
        maximum = 0
        maximum_settings = []
        for a1 in range(min, max):
            for a2 in range(min, max):
                for a3 in range(min, max):
                    for a4 in range(min, max):
                        for a5 in range(min, max):
                            settings = [a1, a2, a3, a4, a5]
                            if self.is_valid(settings):
                                current = 0
                                if part1:
                                    current = self.try_settings(settings)
                                else:
                                    current = self.try_settings_part2(settings)
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

    def try_settings_part2(self, settings):
        connections = []
        for i in range(0, 5):
            connection = AmpConnection("Amp" + str(i))
            connection.queue.append(settings[i])
            connections.append(connection)
        connections[0].queue.append(0)

        computers = []
        for i in range(0, 5):
            inputConnection = connections[i]
            if i + 1 < len(connections):
                outputConnection = connections[i + 1]
            else:
                outputConnection = connections[0]

            print(f"Connecting computer '{i}, input: '{inputConnection.name}', output: '{outputConnection.name}'")
            computer = intcode.IntCode(inputConnection, outputConnection)
            computers.append(computer)

        threads = []
        for c in computers:
            threads.append(ComputerThread(c, self.program))

        for t in threads:
            t.start()

        for t in threads:
            t.join()

        return connections[0].queue.pop()

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

class ComputerThread(threading.Thread):
    def __init__(self, computer, memory):
        threading.Thread.__init__(self)
        self.computer = computer
        self.memory = list(memory)

    def run(self):
        self.computer.run_to_completion(self.memory)

class Day7Part2Tests(unittest.TestCase):
    def verify(self, expected_output, expected_sequence, program):
        output, sequence = Day7(program).part2()
        self.assertEqual(expected_output, output)
        self.assertSequenceEqual(expected_sequence, sequence)

    def test_1(self):
        self.verify(139629729, [9,8,7,6,5], [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5])

    def test_2(self):
        self.verify(18216, [9,7,8,5,6], [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10])

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

def solve_part1():
    with open("input.txt") as f:
        content = f.read()
    program = list(map(int, content.split(",")))
    print(program)
    output, sequence = Day7(program).part1()
    print(f"Day 7, Part 1 result: {output}")

def solve_part2():
    with open("input.txt") as f:
        content = f.read()
    program = list(map(int, content.split(",")))
    print(program)
    output, sequence = Day7(program).part2()
    print(f"Day 7, Part 2 result: {output}")

def main():
    #unittest.main()
    #solve_part1()
    solve_part2()

if __name__ == "__main__":
    main()
