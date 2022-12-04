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

	count := 0
	for scanner.Scan() {
		line := scanner.Text()
		sections := strings.Split(line, ",")
		min1, max1 := get_range(sections[0])
		min2, max2 := get_range(sections[1])

		if min1 <= min2 && max1 >= max2 || min2 <= min1 && max2 >= max1 {
			count++
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("The number of contained sections is %d\n", count)
}

func get_range(s string) (int, int) {
	parts := strings.Split(s, "-")
	min, err := strconv.Atoi(parts[0])
	if err != nil {
		log.Fatal(err)
	}

	max, err := strconv.Atoi(parts[1])
	if err != nil {
		log.Fatal(err)
	}
	return min, max
}
