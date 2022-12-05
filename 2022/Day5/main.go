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
		fmt.Printf("Parsing line: '%s'\n", line)
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
		for m := 0; m < instructions[i].count; m++ {
			crate := stacks[instructions[i].from-1].Back()
			stacks[instructions[i].from-1].Remove(crate)
			stacks[instructions[i].to-1].PushBack(crate.Value)
		}
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

func Atoi(s string) int {
	i, err := strconv.Atoi(s)

	if err != nil {
		log.Fatal(err.Error())
	}
	return i
}
