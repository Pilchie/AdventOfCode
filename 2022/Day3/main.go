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

	sum := 0
	count := 0
	for scanner.Scan() {
		line := scanner.Text()
		count++
		set := map[byte]bool{}
		for i := 0; i < len(line)/2; i++ {
			set[line[i]] = true
		}

		for i := len(line) / 2; i < len(line); i++ {
			if _, ok := set[line[i]]; ok {
				priority := line[i] - 'A' + 1
				if priority <= 26 {
					priority += 26
				} else {
					priority = line[i] - 'a' + 1
				}
				sum += int(priority)
				fmt.Printf("Line length was %d, split at %d, Found %c, adding %d\n", len(line), len(line)/2, line[i], priority)

				if priority < 1 || priority > 52 {
					fmt.Printf("Calculated priority wrong with %d, char was %c", priority, line[i])
				}
				break
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("The sum of priorities is %d, from %d\n", sum, count)
}
