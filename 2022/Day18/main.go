package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// Represent point by array instead of names, to make it easier to perform the same operation on each axis.
	cubes := [][3]int{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		parts := strings.Split(scanner.Text(), ",")
		cube := [3]int{}
		for i, p := range parts {
			cube[i], _ = strconv.Atoi(p)
		}
		cubes = append(cubes, cube)
	}

	area := len(cubes) * 6
	area -= adjacentFaces(cubes, 0)
	area -= adjacentFaces(cubes, 1)
	area -= adjacentFaces(cubes, 2)

	fmt.Printf("The area is %d\n", area)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func adjacentFaces(cubes [][3]int, axis int) int {
	sort.Slice(cubes, func(i, j int) bool {
		return cubes[i][axis] < cubes[j][axis]
	})

	adjacent := 0
	for i := range cubes {
		for j := i + 1; j < len(cubes); j++ {
			if cubes[j][axis] == cubes[i][axis]+1 {
				isAdjacent := true
				for ax := 0; ax < 3; ax++ {
					if ax != axis {
						if cubes[i][ax] != cubes[j][ax] {
							isAdjacent = false
							break
						}
					}
				}
				if isAdjacent {
					adjacent++
				}
			} else if cubes[j][axis] > cubes[i][axis]+1 {
				break
			}
		}
	}
	return adjacent * 2
}
