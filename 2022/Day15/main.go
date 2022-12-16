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

	locations := map[Point]int{}
	beacons := map[Point]bool{}

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		sx, _ := strconv.Atoi(parts[2][2 : len(parts[2])-1])
		sy, _ := strconv.Atoi(parts[3][2 : len(parts[3])-1])
		bx, _ := strconv.Atoi(parts[8][2 : len(parts[8])-1])
		by, _ := strconv.Atoi(parts[9][2:])
		beacons[Point{x: bx, y: by}] = false

		dx := intabs(bx - sx)
		dy := intabs(by - sy)
		dist := dx + dy

		locations[Point{x: sx, y: sy}] = dist
	}

	min := 0
	max := 4000000
	done := false
	for y := min; y <= max && !done; y++ {
		intersections := findIntersections(locations, y)
		sort.Slice(intersections, func(i, j int) bool {
			return intersections[i].min < intersections[j].min
		})

		intersections = mergeRanges(intersections)

		if len(intersections) > 1 {
			fmt.Printf("Intersections on y=%d are: %v\n", y, intersections)
			x := intersections[0].max + 1
			fmt.Printf("Tuning frequency is %d\n", x*4000000+y)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

type Point struct {
	x int
	y int
}

type Range struct {
	min int
	max int
}

func findIntersections(locations map[Point]int, y_value int) []Range {
	res := []Range{}
	for p := range locations {
		r := findIntersection(p, locations[p], y_value)
		if r != nil {
			res = append(res, *r)
		}
	}
	return res
}

func findIntersection(p Point, distance int, y_value int) *Range {
	//fmt.Printf("Finding the intersection of (%d,%d) with range %d, and line at y=%d",
	//	p.x, p.y, distance, y_value)
	y_dist := intabs(p.y - y_value)
	if y_dist > distance {
		//fmt.Printf(" - y_dist is %d, so no intersection\n", y_dist)
		return nil
	}

	min := p.x - (distance - y_dist)
	max := p.x + (distance - y_dist)
	//fmt.Printf(" - y_dist is %d, so intersection from %d to %d\n", y_dist, min, max)
	return &Range{min, max}
}

func mergeRanges(ranges []Range) []Range {
	length := len(ranges)
	if length == 1 {
		return ranges
	}

	merged := []Range{}
	start := 0
	for start < length {
		max := ranges[start].max
		end := start
		for end < length && ranges[end].min <= max+1 {
			max = intmax(max, ranges[end].max)
			end++
		}
		if end < length {
			merged = append(merged, Range{min: ranges[start].min, max: max})
			start = end
		} else {
			merged = append(merged, Range{min: ranges[start].min, max: intmax(max, ranges[length-1].max)})
			break
		}
	}
	return merged
}

func intabs(i int) int {
	if i < 0 {
		return -1 * i
	} else {
		return i
	}
}
func intmax(i, j int) int {
	if i >= j {
		return i
	} else {
		return j
	}
}
