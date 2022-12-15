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

	locations := map[Point]int{}
	beacons := []Point{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		sx, _ := strconv.Atoi(parts[2][2 : len(parts[2])-1])
		sy, _ := strconv.Atoi(parts[3][2 : len(parts[3])-1])
		bx, _ := strconv.Atoi(parts[8][2 : len(parts[8])-1])
		by, _ := strconv.Atoi(parts[9][2:])
		beacons = append(beacons, Point{x: bx, y: by})

		dx := intabs(bx - sx)
		dy := intabs(by - sy)
		dist := dx + dy

		locations[Point{x: sx, y: sy}] = dist
	}

	y_value := 2000000
	intersections := findIntersections(locations, y_value)

	blocked := map[int]bool{}
	for b := range beacons {
		if beacons[b].y == y_value {
			blocked[beacons[b].x] = false
		}
	}
	for i := range intersections {
		fmt.Printf("No beacons from %3d to %3d\n", intersections[i].min, intersections[i].max)
		for x := intersections[i].min; x <= intersections[i].max; x++ {
			if _, ok := blocked[x]; !ok {
				blocked[x] = true
			}
		}
	}

	count := 0
	for x := range blocked {
		b := blocked[x]
		if b {
			count++
		}
	}

	fmt.Printf("The count is %d\n", count)

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
	fmt.Printf("Finding the intersection of (%d,%d) with range %d, and line at y=%d",
		p.x, p.y, distance, y_value)
	y_dist := intabs(p.y - y_value)
	if y_dist > distance {
		fmt.Printf(" - y_dist is %d, so no intersection\n", y_dist)
		return nil
	}

	min := p.x - (distance - y_dist)
	max := p.x + (distance - y_dist)
	fmt.Printf(" - y_dist is %d, so intersection from %d to %d\n", y_dist, min, max)
	return &Range{min, max}
}

func intabs(i int) int {
	if i < 0 {
		return -1 * i
	} else {
		return i
	}
}
