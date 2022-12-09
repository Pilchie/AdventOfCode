package main

import (
	"bufio"
	"fmt"
	"log"
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

	head := Point{x: 0, y: 0}
	tail := Point{x: 0, y: 0}

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		dir := parts[0]
		dist, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Fatal(err.Error())
		}

		for i := 0; i < dist; i++ {
			head = move_head(head, dir)
			tail = move_tail(head, tail)
			fmt.Printf("Head at (%d, %d), tail at (%d, %d)\n", head.x, head.y, tail.x, tail.y)
			positions[tail] = true
		}

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
		return Point{x: tail.x + 1, y: head.y}
	} else if head.x-tail.x < -1 {
		return Point{x: tail.x - 1, y: head.y}
	}

	if head.y-tail.y > 1 {
		return Point{x: head.x, y: tail.y + 1}
	} else if head.y-tail.y < -1 {
		return Point{x: head.x, y: tail.y - 1}
	}
	return tail
}
