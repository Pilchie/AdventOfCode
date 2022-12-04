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
	firstline := make(map[byte]bool)
	for scanner.Scan() {
		line := scanner.Text()
		//fmt.Printf("The line is: %s, line number: %d, place group: %d\n", line, count, count%3)
		thisline := make(map[byte]bool)

		switch count % 3 {
		case 0:
			for i := 0; i < len(line); i++ {
				thisline[line[i]] = true
			}
		case 1:
			fallthrough
		case 2:
			for i := 0; i < len(line); i++ {
				_, ok := firstline[line[i]]
				//fmt.Printf("%d was %v, ", line[i], ok)
				if ok {
					thisline[line[i]] = true
				}
			}
		}
		//fmt.Printf("Prev map was %v, Current map is %v\n", firstline, thisline)

		firstline = thisline

		if count%3 == 2 {
			common := 0
			for key := range firstline {
				common++
				priority := key - 'A' + 1
				if priority <= 26 {
					priority += 26
				} else {
					priority = key - 'a' + 1
				}
				sum += int(priority)
				fmt.Printf("Found %c in common, adding %d to sum\n", key, priority)
			}
			if common == 0 {
				log.Fatal("Didn't find any items in common among 3 lines\n")
			} else if common > 1 {
				log.Fatal(fmt.Sprintf("%d items in common among 3 lines\n", common))
			}
		}
		count++
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("The sum of priorities is %d, from %d\n", sum, count)
}
