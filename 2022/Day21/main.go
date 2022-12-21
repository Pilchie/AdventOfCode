package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	terms := map[string]Term{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		nameAndTerm := strings.Split(line, ": ")

		parts := strings.Split(nameAndTerm[1], " ")

		var term Term
		if len(parts) == 1 {
			v, err := strconv.Atoi(parts[0])
			if err != nil {
				log.Fatalf(err.Error())
			}
			term = Number(v)
		} else if len(parts) == 3 {
			var op func(int, int) int
			switch parts[1] {
			case "+":
				op = func(l, r int) int { return l + r }
				break
			case "-":
				op = func(l, r int) int { return l - r }
				break
			case "*":
				op = func(l, r int) int { return l * r }
				break
			case "/":
				op = func(l, r int) int { return l / r }
				break
			default:
				log.Fatalf("Unexpected operation %v", parts[1])
			}

			term = Calculation{
				left:      parts[0],
				right:     parts[2],
				operation: op,
			}
		} else {
			log.Fatalf("Unexpected input %v", parts)
		}

		terms[nameAndTerm[0]] = term

	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	part1(terms)
	part2(terms)
}

func part1(terms map[string]Term) {
	fmt.Println("Starting Part1")
	rootTerm, ok := terms["root"]
	if !ok {
		log.Fatal("Unable to find term 'root'")
	}
	fmt.Printf("The value is %d\n", rootTerm.calculate(terms))
}

func part2(terms map[string]Term) {
	fmt.Println("Starting Part2")
}

type Term interface {
	calculate(terms map[string]Term) int
}

type Number int

func (n Number) calculate(terms map[string]Term) int {
	return int(n)
}

type Calculation struct {
	left      string
	right     string
	operation func(int, int) int
}

func (c Calculation) calculate(terms map[string]Term) int {
	leftTerm, ok := terms[c.left]
	if !ok {
		log.Fatalf("couldn't find term %s", c.left)
	}
	rightTerm, ok := terms[c.right]
	if !ok {
		log.Fatalf("couldn't find term %s", c.right)
	}

	leftVal := leftTerm.calculate(terms)
	rightVal := rightTerm.calculate(terms)
	return c.operation(leftVal, rightVal)
}
