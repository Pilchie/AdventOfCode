package main

import "fmt"

func main() {
	p := RunProgram([]int{
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
		2, 14, 0, 0})
	fmt.Println(p[0])
}

// RunProgram runs an "IntCode" program
func RunProgram(program []int) []int {
	l := len(program)
	for i := 0; i < l; i += 4 {
		opcode := program[i]
		if opcode == 99 {
			//fmt.Println("Got 99, exiting")
			return program
		}

		firstAddr := program[i+1]
		firstArg := program[firstAddr]
		secondAddr := program[i+2]
		secondArg := program[secondAddr]
		result := 0
		op := ""
		if opcode == 1 {
			result = firstArg + secondArg
			op = "+"
			//fmt.Println("Adding", first, second)
		} else if opcode == 2 {
			result = firstArg * secondArg
			op = "*"
			//fmt.Println("Multiplying", first, second)
		} else {
			fmt.Println("****ERROR: invalid opcode:", opcode)
		}
		//fmt.Println("Storing", result, "at", program[i+3])
		destination := program[i+3]
		old := program[destination]
		program[destination] = result
		fmt.Println("Processed:", Print([]int{opcode, firstAddr, secondAddr, destination}), "\tas", firstArg, op, secondArg, "=", result, "\t- replaced", old, "at", destination, "with", result)
		//fmt.Println("State: ", program)

	}
	return program
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
