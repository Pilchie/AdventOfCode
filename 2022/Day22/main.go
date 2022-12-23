package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
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
	fmt.Printf("---------------\n")
	fmt.Printf("Starting Part 1\n")
	fmt.Printf("---------------\n")
	board, directions := parseInput(input)
	startPoint := Point{1, 1}

	for {
		open, ok := board.tiles[startPoint]
		if ok && open {
			break
		}
		startPoint.col++
	}
	state := State{
		location: startPoint,
		facing:   0,
	}
	fmt.Printf("Starting at (row:%d, col:%d), facing %d\n", state.location.row, state.location.col, state.facing)

	for _, d := range directions {
		state = board.apply(state, d)
		fmt.Printf(" Applied %s, now at (row:%d, col:%d), facing %d\n", d, state.location.row, state.location.col, state.facing)
	}

	fmt.Printf("At (row:%d, col:%d), facing %d. Password is %d\n", state.location.row, state.location.col, state.facing, 1000*state.location.row+4*state.location.col+state.facing)
}

func part2(input []string) {
	fmt.Printf("---------------\n")
	fmt.Printf("Starting Part 2\n")
	fmt.Printf("---------------\n")
	board, directions := parseInput2(input)

	startPoint := Point{1, 1}

	for {
		open, ok := board.tiles[0][startPoint]
		if ok && open {
			break
		}
		startPoint.col++
	}
	state := State2{
		location: startPoint,
		face:     1,
		rowDelta: 0,
		colDelta: 1,
	}
	fmt.Printf("Starting at (face: %d, row:%d, col:%d)\n", state.face, state.location.row, state.location.col)

	for _, d := range directions {
		state = board.apply(state, d)
		fmt.Printf(" Applied %s, now at (face: %d, row:%d, col:%d)\n", d, state.face, state.location.row, state.location.col)
	}

	fmt.Printf("At (face: %d, row:%d, col:%d)", state.face, state.location.row, state.location.col)
	mapLocation := board.mapLocation(state.face, state.location)
	fmt.Printf("Password is %d\n", 1000*mapLocation.row+4*mapLocation.col+state.facing())
}

type Point struct {
	row int
	col int
}

type Board struct {
	tiles  map[Point]bool
	maxrow int
	maxcol int
}

func (b *Board) apply(state State, direction string) State {
	if direction == "L" {
		state.facing -= 1
		if state.facing < 0 {
			state.facing = 3
		}
	} else if direction == "R" {
		state.facing += 1
		if state.facing > 3 {
			state.facing = 0
		}
	} else {
		steps, err := strconv.Atoi(direction)
		if err != nil {
			log.Fatalf("Unexpected number of steps: '%s', Error: '%v'\n", direction, err.Error())
		}

		rowDelta := 0
		colDelta := 0
		switch state.facing {
		case 0:
			colDelta = 1
		case 1:
			rowDelta = 1
		case 2:
			colDelta = -1
		case 3:
			rowDelta = -1
		}
		for s := 0; s < steps; s++ {
			next := b.nextLocation(state.location, rowDelta, colDelta)
			if state.location == next {
				break
			}
			state.location = next
		}
	}
	return state
}

func (b *Board) nextLocation(start Point, rowDelta, colDelta int) Point {
	nextLocation := Point{row: start.row + rowDelta, col: start.col + colDelta}
	open, ok := b.tiles[nextLocation]
	if !ok {
		if rowDelta != 0 {
			startRow := 0
			if rowDelta < 0 {
				startRow = b.maxrow
			}
			wrappedLocation := Point{row: startRow, col: start.col}
			for {
				open, ok := b.tiles[wrappedLocation]
				if !ok {
					// No tile here, keep searching
				} else if !open {
					// We hit a wall, stop here
					fmt.Printf("Hit a wall while wapping. Stopping at %d, %d\n", start.row, start.col)
					return start
				} else {
					// We found a an open tile
					fmt.Printf("Wrapped to %d, %d\n", wrappedLocation.row, wrappedLocation.col)
					return wrappedLocation
				}
				wrappedLocation.row += rowDelta
			}
		} else if colDelta != 0 {
			startCol := 0
			if colDelta < 0 {
				startCol = b.maxcol
			}
			wrappedLocation := Point{row: start.row, col: startCol}
			for {
				open, ok := b.tiles[wrappedLocation]
				if !ok {
					// No tile here, keep searching
				} else if !open {
					// We hit a wall, stop here
					fmt.Printf("Hit a wall while wapping. Stopping at %d, %d\n", start.row, start.col)
					return start
				} else {
					// We found a an open tile
					fmt.Printf("Wrapped to %d, %d\n", wrappedLocation.row, wrappedLocation.col)
					return wrappedLocation
				}
				wrappedLocation.col += colDelta
			}
		} else {
			log.Fatalln("Unexpected direction to move")
			return nextLocation
		}
	} else if !open {
		// We hit a wall, stop here.
		fmt.Printf("Hit a wall at %d,%d\n", nextLocation.row, nextLocation.col)
		return start
	} else {
		return nextLocation
	}
}

type State struct {
	location Point
	facing   int
}

func parseInput(input []string) (Board, []string) {
	row := 1
	board := Board{tiles: make(map[Point]bool)}
	maxcol := 0
	for _, line := range input {
		if len(line) == 0 {
			break
		}

		col := 1
		for _, char := range line {
			if char == ' ' {
				// Empty space
			} else if char == '.' {
				board.tiles[Point{row, col}] = true
			} else if char == '#' {
				board.tiles[Point{row, col}] = false
			}
			col++
		}
		if col > maxcol {
			maxcol = col
		}
		row++
	}
	board.maxrow = row
	board.maxcol = maxcol

	directions := []string{}

	start := 0
	for start < len(input[row]) {
		dir, s := extractpart(input[row], start)
		directions = append(directions, dir)
		start = s
	}

	return board, directions
}

type Board2 struct {
	tiles      [6]map[Point]bool
	dimensions int
}

type State2 struct {
	location Point
	face     int
	rowDelta int
	colDelta int
}

func parseInput2(input []string) (Board2, []string) {
	board := Board2{
		tiles: [6]map[Point]bool{
			make(map[Point]bool),
			make(map[Point]bool),
			make(map[Point]bool),
			make(map[Point]bool),
			make(map[Point]bool),
			make(map[Point]bool),
		},
	}

	line := 0
	maxcol := 0
	for face := 0; face < 6; face++ {
		row := 1
		for _, char := range input[line] {
			col := 1
			if char == ' ' {
				// Empty space
			} else if char == '.' {
				board.tiles[face][Point{row, col}] = true
				col++

			} else if char == '#' {
				board.tiles[face][Point{row, col}] = false
				col++
			}
			maxcol = col - 1
		}
		row++
		line++
		if row == maxcol {
			face++
			continue
		}
	}
	board.dimensions = maxcol

	directions := []string{}

	start := 0
	for start < len(input[line]) {
		dir, s := extractpart(input[line], start)
		directions = append(directions, dir)
		start = s
	}

	return board, directions
}

func extractpart(input string, start int) (string, int) {
	end := start
	for end < len(input) && input[end] != 'R' && input[end] != 'L' {
		end++
	}
	if end > start {
		return input[start:end], end
	}
	return input[start : start+1], start + 1
}

func (b *Board2) apply(state State2, direction string) State2 {
	if direction == "R" {
		return state.rotateRight()
	} else if direction == "L" {
		return state.rotateLeft()
	} else {
		steps := 1
		res := state
		res.location = Point{
			row: state.location.row + state.rowDelta*steps,
			col: state.location.col + state.colDelta*steps}
		return res
	}
}

func (b *Board2) mapLocation(face int, locationOnFace Point) Point {
	switch face {
	case 1:
		return Point{row: locationOnFace.row, col: locationOnFace.col + 2*b.dimensions}
	case 2:
		return Point{row: locationOnFace.row + b.dimensions, col: locationOnFace.col}
	case 3:
		return Point{row: locationOnFace.row + b.dimensions, col: locationOnFace.col + b.dimensions}
	case 4:
		return Point{row: locationOnFace.row + b.dimensions, col: locationOnFace.col + 2*b.dimensions}
	case 5:
		return Point{row: locationOnFace.row + 2*b.dimensions, col: locationOnFace.col + 2*b.dimensions}
	case 6:
		return Point{row: locationOnFace.row + 2*b.dimensions, col: locationOnFace.col + 3*b.dimensions}
	default:
		log.Fatalf("Unexpected face: %d\n", face)
		return locationOnFace
	}
}

func (s *State2) facing() int {
	if s.colDelta == 1 {
		return 0
	} else if s.rowDelta == 1 {
		return 1
	} else if s.colDelta == -1 {
		return 2
	} else if s.rowDelta == -1 {
		return 3
	} else {
		log.Fatalf("Unexpected facing state %d,%d\n", s.rowDelta, s.colDelta)
		return -1
	}
}

func (s *State2) rotateLeft() State2 {
	res := *s
	if s.colDelta == 1 {
		s.colDelta = 0
		s.rowDelta = -1
	} else if s.rowDelta == 1 {
		s.colDelta = 1
		s.rowDelta = 0
	} else if s.colDelta == -1 {
		s.colDelta = 0
		s.rowDelta = 1
	} else if s.rowDelta == -1 {
		s.colDelta = -1
		s.rowDelta = 0
	} else {
		log.Fatalf("Unexpected facing state %d,%d\n", s.rowDelta, s.colDelta)
	}
	return res
}

func (s *State2) rotateRight() State2 {
	res := s.rotateLeft()
	res.rowDelta *= -1
	res.colDelta *= -1
	return res
}
