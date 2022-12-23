package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	lines := []string{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	part1(lines)
	part2(lines)
}

func part1(input []string) {
	fmt.Println("Starting Part 1")
	board := parseBoard(input)

	movements := []func(*Board, Point) Point{
		checkNorth,
		checkSouth,
		checkWest,
		checkEast,
	}

	fmt.Println("Initial:")
	board.print()
	fmt.Println()

	for round := 1; round <= 10; round++ {
		proposals := board.proposePositions(movements)
		board = board.applyProposals(proposals)

		// fmt.Println("After round ", round)
		// board.print()
		// fmt.Println()

		m := movements[0]
		movements = movements[1:]
		movements = append(movements, m)
	}

	fmt.Println("Final")
	board.print()
	fmt.Printf("There are %d unoccupied spaces\n", board.countEmpty())
}

type Pair[T any] struct {
	first  T
	second T
}

type Point struct {
	x, y int
}

type Board struct {
	elves map[Point]bool
}

func parseBoard(input []string) Board {
	res := Board{
		elves: map[Point]bool{},
	}

	for y, line := range input {
		for x, c := range line {
			if c == '#' {
				res.elves[Point{x: x, y: y}] = true
			}
		}
	}
	return res
}

func (b *Board) proposePositions(movements []func(*Board, Point) Point) []Pair[Point] {
	res := []Pair[Point]{}
	for p := range b.elves {
		if b.shouldMove(p) {
			c := len(res)
			for _, m := range movements {
				if n := m(b, p); n != p {
					res = append(res, Pair[Point]{first: p, second: n})
					break
				}
			}

			// We wanted to move, but there was no where to go.
			if c == len(res) {
				res = append(res, Pair[Point]{first: p, second: p})
			}
		} else {
			res = append(res, Pair[Point]{first: p, second: p})
		}
	}
	return res
}

func (b *Board) shouldMove(p Point) bool {
	for x := -1; x <= 1; x++ {
		for y := -1; y <= 1; y++ {
			if x != 0 || y != 0 {
				if _, ok := b.elves[Point{x: p.x + x, y: p.y + y}]; ok {
					return true
				}
			}
		}
	}
	return false
}

func (b *Board) applyProposals(proposals []Pair[Point]) Board {
	new := make(map[Point]bool)

	// Sort by the new positions so that we can easily find things that are moving to the same position.
	sort.Slice(proposals, func(i, j int) bool {
		leftPoint := proposals[i].second
		rightPoint := proposals[j].second
		if leftPoint.x < rightPoint.x {
			return true
		} else if leftPoint.x > rightPoint.x {
			return false
		} else {
			return leftPoint.y < rightPoint.y
		}
	})

	length := len(proposals)
	if length == 1 {
		new[proposals[0].second] = true
	} else {
		if proposals[0].second == proposals[1].second {
			new[proposals[0].first] = true
		} else {
			new[proposals[0].second] = true
		}

		for i := 1; i < length-1; i++ {
			prev := proposals[i-1]
			curr := proposals[i]
			next := proposals[i+1]
			if curr.second == prev.second || curr.second == next.second {
				new[curr.first] = true
			} else {
				new[curr.second] = true
			}
		}

		if proposals[length-2].second == proposals[length-1].second {
			new[proposals[length-1].first] = true
		} else {
			new[proposals[length-1].second] = true
		}
	}

	return Board{elves: new}
}

func (b *Board) bounds() (int, int, int, int) {
	minx := math.MaxInt
	miny := math.MaxInt
	maxx := math.MinInt
	maxy := math.MinInt

	for p := range b.elves {
		if p.x < minx {
			minx = p.x
		}
		if p.y < miny {
			miny = p.y
		}
		if p.x > maxx {
			maxx = p.x
		}
		if p.y > maxy {
			maxy = p.y
		}
	}
	return minx, miny, maxx, maxy
}

func (b *Board) print() {
	minx, miny, maxx, maxy := b.bounds()
	for y := miny; y <= maxy; y++ {
		for x := minx; x <= maxx; x++ {
			if _, ok := b.elves[Point{x: x, y: y}]; ok {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func (b *Board) countEmpty() int {
	minx, miny, maxx, maxy := b.bounds()
	res := 0
	for x := minx; x <= maxx; x++ {
		for y := miny; y <= maxy; y++ {
			if _, ok := b.elves[Point{x: x, y: y}]; !ok {
				res++
			}
		}
	}
	return res
}

func checkNorth(b *Board, p Point) Point {
	for x := -1; x <= 1; x++ {
		if _, ok := b.elves[Point{x: p.x + x, y: p.y - 1}]; ok {
			return p
		}
	}
	return Point{x: p.x, y: p.y - 1}
}

func checkSouth(b *Board, p Point) Point {
	for x := -1; x <= 1; x++ {
		if _, ok := b.elves[Point{x: p.x + x, y: p.y + 1}]; ok {
			return p
		}
	}
	return Point{x: p.x, y: p.y + 1}
}

func checkEast(b *Board, p Point) Point {
	for y := -1; y <= 1; y++ {
		if _, ok := b.elves[Point{x: p.x + 1, y: p.y + y}]; ok {
			return p
		}
	}
	return Point{x: p.x + 1, y: p.y}
}

func checkWest(b *Board, p Point) Point {
	for y := -1; y <= 1; y++ {
		if _, ok := b.elves[Point{x: p.x - 1, y: p.y + y}]; ok {
			return p
		}
	}
	return Point{x: p.x - 1, y: p.y}
}

func part2(input []string) {
	fmt.Println("Starting Part 2")
	board := parseBoard(input)

	movements := []func(*Board, Point) Point{
		checkNorth,
		checkSouth,
		checkWest,
		checkEast,
	}

	fmt.Println("Initial:")
	board.print()
	fmt.Println()

	round := 1
	for {
		proposals := board.proposePositions(movements)
		newBoard := board.applyProposals(proposals)

		if newBoard.equals(&board) {
			break
		}

		round++
		board = newBoard
		m := movements[0]
		movements = movements[1:]
		movements = append(movements, m)
	}

	fmt.Println("Final")
	board.print()
	fmt.Printf("It took %d rounds.\n", round)
}

func (b *Board) equals(other *Board) bool {
	if len(b.elves) != len(other.elves) {
		return false
	}

	for p, v := range b.elves {
		o, ok := other.elves[p]
		if !ok || v != o {
			return false
		}
	}

	return true
}
