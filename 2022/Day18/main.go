package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
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

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	part1(cubes)
	part2(cubes)
}

func part1(cubes [][3]int) {
	area := len(cubes) * 6
	area -= adjacentFaces(cubes, 0)
	area -= adjacentFaces(cubes, 1)
	area -= adjacentFaces(cubes, 2)

	fmt.Printf("The area is %d\n", area)
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

func part2(cubes [][3]int) {
	// Determine the bounding box, and, starting from one corner, enumerate all reachable cubes and count their faces.

	startTime := time.Now()

	mins := [3]int{math.MaxInt, math.MaxInt, math.MaxInt}
	maxs := [3]int{math.MinInt, math.MinInt, math.MinInt}

	cubePoints := make(map[[3]int]bool)
	for _, p := range cubes {
		cubePoints[p] = true
		for ax := 0; ax < len(mins); ax++ {
			if p[ax] < mins[ax] {
				mins[ax] = p[ax]
			}
			if p[ax] > maxs[ax] {
				maxs[ax] = p[ax]
			}
		}
	}

	// Start one past the minimums, in case there is a cube at minx,miny,minz
	toExplore := [][3]int{{mins[0] - 1, mins[1] - 1, mins[2] - 1}}
	seen := map[[3]int]bool{}
	surfaceArea := 0

	for len(toExplore) > 0 {
		current := toExplore[0]
		toExplore = toExplore[1:]

		if _, ok := seen[current]; ok {
			continue
		}
		seen[current] = true

		for ax := 0; ax < len(mins); ax++ {
			// explore negative direction
			if current[ax] > mins[ax]-1 {
				potential := current
				potential[ax]--
				if _, ok := cubePoints[potential]; ok {
					// There is a cube there, so there is a unit of surface area between this cube and it
					surfaceArea++
				} else {
					// No cube, so add it to the list to explore
					toExplore = append(toExplore, potential)
				}
			}
			// explore positive direction
			if current[ax] <= maxs[ax]+1 {
				potential := current
				potential[ax]++
				if _, ok := cubePoints[potential]; ok {
					// There is a cube there, so there is a unit of surface area between this cube and it
					surfaceArea++
				} else {
					// No cube, so add it to the list to explore
					toExplore = append(toExplore, potential)
				}
			}
		}
	}

	fmt.Printf("Exterior surface area is %d (calculated in %v)\n", surfaceArea, time.Since(startTime))
}
