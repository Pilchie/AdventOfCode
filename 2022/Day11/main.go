package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type Test struct {
	divisor     int
	destIfTrue  int
	destIfFalse int
}

type Monkey struct {
	items       []int
	operation   func(x int, y int) int
	operand     int
	test        Test
	inspections int
}

func (m *Monkey) PlayTurn(monkeys []Monkey) {
	fmt.Printf("  Playing monkey with items %v\n", m.items)
	c := len(m.items)
	for i := range m.items {
		m.inspections++
		v := m.items[i]
		o := m.operand
		if o == 0 {
			o = v
		}
		v = m.operation(v, o)
		v = v / 3
		if v%m.test.divisor == 0 {
			//fmt.Printf("    throwing item with value %d to %d\n", v, m.test.destIfTrue)
			monkeys[m.test.destIfTrue].items = append(monkeys[m.test.destIfTrue].items, v)
		} else {
			//fmt.Printf("    throwing item with value %d to %d\n", v, m.test.destIfFalse)
			monkeys[m.test.destIfFalse].items = append(monkeys[m.test.destIfFalse].items, v)
		}
	}
	m.items = m.items[c:]
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	monkeys := []Monkey{}

	for {
		m := ParseMonkey(scanner)
		if m == nil {
			break
		}
		monkeys = append(monkeys, *m)
	}
	fmt.Printf("Parsed %d monkeys", len(monkeys))
	fmt.Println()

	for round := 0; round < 20; round++ {
		fmt.Printf("Playing round %d\n", round)
		for m := range monkeys {
			monkeys[m].PlayTurn(monkeys)
		}
	}

	m1, m2 := math.MinInt, math.MinInt
	for m := range monkeys {
		i := monkeys[m].inspections
		//fmt.Printf("Monkey %d: %d ", m, i)
		if i > m1 {
			m2 = m1
			m1 = i
		} else if i > m2 {
			m2 = i
		}
	}

	fmt.Printf("The level of monkey business after 20 rounds is %d*%d = %d\n", m1, m2, m1*m2)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func ParseMonkey(scanner *bufio.Scanner) *Monkey {
	// Get the monkey id
	if !scanner.Scan() {
		return nil
	}

	m := Monkey{}

	scanner.Scan()
	startingLine := scanner.Text()
	startingLine = startingLine[len("  Starting items: "):]
	items := strings.Split(startingLine, ", ")
	for i := range items {
		v, _ := strconv.Atoi(items[i])
		m.items = append(m.items, v)
	}

	scanner.Scan()
	operationLine := scanner.Text()
	operationLine = operationLine[len("  Operation: new = old "):]
	switch operationLine[0] {
	case '+':
		m.operation = func(x int, y int) int { return x + y }
		break
	case '*':
		m.operation = func(x int, y int) int { return x * y }
		break
	default:
		log.Fatalf("Unexpected operation: '%c'\n", operationLine[0])
	}
	if operationLine[2:] == "old" {
		m.operand = 0
	} else {
		m.operand, _ = strconv.Atoi(operationLine[2:])
	}

	scanner.Scan()
	testLine := scanner.Text()
	testLine = testLine[len("  Test: divisible by "):]
	m.test.divisor, _ = strconv.Atoi(testLine)

	scanner.Scan()
	trueLine := scanner.Text()
	trueLine = trueLine[len("    If true: throw to monkey "):]
	m.test.destIfTrue, _ = strconv.Atoi(trueLine)
	fmt.Printf("m.test.destIfTrue is: '%d'\n", m.test.destIfTrue)

	scanner.Scan()
	falseLine := scanner.Text()
	falseLine = falseLine[len("    If false: throw to monkey "):]
	m.test.destIfFalse, _ = strconv.Atoi(falseLine)
	fmt.Printf("m.test.destIfFalse is: '%d'\n", m.test.destIfFalse)

	// consume the blank line
	scanner.Scan()
	return &m
}
