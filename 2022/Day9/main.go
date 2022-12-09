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
	positions := make(map[Point]bool)

	points := [10]Point{}

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		dir := parts[0]
		dist, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Fatal(err.Error())
		}

		for i := 0; i < dist; i++ {
			points[0] = move_head(points[0], dir)
			for p := 1; p < len(points); p++ {
				points[p] = move_tail(points[p-1], points[p])
			}
			positions[points[len(points)-1]] = true
		}

		fmt.Printf("Processed instruction: %s\n", line)
		draw(points)
		fmt.Println()

	}

	fmt.Printf("The tail visited %d positions\n", len(positions))
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func move_head(p Point, dir string) Point {
	switch dir {
	case "L":
		return Point{x: p.x - 1, y: p.y}
	case "R":
		return Point{x: p.x + 1, y: p.y}
	case "U":
		return Point{x: p.x, y: p.y - 1}
	case "D":
		return Point{x: p.x, y: p.y + 1}
	default:
		log.Fatalf("Unexpected direction %s\n", dir)
		return p
	}
}

func move_tail(head Point, tail Point) Point {
	if head.x-tail.x > 1 {
		if head.y-tail.y > 1 {
			return Point{x: tail.x + 1, y: tail.y + 1}
		}
		if head.y-tail.y < -1 {
			return Point{x: tail.x + 1, y: tail.y - 1}
		}
		return Point{x: tail.x + 1, y: head.y}
	}

	if head.x-tail.x < -1 {
		if head.y-tail.y > 1 {
			return Point{x: tail.x - 1, y: tail.y + 1}
		}
		if head.y-tail.y < -1 {
			return Point{x: tail.x - 1, y: tail.y - 1}
		}
		return Point{x: tail.x - 1, y: head.y}
	}

	if head.y-tail.y > 1 {
		return Point{x: head.x, y: tail.y + 1}
	}

	if head.y-tail.y < -1 {
		return Point{x: head.x, y: tail.y - 1}
	}
	return tail
}

func draw(points [10]Point) {
	minx := math.MaxInt
	miny := math.MaxInt
	maxx := math.MinInt
	maxy := math.MinInt

	for ip := range points {
		p := points[ip]
		if p.x < minx {
			minx = p.x
		}
		if p.x > maxx {
			maxx = p.x
		}
		if p.y < miny {
			miny = p.y
		}
		if p.y > maxy {
			maxy = p.y
		}
	}

	for y := miny; y < maxy+1; y++ {
		for x := minx; x < maxx+1; x++ {
			pixel := "."
			for ip := range points {
				if x == points[ip].x && y == points[ip].y {
					pixel = fmt.Sprintf("%d", ip)
					break
				}
			}
			fmt.Printf(pixel)
		}
		fmt.Println()

	}
}
