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

	scanner := bufio.NewScanner(f)
	sum := 0

	cycle := 0
	xreg := 1

	for scanner.Scan() {
		line := scanner.Text()
		fmt.Printf("Processed instruction: %s\n", line)
		if line == "noop" {
			cycle++
			sum += update(cycle, xreg)
		} else if strings.HasPrefix(line, "addx ") {
			cycle++
			sum += update(cycle, xreg)
			cycle++
			sum += update(cycle, xreg)

			parts := strings.Split(line, " ")
			val, err := strconv.Atoi(parts[1])
			if err != nil {
				log.Fatalf(err.Error())
			}
			xreg += val
		} else {
			log.Fatalf("Unexpected instruction %s\n", line)
		}
	}

	fmt.Printf("The sum of the signal strengths is %d\n", sum)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func update(cycle int, xreg int) int {
	if (cycle-20)%40 == 0 {
		log.Printf("x register during cycle %d is %d -> signal strength: %d\n", cycle, xreg, cycle*xreg)
		return cycle * xreg
	}
	return 0
}
