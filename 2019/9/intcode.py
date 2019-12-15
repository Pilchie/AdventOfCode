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

    def run_to_completion(self, memory):
        while True:
            self.program_counter = self.execute_next_instruction(memory)
            if self.program_counter < 0:
                return

    def execute_next_instruction(self, memory):
        instruction = self.parse_instruction(memory)
        return instruction.execute(memory, self.program_counter, self.input_provider, self.output_sink)

    def parse_instruction(self, memory):
        opcode_and_parameter_modes = memory[self.program_counter]
        opcode = opcode_and_parameter_modes % 100
        if opcode == 99:
            return StopInstruction()

        parameter_modes = opcode_and_parameter_modes / 100

        if opcode == 1:
            return AddInstruction(
                param1=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory[self.program_counter + 2], self.get_mode(parameter_modes, 2)),
                output=Parameter(memory[self.program_counter + 3], self.get_mode(parameter_modes, 3)))
        elif opcode == 2:
            return MultiplyInstruction(
                param1=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory[self.program_counter + 2], self.get_mode(parameter_modes, 2)),
                output=Parameter(memory[self.program_counter + 3], self.get_mode(parameter_modes, 3)))
        elif opcode == 3:
            return InputInstruction(
                position=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)))
        elif opcode == 4:
            return OutputInstruction(
                position=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)))
        elif opcode == 5:
            return JumpIfTrueInstruction(
                value=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)),
                dest=Parameter(memory[self.program_counter + 2], self.get_mode(parameter_modes, 2)))
        elif opcode == 6:
            return JumpIfFalseInstruction(
                value=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)),
                dest=Parameter(memory[self.program_counter + 2], self.get_mode(parameter_modes, 2)))
        elif opcode == 7:
            return LessThanInstruction(
                param1=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory[self.program_counter + 2], self.get_mode(parameter_modes, 2)),
                output=Parameter(memory[self.program_counter + 3], self.get_mode(parameter_modes, 3)))
        elif opcode == 8:
            return EqualsInstruction(
                param1=Parameter(memory[self.program_counter + 1], self.get_mode(parameter_modes, 1)),
                param2=Parameter(memory[self.program_counter + 2], self.get_mode(parameter_modes, 2)),
                output=Parameter(memory[self.program_counter + 3], self.get_mode(parameter_modes, 3)))
        else:
            raise Invalid()

    def get_mode(self, parameter_modes, position):
        position = position - 1
        mask = 10 ** position
        parameter_modes = parameter_modes // mask
        parameter_modes = parameter_modes % 10
        if parameter_modes == 1:
            return ParameterMode.immediate

        return ParameterMode.position

class ParameterMode:
    immediate = "Immediate"
    position = "Position"

class Invalid(Exception):
    pass

class InputProvider:
    def get_input(self):
        return input()

class OutputSink:
    def print_output(self, output):
        print(output)

class Parameter:
    def __init__(self, value, mode):
        self.value = value
        self.mode = mode

    def load(self, memory):
        if self.mode == ParameterMode.immediate:
            return self.value
        elif self.mode == ParameterMode.position:
            return memory[self.value]
        
        raise Invalid()

    def store(self, memory, value):
        memory[self.value] = value

class Instruction:
    def execute(self, memory, program_counter, input_provider, output_sink):
        raise NotImplemented

class StopInstruction(Instruction):
    def execute(self, memory, program_counter, input_provider, output_sink):
        return -1

class ArithmeticInstruction(Instruction):
    def __init__(self, param1, param2, output):
        self.param1 = param1
        self.param2 = param2
        self.output = output

    def execute(self, memory, program_counter, input_provider, output_sink):
        input1 = self.param1.load(memory)
        input2 = self.param2.load(memory)
        result = self.operation(input1, input2)
        self.output.store(memory, result)
        return program_counter + 4

    def operation(self, input1, input2):
        raise NotImplementedError

class AddInstruction(ArithmeticInstruction):
    def __init__(self, param1, param2, output):
        super().__init__(param1, param2, output)

    def operation(self, input1, input2):
        return input1 + input2

class MultiplyInstruction(ArithmeticInstruction):
    def __init__(self, param1, param2, output):
        super().__init__(param1, param2, output)

    def operation(self, input1, input2):
        return input1 * input2

class IOInstruction(Instruction):
    def __init__(self, position):
        self.position = position

class InputInstruction(IOInstruction):
    def __init__(self, position):
        super().__init__(position)

    def execute(self, memory, program_counter, input_provider, output_sink):
        value = input_provider.get_input()
        self.position.store(memory, value)
        return program_counter + 2

class OutputInstruction(IOInstruction):
    def __init__(self, position):
        super().__init__(position)

    def execute(self, memory, program_counter, input_provider, output_sink):
        output_sink.print_output(self.position.load(memory))
        return program_counter + 2

class JumpInstruction(Instruction):
    def __init__(self, value, dest):
        self.value = value
        self.dest = dest

    def execute(self, memory, program_counter, input_provider, output_sink):
        value = self.value.load(memory)
        dest = self.dest.load(memory)
        if self.compare(value):
            return dest
        return program_counter + 3

    def compare(self, value):
        raise NotImplementedError

class JumpIfTrueInstruction(JumpInstruction):
    def __init__(self, value, dest):
        super().__init__(value, dest)

    def compare(self, value):
        return value != 0

class JumpIfFalseInstruction(JumpInstruction):
    def __init__(self, value, dest):
        super().__init__(value, dest)

    def compare(self, value):
        return value == 0

class LessThanInstruction(ArithmeticInstruction):
    def __init__(self, param1, param2, output):
        super().__init__(param1, param2, output)

    def operation(self, input1, input2):
        if input1 < input2:
            return 1
        return 0

class EqualsInstruction(ArithmeticInstruction):
    def __init__(self, param1, param2, output):
        super().__init__(param1, param2, output)

    def operation(self, input1, input2):
        if input1 == input2:
            return 1
        return 0
