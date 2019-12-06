package main

import "fmt"

func main() {

}

// ParameterMode represents how to interpret a parameter
type ParameterMode int

const (
	// Position means interpret the value at the memory position
	Position ParameterMode = iota

	// Immediate means to use the value directly
	Immediate
)

// Instruction represents an IntCode instruction
type Instruction interface {
	Parameters() []Parameter
}

// Parameter represents a parameter to an instruction
type Parameter struct {
	Value int
	Mode  ParameterMode
}

// StopInstruction represents a stop instruction (opcode 99)
type StopInstruction struct {
}

// Parameters returns the parameters of a StopInstruction
func (si StopInstruction) Parameters() []Parameter {
	return nil
}

type AddInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (ai AddInstruction) Parameters() []Parameter {
	return []Parameter{ai.Input1, ai.Input2, ai.Output}
}

type MultiplyInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (mi MultiplyInstruction) Parameters() []Parameter {
	return []Parameter{mi.Input1, mi.Input2, mi.Output}
}

type InputInstruction struct {
	Position Parameter
}

func (ii InputInstruction) Parameters() []Parameter {
	return []Parameter{ii.Position}
}

type OutputInstruction struct {
	Position Parameter
}

func (oi OutputInstruction) Parameters() []Parameter {
	return []Parameter{oi.Position}
}
// ParseInstruction extracts the instructions at the beginning of memory, then returns it,
func ParseInstruction(memory []int) (Instruction, []int) {
	opcodeAndParameterModes := memory[0]
	opcode := opcodeAndParameterModes % 100
	if opcode == 99 {
		return StopInstruction{}, memory[1:]
	}

	//parameterModes := opcodeAndParameterModes / 100

	switch opcode {
	case 1:
		return AddInstruction{
			Input1: Parameter{Value: memory[1], Mode: Position},
			Input2: Parameter{Value: memory[2], Mode: Position},
			Output: Parameter{Value: memory[3], Mode: Position},
		}, memory[4:]
	case 2:
		return MultiplyInstruction{
			Input1: Parameter{Value: memory[1], Mode: Position},
			Input2: Parameter{Value: memory[2], Mode: Position},
			Output: Parameter{Value: memory[3], Mode: Position},
		}, memory[4:]
	case 3:
		return InputInstruction{
			Position: Parameter{Value: memory[1], Mode: Position},
		}, memory[2:]
	case 4:
		return OutputInstruction{
			Position: Parameter{Value: memory[1], Mode: Position},
		}, memory[2:]
	}
	return nil, nil
}

// RunProgram runs an "IntCode" program
func RunProgram(program []int, trace bool) []int {

	for i := 0; ; i += 4 {
		opcode := program[i]
		if opcode == 99 {
			return program
		}

		input1Addr := program[i+1]
		input2Addr := program[i+2]
		outputAddr := program[i+3]

		input1 := program[input1Addr]
		input2 := program[input2Addr]
		old := program[outputAddr]

		result := 0
		op := ""
		if opcode == 1 {
			result = input1 + input2
			op = "+"
		} else if opcode == 2 {
			result = input1 * input2
			op = "*"
		} else {
			fmt.Println("****ERROR: invalid opcode:", opcode)
		}

		program[outputAddr] = result

		if trace {
			fmt.Println("Processed:", Print([]int{opcode, input1Addr, input2Addr, outputAddr}), "\tas", input1, op, input2, "=", result, "\t- replaced", old, "at", outputAddr, "with", result)
		}

	}
}

// Print prints an array
func Print(array []int) string {
	l := len(array)
	if l == 0 {
		return "{}"
	}

	s := "{"
	for i := 0; i < len(array); i++ {
		s = fmt.Sprintf("%s%d, ", s, array[i])
	}
	s = s[0:len(s)-2] + "}"

	return s
}
