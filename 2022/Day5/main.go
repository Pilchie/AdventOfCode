package main

import (
	"bufio"
	"container/list"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Instr struct {
	count int
	from  int
	to    int
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	stacks := []*list.List{}
	instructions := []Instr{}
	inStacks := true

	for scanner.Scan() {
		line := scanner.Text()
		//fmt.Printf("Parsing line: '%s'\n", line)
		if inStacks {
			if line[1] == '1' {
				inStacks = false
			} else {
				for i := 0; i < len(line); i++ {
					if i%4 == 1 && line[i] >= 'A' && line[i] <= 'Z' {
						stack := i / 4
						if len(stacks) <= stack {
							for j := len(stacks); j <= stack; j++ {
								stacks = append(stacks, list.New())
							}
						}
						stacks[stack].PushFront(line[i])
					}
				}
			}
		} else if len(line) > 0 {
			parts := strings.Split(line, " ")
			instructions = append(instructions, Instr{count: Atoi(parts[1]), from: Atoi(parts[3]), to: Atoi(parts[5])})
		}
	}

	for i := range instructions {
		fmt.Printf("Moving %d from %d to %d. From has %d, to has %d\n",
			instructions[i].count, instructions[i].from, instructions[i].to,
			stacks[instructions[i].from-1].Len(), stacks[instructions[i].to-1].Len())

		temp := list.New()
		for m := 0; m < instructions[i].count; m++ {
			crate := stacks[instructions[i].from-1].Back()
			stacks[instructions[i].from-1].Remove(crate)
			temp.PushBack(crate.Value)
		}

		for c := temp.Back(); c != nil; c = c.Prev() {
			stacks[instructions[i].to-1].PushBack(c.Value)
		}

		fmt.Printf("After move, from is: '")
		printstack(stacks[instructions[i].from-1])
		fmt.Print("', to is '")
		printstack(stacks[instructions[i].to-1])
		fmt.Println("'")
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Print("The top crate in each stack is: ")
	for s := range stacks {
		fmt.Printf("%c", stacks[s].Back().Value)
	}
	fmt.Println()
}

func printstack(l *list.List) {
	for c := l.Front(); c != nil; c = c.Next() {
		fmt.Printf("%c", c.Value)
	}
}

func Atoi(s string) int {
	i, err := strconv.Atoi(s)

	if err != nil {
		log.Fatal(err.Error())
	}
	return i
}
