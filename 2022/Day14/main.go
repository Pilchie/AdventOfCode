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

type Point struct {
	x int
	y int
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	cave := map[Point]bool{}
	for scanner.Scan() {
		line := scanner.Text()

		points := parsePoints(strings.Split(line, " -> "))
		len := len(points)
		for i := 1; i < len; i++ {
			if points[i-1].x == points[i].x {
				for y := intmin(points[i-1].y, points[i].y); y <= intmax(points[i-1].y, points[i].y); y++ {
					cave[Point{x: points[i].x, y: y}] = true
				}
			} else {
				for x := intmin(points[i-1].x, points[i].x); x <= intmax(points[i-1].x, points[i].x); x++ {
					cave[Point{x: x, y: points[i].y}] = true
				}
			}
		}
	}

	maxy := math.MinInt
	for p := range cave {
		maxy = intmax(maxy, p.y)
	}

	fmt.Println("Starting:")
	printCave(cave)

	count := 0
	done := false
	for !done {
		initial := Point{500, 0}
		sand := initial
		for {
			newSand := fall(sand, cave, maxy)
			//printCave(cave)
			if newSand == initial {
				fmt.Println("Entrance blocked")
				count++
				done = true
				break
			} else if newSand == sand {
				fmt.Println("Sand resting")
				count++
				cave[newSand] = false
				break
			}

			sand = newSand
		}
	}

	fmt.Printf("The count is %d\n", count)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func parsePoints(inputs []string) []Point {
	res := []Point{}
	for i := range inputs {
		parts := strings.Split(inputs[i], ",")
		x, _ := strconv.Atoi(parts[0])
		y, _ := strconv.Atoi(parts[1])
		p := Point{x, y}
		res = append(res, p)
	}
	return res
}

func fall(p Point, cave map[Point]bool, maxy int) Point {
	// If we're already on the floor, we're done
	if p.y == maxy+1 {
		return p
	}

	// Try down one.
	p.y++
	if _, ok := cave[p]; ok {
		// Nope, taken. Try left of down one
		p.x--
		if _, ok := cave[p]; ok {
			// Nope, taken. Try right of down one
			p.x += 2
			if _, ok := cave[p]; ok {
				// Nope, taken, this sand can't fall, so reset to starting values
				p.x--
				p.y--
			}
		}
	}
	return p
}

func printCave(cave map[Point]bool) {
	minx := math.MaxInt
	maxx := math.MinInt
	miny := math.MaxInt
	maxy := math.MinInt
	for p := range cave {
		minx = intmin(minx, p.x)
		maxx = intmax(maxx, p.x)
		miny = intmin(miny, p.y)
		maxy = intmax(maxy, p.y)
	}
	for y := miny; y <= maxy; y++ {
		for x := minx; x <= maxx; x++ {
			b, ok := cave[Point{x, y}]
			if ok {
				if b {
					fmt.Print("#")
				} else {
					fmt.Print("o")
				}
			} else {
				print(".")
			}
		}
		fmt.Println()
	}
	fmt.Println()
}

func intmin(i1 int, i2 int) int {
	if i1 <= i2 {
		return i1
	} else {
		return i2
	}
}

func intmax(i1 int, i2 int) int {
	if i1 >= i2 {
		return i1
	} else {
		return i2
	}
}
