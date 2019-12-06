package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	scanner.Scan()
	program := parseProgram(scanner.Text())

	RunProgram(program)
}

func parseProgram(input string) []int {
	program := []int{}
	if input[len(input)-1] != ',' {
		input = input + ","
	}

	start := 0
	for index, char := range input {
		if char == ',' {
			i, _ := strconv.Atoi(input[start:index])
			program = append(program, i)
			start = index + 1
		}
	}
	return program
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
	Execute(memory []int) bool
}

// Parameter represents a parameter to an instruction
type Parameter struct {
	Value int
	Mode  ParameterMode
}

func (p Parameter) Load(memory []int) int {
	if p.Mode == Immediate {
		return p.Value
	} else if p.Mode == Position {
		return memory[p.Value]
	}

	// Uh, error?
	return 0
}

func (p Parameter) Store(memory []int, value int) {
	if p.Mode != Position {
		// Uh, add error handling?
	}

	memory[p.Value] = value
}

// StopInstruction represents a stop instruction (opcode 99)
type StopInstruction struct {
}

// Parameters returns the parameters of a StopInstruction
func (si StopInstruction) Parameters() []Parameter {
	return nil
}

func (si StopInstruction) Execute(memory []int) bool {
	return false
}

// AddInstruction represents an addition operation and its parameters
type AddInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (ai AddInstruction) Parameters() []Parameter {
	return []Parameter{ai.Input1, ai.Input2, ai.Output}
}

func (ai AddInstruction) Execute(memory []int) bool {
	input1 := ai.Input1.Load(memory)
	input2 := ai.Input2.Load(memory)
	result := input1 + input2
	ai.Output.Store(memory, result)
	return true
}

// MultiplyInstruction represents an addition operation and its parameters
type MultiplyInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (mi MultiplyInstruction) Parameters() []Parameter {
	return []Parameter{mi.Input1, mi.Input2, mi.Output}
}

func (mi MultiplyInstruction) Execute(memory []int) bool {
	input1 := mi.Input1.Load(memory)
	input2 := mi.Input2.Load(memory)
	result := input1 * input2
	mi.Output.Store(memory, result)
	return true
}

// InputInstruction represents an addition operation and its parameters
type InputInstruction struct {
	Position Parameter
}

func (ii InputInstruction) Parameters() []Parameter {
	return []Parameter{ii.Position}
}

func (ii InputInstruction) Execute(memory []int) bool {
	var i int
	fmt.Scan(&i)
	ii.Position.Store(memory, i)
	return true
}

// OutputInstruction represents an addition operation and its parameters
type OutputInstruction struct {
	Position Parameter
}

func (oi OutputInstruction) Parameters() []Parameter {
	return []Parameter{oi.Position}
}

func (oi OutputInstruction) Execute(memory []int) bool {
	fmt.Println(oi.Position.Load(memory))
	return true
}

// ExecuteNextInstruction extracts the next instruction, and executes it.
// returns a negative number if the program is finished, otherwise the amount to
// adjust the programCounter by
func ExecuteNextInstruction(memory []int, programCounter int) int {
	instruction := parseInstruction(memory, programCounter)
	if instruction.Execute(memory) {
		return 1 + len(instruction.Parameters())
	}

	return -1
}

func parseInstruction(memory []int, programCounter int) Instruction {
	opcodeAndParameterModes := memory[programCounter]
	opcode := opcodeAndParameterModes % 100
	if opcode == 99 {
		return StopInstruction{}
	}

	parameterModes := opcodeAndParameterModes / 100

	switch opcode {
	case 1:
		return AddInstruction{
			Input1: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
			Input2: Parameter{Value: memory[programCounter+2], Mode: getMode(parameterModes, 1)},
			Output: Parameter{Value: memory[programCounter+3], Mode: getMode(parameterModes, 2)},
		}
	case 2:
		return MultiplyInstruction{
			Input1: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
			Input2: Parameter{Value: memory[programCounter+2], Mode: getMode(parameterModes, 1)},
			Output: Parameter{Value: memory[programCounter+3], Mode: getMode(parameterModes, 2)},
		}
	case 3:
		return InputInstruction{
			Position: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
		}
	case 4:
		return OutputInstruction{
			Position: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
		}
	}

	return nil
}

func getMode(parameterModes int, position int) ParameterMode {
	mask := int(math.Pow10(position))
	parameterModes = parameterModes / mask
	parameterModes = parameterModes % 10
	if parameterModes == 1 {
		return Immediate
	}

	return Position
}

// RunProgram runs an "IntCode" program
func RunProgram(program []int) {
	programCounter := 0
	for {
		delta := ExecuteNextInstruction(program, programCounter)
		if delta < 0 {
			return
		}

		programCounter += delta
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
