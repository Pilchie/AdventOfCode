package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	var result = 0
	for scanner.Scan() {
		line := scanner.Text()
		for i := 3; i < len(line); i++ {
			result = check_four(line, i)
			if result > 0 {
				fmt.Printf("Found start-of-packet at %d\n", result)
				break
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func check_four(line string, index int) int {
	set := make(map[byte]bool)
	set[line[index]] = true
	for j := 1; j <= 3; j++ {
		if _, contains := set[line[index-j]]; !contains {
			set[line[index-j]] = true
		} else {
			return -1
		}
	}

	return index + 1
}
