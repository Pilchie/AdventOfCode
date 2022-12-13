package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	pair := 1
	sum := 0
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		firstLine := scanner.Text()
		first := parsePacket(firstLine)

		scanner.Scan()
		secondLine := scanner.Text()
		second := parsePacket(secondLine)

		res := comparePackets(first, second)
		//fmt.Printf("For pair %d, result was %d\n", pair, res)
		if res < 0 {
			fmt.Println("Decided to add:")
			fmt.Printf("  %s\n", firstLine)
			fmt.Printf("  %s\n", secondLine)
			fmt.Println()
			sum += pair
		}

		// consume blank line
		scanner.Scan()
		pair++
	}

	fmt.Printf("The sum of the pairs in the right order is %d\n", sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

type Either[A, B any] struct {
	value any
}

func (e *Either[A, B]) SetA(a A) {
	e.value = a
}

func (e *Either[A, B]) SetB(b B) {
	e.value = b
}

func (e *Either[A, B]) IsA() bool {
	_, ok := e.value.(A)
	return ok
}

func (e *Either[A, B]) IsB() bool {
	_, ok := e.value.(B)
	return ok
}

type Packet struct {
	elements []Either[int, Packet]
}

func parsePacket(input string) Packet {
	elements := []Either[int, Packet]{}
	index := parseList(input, 0, &elements)
	if index != len(input) {
		log.Fatalf("parsePacket didn't consume all input, at: %d, expected: %d\n", index, len(input))
	}
	return Packet{elements}
}

func parseList(input string, index int, elements *[]Either[int, Packet]) int {
	if input[index] != '[' {
		log.Fatalf("Unexpected char '%c' at %d\n", input[index], index)
	}

	// Consume the [
	index++

	for input[index] != ']' {
		if input[index] == '[' {
			children := []Either[int, Packet]{}
			index = parseList(input, index, &children)
			*elements = append(*elements, Either[int, Packet]{value: Packet{elements: children}})
		} else {
			start := index
			for input[index] != ']' && input[index] != ',' {
				index++
			}
			val, err := strconv.Atoi(input[start:index])
			if err != nil {
				log.Fatalf("Failed to parse '%s' from '%d' to %d'.  Error: %s\n", input, start, index, err.Error())
			}
			*elements = append(*elements, Either[int, Packet]{value: val})
		}
		if input[index] == ',' {
			index++
		}
	}

	// Consume the ]
	index++
	return index
}

func comparePackets(p1 Packet, p2 Packet) int {
	i := 0
	p1len := len(p1.elements)
	p2len := len(p2.elements)

	for {
		if i == p1len && i == p2len {
			return 0
		} else if i == p1len {
			return -1
		} else if i == p2len {
			return 1
		} else {
			res := compare(p1.elements[i], p2.elements[i])
			if res != 0 {
				return res
			}
		}
		i++
	}
}

func compare(e1 Either[int, Packet], e2 Either[int, Packet]) int {
	if e1.IsA() && e2.IsA() {
		return e1.value.(int) - e2.value.(int)
	} else if e1.IsB() && e2.IsB() {
		return comparePackets(e1.value.(Packet), e2.value.(Packet))
	} else if e1.IsA() && e2.IsB() {
		p1 := Packet{elements: []Either[int, Packet]{{value: e1.value.(int)}}}
		return comparePackets(p1, e2.value.(Packet))
	} else if e1.IsB() && e2.IsA() {
		p2 := Packet{elements: []Either[int, Packet]{{value: e2.value.(int)}}}
		return comparePackets(e1.value.(Packet), p2)
	} else {
		log.Fatalf("Unexpected set of inputs to compare??? e1 (IsA %v, IsB %v), e2 (IsA %v, IsB %v), e1 '%v', e2 '%v'\n", e1.IsA(), e1.IsB(), e2.IsA(), e2.IsB(), e1, e2)
		return 0
	}
}
