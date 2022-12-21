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
			var inv func(int, int, bool) int
			switch parts[1] {
			case "+":
				op = func(l, r int) int { return l + r }
				inv = func(o, t int, oilhs bool) int { return t - o }

			case "-":
				op = func(l, r int) int { return l - r }
				inv = func(other, target int, otherIsLhs bool) int {
					if otherIsLhs {
						return other - target
					} else {
						return other + target
					}
				}

			case "*":
				op = func(l, r int) int { return l * r }
				inv = func(o, t int, oilhs bool) int { return t / o }

			case "/":
				op = func(l, r int) int { return l / r }
				inv = func(other, target int, otherIsLhs bool) int {
					if otherIsLhs {
						return other / target
					} else {
						return other * target
					}
				}

			default:
				log.Fatalf("Unexpected operation %v", parts[1])
			}

			term = Calculation{
				left:      parts[0],
				right:     parts[2],
				operation: op,
				inverse:   inv,
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
	res, _ := rootTerm.calculate(terms)
	fmt.Printf("The value is %d\n", res)
}

func part2(terms map[string]Term) {
	fmt.Println("Starting Part2")

	rootTerm, ok := terms["root"]
	if !ok {
		log.Fatal("Unable to find term 'root'")
	}

	rootTerm = Calculation{
		left:      rootTerm.(Calculation).left,
		right:     rootTerm.(Calculation).right,
		operation: func(l, r int) int { return l - r },
		inverse: func(other, target int, otherIsLhs bool) int {
			if otherIsLhs {
				return other - target
			} else {
				return other + target
			}
		},
	}

	res := rootTerm.find(0, terms)
	fmt.Printf("The necessary value is %d\n", res)
}

type Term interface {
	calculate(terms map[string]Term) (int, bool)
	find(target int, terms map[string]Term) int
}

type Number int

func (n Number) calculate(terms map[string]Term) (int, bool) {
	return int(n), false
}

func (n Number) find(target int, terms map[string]Term) int {
	return int(n)
}

type Calculation struct {
	left      string
	right     string
	operation func(int, int) int
	inverse   func(other int, target int, otherIsLhs bool) int
}

func (c Calculation) calculate(terms map[string]Term) (int, bool) {
	leftTerm, ok := terms[c.left]
	if !ok {
		log.Fatalf("couldn't find term %s", c.left)
	}
	rightTerm, ok := terms[c.right]
	if !ok {
		log.Fatalf("couldn't find term %s", c.right)
	}

	leftVal, leftHasHumn := leftTerm.calculate(terms)
	rightVal, rightHasHumn := rightTerm.calculate(terms)
	hasHumn := leftHasHumn || c.left == "humn" || rightHasHumn || c.right == "humn"
	return c.operation(leftVal, rightVal), hasHumn
}

func (c Calculation) find(target int, terms map[string]Term) int {
	leftTerm := terms[c.left]
	rightTerm := terms[c.right]
	leftVal, leftHasHumn := leftTerm.calculate(terms)
	rightVal, rightHasHumn := rightTerm.calculate(terms)

	if c.left == "humn" {
		return c.inverse(rightVal, target, false)
	} else if c.right == "humn" {
		return c.inverse(leftVal, target, true)
	}

	if leftHasHumn {
		return leftTerm.find(c.inverse(rightVal, target, false), terms)
	} else if rightHasHumn {
		return rightTerm.find(c.inverse(leftVal, target, true), terms)
	} else {
		log.Fatal("Neither side had humn?")
		return 0
	}
}
