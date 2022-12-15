package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
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

	locations := map[Point]float64{}
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

		dx := bx - sx
		dy := by - sy
		dist := math.Sqrt(float64(dx*dx + dy*dy))

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

func findIntersections(locations map[Point]float64, y_value int) []Range {
	res := []Range{}
	for p := range locations {
		r := findIntersection(p, locations[p], y_value)
		if r != nil {
			res = append(res, *r)
		}
	}
	return res
}

// From https://cp-algorithms.com/geometry/circle-line-intersection.html
func findIntersection(p Point, r float64, y_value int) *Range {
	a := 0.0
	b := 1.0
	c := float64(-y_value + p.y)
	x0 := -a * c / (a*a + b*b)
	//y0 := -b * c / (a*a + b*b)

	fmt.Printf("Finding intersection of circle at (%2d,%2d) radius: %5.2f", p.x, p.y, r)
	if c*c > r*r*(a*a+b*b+0.001) {
		fmt.Println(" - no intersection")
		return nil
	} else if math.Abs(c*c-r*r*(a*a+b*b)) < 0.001 {
		res := Range{min: int(math.Round(x0)) + p.x, max: int(math.Round(x0)) + p.x}
		fmt.Printf(" - single result at %d\n", res.min)
		return &res
	} else {
		d := r*r - c*c/(a*a+b*b)
		mult := math.Sqrt(d / (a*a + b*b))
		ax := x0 + b*mult
		bx := x0 - b*mult
		//ay = y0 - a*mult
		//by = y0 + a*mult
		res := Range{min: int(math.Round(math.Min(ax, bx))) + p.x, max: int(math.Round(math.Max(ax, bx))) + p.x}
		fmt.Printf(" - result from %d to %d\n", res.min, res.max)
		return &res
	}
}
