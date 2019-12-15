def main():
    computer = IntCode(InputProvider(), OutputSink())
    param = Parameter(42, ParameterMode.immediate)
    print(param)

if __name__ == "__main__":
    main()

class IntCode:
    def __init__(self, input_provider, output_sink):
        self.program_counter = 0
        self.input_provider = input_provider
        self.output_sink = output_sink
        self.relative_base = 0

    def run_to_completion(self, program):
        memory = Memory(program)
        while True:
            self.program_counter = self.execute_next_instruction(memory)
            if self.program_counter < 0:
                return

    def execute_next_instruction(self, memory):
        instruction = self.parse_instruction(memory)
        return instruction.execute(memory)

    def parse_instruction(self, memory):
        opcode_and_parameter_modes = memory.get_at(self.program_counter)
        opcode = opcode_and_parameter_modes % 100
        if opcode == 99:
            return StopInstruction(self)

        parameter_modes = opcode_and_parameter_modes / 100

        if opcode == 1:
            return AddInstruction(self,
                param1=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory.get_at(self.program_counter + 2), self.get_mode(parameter_modes, 2)),
                output=Parameter(memory.get_at(self.program_counter + 3), self.get_mode(parameter_modes, 3)))
        elif opcode == 2:
            return MultiplyInstruction(self,
                param1=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory.get_at(self.program_counter + 2), self.get_mode(parameter_modes, 2)),
                output=Parameter(memory.get_at(self.program_counter + 3), self.get_mode(parameter_modes, 3)))
        elif opcode == 3:
            return InputInstruction(self,
                position=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)))
        elif opcode == 4:
            return OutputInstruction(self,
                position=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)))
        elif opcode == 5:
            return JumpIfTrueInstruction(self,
                value=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)),
                dest=Parameter(memory.get_at(self.program_counter + 2), self.get_mode(parameter_modes, 2)))
        elif opcode == 6:
            return JumpIfFalseInstruction(self,
                value=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)),
                dest=Parameter(memory.get_at(self.program_counter + 2), self.get_mode(parameter_modes, 2)))
        elif opcode == 7:
            return LessThanInstruction(self,
                param1=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory.get_at(self.program_counter + 2), self.get_mode(parameter_modes, 2)),
                output=Parameter(memory.get_at(self.program_counter + 3), self.get_mode(parameter_modes, 3)))
        elif opcode == 8:
            return EqualsInstruction(self,
                param1=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory.get_at(self.program_counter + 2), self.get_mode(parameter_modes, 2)),
                output=Parameter(memory.get_at(self.program_counter + 3), self.get_mode(parameter_modes, 3)))
        elif opcode == 9:
            return AdjustRelativeBaseInstruction(self,
                param=Parameter(memory.get_at(self.program_counter + 1), self.get_mode(parameter_modes, 1)))
        else:
            raise Invalid()

    def get_mode(self, parameter_modes, position):
        position = position - 1
        mask = 10 ** position
        parameter_modes = parameter_modes // mask
        parameter_modes = parameter_modes % 10
        if parameter_modes == 0:
            return ParameterMode.position
        elif parameter_modes == 1:
            return ParameterMode.immediate
        elif parameter_modes == 2:
            return ParameterMode.relative
        else:
            raise Invalid

class ParameterMode:
    immediate = "Immediate"
    position = "Position"
    relative = "relative"

class Invalid(Exception):
    pass

class InputProvider:
    def get_input(self):
        return int(input())

class OutputSink:
    def print_output(self, output):
        print(output)

class Parameter:
    def __init__(self, value, mode):
        self.value = value
        self.mode = mode

    def load(self, memory, relative_base):
        if self.mode == ParameterMode.immediate:
            return self.value
        elif self.mode == ParameterMode.position:
            return memory.get_at(self.value)
        elif self.mode == ParameterMode.relative:
            return memory.get_at(relative_base + self.value)

        raise Invalid()

    def store(self, memory, value, relative_base):
        if self.mode == ParameterMode.position:
            memory.set_at(self.value, value)
        elif self.mode == ParameterMode.relative:
            memory.set_at(relative_base + self.value, value)
        else:
            raise Invalid()

class Memory:
    def __init__(self, program):
        self.program = program
        self.other_values = { }

    def get_at(self, address):
        if address < len(self.program):
            return self.program[address]
        else:
            value = self.other_values.get(address, 0)
            return value

    def set_at(self, address, value):
        if address < len(self.program):
            self.program[address] = value
        else:
            self.other_values[address] = value

class Instruction:
    def __init__(self, computer):
        self.computer = computer

    def execute(self, memory):
        raise NotImplemented

class StopInstruction(Instruction):
    def __init__(self, computer):
        super().__init__(computer)

    def execute(self, memory):
        return -1

class ArithmeticInstruction(Instruction):
    def __init__(self, computer, param1, param2, output):
        super().__init__(computer)
        self.param1 = param1
        self.param2 = param2
        self.output = output

    def execute(self, memory):
        input1 = self.param1.load(memory, self.computer.relative_base)
        input2 = self.param2.load(memory, self.computer.relative_base)
        result = self.operation(input1, input2)
        self.output.store(memory, result, self.computer.relative_base)
        return self.computer.program_counter + 4

    def operation(self, input1, input2):
        raise NotImplementedError

class AddInstruction(ArithmeticInstruction):
    def __init__(self, computer, param1, param2, output):
        super().__init__(computer, param1, param2, output)

    def operation(self, input1, input2):
        return input1 + input2

class MultiplyInstruction(ArithmeticInstruction):
    def __init__(self, computer, param1, param2, output):
        super().__init__(computer, param1, param2, output)

    def operation(self, input1, input2):
        return input1 * input2

class IOInstruction(Instruction):
    def __init__(self, computer, position):
        super().__init__(computer)
        self.position = position

class InputInstruction(IOInstruction):
    def __init__(self, computer, position):
        super().__init__(computer, position)

    def execute(self, memory):
        value = self.computer.input_provider.get_input()
        self.position.store(memory, value, self.computer.relative_base)
        return self.computer.program_counter + 2

class OutputInstruction(IOInstruction):
    def __init__(self, computer, position):
        super().__init__(computer, position)

    def execute(self, memory):
        output = self.position.load(memory, self.computer.relative_base)
        self.computer.output_sink.print_output(output)
        return self.computer.program_counter + 2

class JumpInstruction(Instruction):
    def __init__(self, computer, value, dest):
        super().__init__(computer)
        self.value = value
        self.dest = dest

    def execute(self, memory):
        value = self.value.load(memory, self.computer.relative_base)
        dest = self.dest.load(memory, self.computer.relative_base)
        if self.compare(value):
            return dest
        return self.computer.program_counter + 3

    def compare(self, value):
        raise NotImplementedError

class JumpIfTrueInstruction(JumpInstruction):
    def __init__(self, computer, value, dest):
        super().__init__(computer, value, dest)

    def compare(self, value):
        return value != 0

class JumpIfFalseInstruction(JumpInstruction):
    def __init__(self, computer, value, dest):
        super().__init__(computer, value, dest)

    def compare(self, value):
        return value == 0

class LessThanInstruction(ArithmeticInstruction):
    def __init__(self, computer, param1, param2, output):
        super().__init__(computer, param1, param2, output)

    def operation(self, input1, input2):
        if input1 < input2:
            return 1
        return 0

class EqualsInstruction(ArithmeticInstruction):
    def __init__(self, computer, param1, param2, output):
        super().__init__(computer, param1, param2, output)

    def operation(self, input1, input2):
        if input1 == input2:
            return 1
        return 0

class AdjustRelativeBaseInstruction(Instruction):
    def __init__(self, computer, param):
        super().__init__(computer)
        self.param = param

    def execute(self, memory):
        value = self.param.load(memory, self.computer.relative_base)
        self.computer.relative_base = self.computer.relative_base + value
        return self.computer.program_counter + 2