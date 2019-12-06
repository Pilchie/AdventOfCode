package main

import "fmt"

func main() {
	program := []int{
		1, 0, 0, 3,
		1, 1, 2, 3,
		1, 3, 4, 3,
		1, 5, 0, 3,
		2, 1, 10, 19,
		1, 19, 5, 23,
		2, 23, 6, 27,
		1, 27, 5, 31,
		2, 6, 31, 35,
		1, 5, 35, 39,
		2, 39, 9, 43,
		1, 43, 5, 47,
		1, 10, 47, 51,
		1, 51, 6, 55,
		1, 55, 10, 59,
		1, 59, 6, 63,
		2, 13, 63, 67,
		1, 9, 67, 71,
		2, 6, 71, 75,
		1, 5, 75, 79,
		1, 9, 79, 83,
		2, 6, 83, 87,
		1, 5, 87, 91,
		2, 6, 91, 95,
		2, 95, 9, 99,
		1, 99, 6, 103,
		1, 103, 13, 107,
		2, 13, 107, 111,
		2, 111, 10, 115,
		1, 115, 6, 119,
		1, 6, 119, 123,
		2, 6, 123, 127,
		1, 127, 5, 131,
		2, 131, 6, 135,
		1, 135, 2, 139,
		1, 139, 9, 0,
		99,
		2, 14, 0, 0}

	part1 := false
	if part1 {
		program[1] = 12
		program[2] = 2
		p := RunProgram(program, false)
		fmt.Println(p[0])
	} else {
		printNounAndVerb(program, false)
	}

}

func printNounAndVerb(program []int, trace bool) {
	for noun := 0; noun < 100; noun++ {
		for verb := 0; verb < 100; verb++ {
			if trace {
				fmt.Printf("Trying: %d, %d", noun, verb)
			}
			p := make([]int, len(program))
			copy(p, program)
			program[1] = noun
			program[2] = verb
			p = RunProgram(p, trace)
			output := p[0]
			if trace {
				fmt.Printf(" - output: %d\n", output)
			}

			if output == 19690720 {
				fmt.Printf("Found: %d\n", noun*100+verb)
				return
			}

		}
	}
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
