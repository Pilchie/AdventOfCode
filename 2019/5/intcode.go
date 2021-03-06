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

	RunProgram(program, ConsoleInputProvider{}, ConsoleOutputSink{})
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

type InputProvider interface {
	GetInput() int
}

type OutputSink interface {
	OutputValue(value int)
}

type ConsoleInputProvider struct {}

func (_ ConsoleInputProvider) GetInput() int {
	var i int
	fmt.Scan(&i)
	return i
}

type ConsoleOutputSink struct {
}

func (_ ConsoleOutputSink) OutputValue(value int) {
	fmt.Println(value)
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
	Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int
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

func (si StopInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	return -1
}

type AddInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (ai AddInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	input1 := ai.Input1.Load(memory)
	input2 := ai.Input2.Load(memory)
	result := input1 + input2
	ai.Output.Store(memory, result)
	return programCounter + 4
}

type MultiplyInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (mi MultiplyInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	input1 := mi.Input1.Load(memory)
	input2 := mi.Input2.Load(memory)
	result := input1 * input2
	mi.Output.Store(memory, result)
	return programCounter + 4
}

type InputInstruction struct {
	Position Parameter
}

func (ii InputInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	ii.Position.Store(memory, inputProvider.GetInput())
	return programCounter + 2
}

type OutputInstruction struct {
	Position Parameter
}

func (oi OutputInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	outputSink.OutputValue(oi.Position.Load(memory))
	return programCounter + 2
}

type JumpIfTrueInstruction struct {
	ValueToTest Parameter
	Destination Parameter
}

func (jiti JumpIfTrueInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	valueToTest := jiti.ValueToTest.Load(memory)
	destination := jiti.Destination.Load(memory)
	if valueToTest != 0 {
		return destination
	}

	return programCounter + 3
}

type JumpIfFalseInstruction struct {
	ValueToTest Parameter
	Destination Parameter
}

func (jifi JumpIfFalseInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	valueToTest := jifi.ValueToTest.Load(memory)
	destination := jifi.Destination.Load(memory)
	if valueToTest == 0 {
		return destination
	}
	
	return programCounter +3
}

type LessThanInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (lti LessThanInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	input1 := lti.Input1.Load(memory)
	input2 := lti.Input2.Load(memory)
	if input1 < input2 {
		lti.Output.Store(memory, 1)
	} else {
		lti.Output.Store(memory, 0)
	}
	return programCounter + 4
}

type EqualsInstruction struct {
	Input1 Parameter
	Input2 Parameter
	Output Parameter
}

func (ei EqualsInstruction) Execute(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	input1 := ei.Input1.Load(memory)
	input2 := ei.Input2.Load(memory)
	if input1 == input2 {
		ei.Output.Store(memory, 1)
	} else {
		ei.Output.Store(memory, 0)
	}
	return programCounter + 4
}

// ExecuteNextInstruction extracts the next instruction, and executes it.
// returns a negative number if the program is finished, otherwise the new value of the programCounter
func ExecuteNextInstruction(memory []int, programCounter int, inputProvider InputProvider, outputSink OutputSink) int {
	instruction := parseInstruction(memory, programCounter)

	return instruction.Execute(memory, programCounter, inputProvider, outputSink)
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
	case 5:
		return JumpIfTrueInstruction{
			ValueToTest: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
			Destination: Parameter{Value: memory[programCounter+2], Mode: getMode(parameterModes, 1)},
		}
	case 6:
		return JumpIfFalseInstruction{
			ValueToTest: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
			Destination: Parameter{Value: memory[programCounter+2], Mode: getMode(parameterModes, 1)},
		}
		case 7:
		return LessThanInstruction{
			Input1: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
			Input2: Parameter{Value: memory[programCounter+2], Mode: getMode(parameterModes, 1)},
			Output: Parameter{Value: memory[programCounter+3], Mode: getMode(parameterModes, 2)},
		}
	case 8:
		return EqualsInstruction{
			Input1: Parameter{Value: memory[programCounter+1], Mode: getMode(parameterModes, 0)},
			Input2: Parameter{Value: memory[programCounter+2], Mode: getMode(parameterModes, 1)},
			Output: Parameter{Value: memory[programCounter+3], Mode: getMode(parameterModes, 2)},
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
func RunProgram(program []int, inputProvider InputProvider, outputSink OutputSink) {
	programCounter := 0
	for {
		programCounter = ExecuteNextInstruction(program, programCounter, inputProvider, outputSink)
		if programCounter < 0 {
			return
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
