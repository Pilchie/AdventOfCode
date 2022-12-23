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

	startPoint := Point{0, 0}

	for !board.tiles[0][0][startPoint.col] {
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

	mapLocation := board.mapLocation(state.face, state.location)
	fmt.Printf("At (face: %d, row:%d, col:%d)\n", state.face, mapLocation.row, mapLocation.col)
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
	tiles      [6][][]bool
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
		tiles:      [6][][]bool{},
		dimensions: len(input[0]) / 3,
	}

	board.tiles[0] = parseFace(input, 0, 2*board.dimensions, board.dimensions)
	board.tiles[1] = parseFace(input, board.dimensions, 0, board.dimensions)
	board.tiles[2] = parseFace(input, board.dimensions, board.dimensions, board.dimensions)
	board.tiles[3] = parseFace(input, board.dimensions, 2*board.dimensions, board.dimensions)
	board.tiles[4] = parseFace(input, 2*board.dimensions, 2*board.dimensions, board.dimensions)
	board.tiles[5] = parseFace(input, 2*board.dimensions, 3*board.dimensions, board.dimensions)

	directions := []string{}

	start := 0
	directionLine := input[3*board.dimensions+1]
	for start < len(directionLine) {
		dir, s := extractpart(directionLine, start)
		directions = append(directions, dir)
		start = s
	}

	return board, directions
}

func parseFace(input []string, startingRow int, startingCol int, dimensions int) [][]bool {
	res := make([][]bool, dimensions)
	for r := 0; r < dimensions; r++ {
		res[r] = make([]bool, dimensions)
		for c := 0; c < dimensions; c++ {
			ch := input[startingRow+r][startingCol+c]
			if ch == '.' {
				res[r][c] = true
			} else if ch == '#' {
				res[r][c] = false
			} else {
				log.Fatalf("Unexpected char '%c' at '%d', '%d'\n", ch, startingRow+r, startingCol+c)
			}
		}
	}
	return res
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
		steps, err := strconv.Atoi(direction)
		if err != nil {
			log.Fatalf("Unable to convert steps to number: %s\n", direction)
		}
		res := state
		for i := 0; i < steps; i++ {
			res = b.next(res)
		}
		return res
	}
}

func (b *Board2) next(state State2) State2 {
	newRow := state.location.row + state.rowDelta
	newCol := state.location.col + state.colDelta

	if newRow < 0 || newCol < 0 || newRow == b.dimensions || newCol == b.dimensions {
		// We hit the edge of our face, move to the next one
		return b.transition(state)
	} else if !b.tiles[state.face-1][newRow][newCol] {
		// We hit a wall - stop here.
		return state
	} else {
		// Just move to the tile
		res := state
		res.location = Point{row: newRow, col: newCol}
		return res
	}
}

func (b *Board2) transition(state State2) State2 {
	res := state
	switch state.face {
	case 1:
		if state.rowDelta > 0 {
			res.face = 4
			res.location.row = 0
		} else if state.rowDelta < 0 {
			res.face = 5
			res.location.row = b.dimensions - 1
		} else if state.colDelta < 0 {
			res.face = 3
			res.location.row = 0
			res.location.col = state.location.row
			res.rowDelta = 1
			res.colDelta = 0
		} else if state.colDelta > 0 {
			res.face = 6
			res.location.col = b.dimensions - 1
			res.colDelta = -1
		}
	case 2:
		if state.rowDelta > 0 {
			res.face = 5
			res.location.row = b.dimensions - 1
			res.location.col = b.dimensions - state.location.col - 1
			res.rowDelta = -1
		} else if state.rowDelta < 0 {
			res.face = 1
			res.location.row = 0
			res.location.col = b.dimensions - state.location.col - 1
			res.rowDelta = 1
		} else if state.colDelta < 0 {
			res.face = 6
			res.location.row = b.dimensions - 1
			res.location.col = b.dimensions - state.location.col - 1
			res.rowDelta = -1
			res.colDelta = 0
		} else if state.colDelta > 0 {
			res.face = 3
			res.location.col = 0
		}
	case 3:
		if state.rowDelta > 0 {
			res.face = 5
			res.location.row = b.dimensions - state.location.col - 1
			res.location.col = 0
			res.rowDelta = 0
			res.colDelta = 1
		} else if state.rowDelta < 0 {
			res.face = 1
			res.location.row = state.location.col
			res.location.col = 0
			res.rowDelta = 0
			res.colDelta = 1
		} else if state.colDelta < 0 {
			res.face = 2
			res.location.col = b.dimensions - 1
		} else if state.colDelta > 0 {
			res.face = 4
			res.location.col = 0
		}
	case 4:
		if state.rowDelta > 0 {
			res.face = 5
			res.location.row = 0
		} else if state.rowDelta < 0 {
			res.face = 1
			res.location.row = b.dimensions - 1
		} else if state.colDelta < 0 {
			res.face = 3
			res.location.col = b.dimensions - 1
		} else if state.colDelta > 0 {
			res.face = 6
			res.location.row = 0
			res.location.col = b.dimensions - state.location.row - 1
			res.rowDelta = 1
			res.colDelta = 0
		}
	case 5:
		if state.rowDelta > 0 {
			res.face = 2
			res.location.row = b.dimensions - 1
			res.location.col = b.dimensions - state.location.col - 1
			res.rowDelta = -1
		} else if state.rowDelta < 0 {
			res.face = 4
			res.location.row = b.dimensions - 1
		} else if state.colDelta < 0 {
			res.face = 3
			res.location.row = b.dimensions - 1
			res.location.col = b.dimensions - state.location.row - 1
			res.rowDelta = -1
			res.colDelta = 0
		} else if state.colDelta > 0 {
			res.face = 6
			res.location.col = 0
		}
	case 6:
		if state.rowDelta > 0 {
			res.face = 2
			res.location.row = b.dimensions - state.location.col - 1
			res.location.col = 0
			res.rowDelta = 0
			res.colDelta = 1
		} else if state.rowDelta < 0 {
			res.face = 4
			res.location.row = b.dimensions - state.location.col - 1
			res.location.col = b.dimensions - 1
			res.rowDelta = 0
			res.colDelta = -1
		} else if state.colDelta < 0 {
			res.face = 5
			res.location.col = b.dimensions - 1
		} else if state.colDelta > 0 {
			res.face = 1
			res.location.col = b.dimensions - 1
			res.colDelta = -1
		}
	}

	if !b.tiles[res.face-1][res.location.row][res.location.col] {
		// There is a wall at the transition point, don't move after all
		return state
	} else {
		return res
	}
}

func (b *Board2) mapLocation(face int, locationOnFace Point) Point {
	switch face {
	case 1:
		return Point{row: locationOnFace.row + 1, col: locationOnFace.col + 2*b.dimensions + 1}
	case 2:
		return Point{row: locationOnFace.row + b.dimensions + 1, col: locationOnFace.col + 1}
	case 3:
		return Point{row: locationOnFace.row + b.dimensions + 1, col: locationOnFace.col + b.dimensions + 1}
	case 4:
		return Point{row: locationOnFace.row + b.dimensions + 1, col: locationOnFace.col + 2*b.dimensions + 1}
	case 5:
		return Point{row: locationOnFace.row + 2*b.dimensions + 1, col: locationOnFace.col + 2*b.dimensions + 1}
	case 6:
		return Point{row: locationOnFace.row + 2*b.dimensions + 1, col: locationOnFace.col + 3*b.dimensions + 1}
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
	if res.colDelta == 1 {
		res.colDelta = 0
		res.rowDelta = -1
	} else if s.rowDelta == 1 {
		res.colDelta = 1
		res.rowDelta = 0
	} else if s.colDelta == -1 {
		res.colDelta = 0
		res.rowDelta = 1
	} else if s.rowDelta == -1 {
		res.colDelta = -1
		res.rowDelta = 0
	} else {
		log.Fatalf("Unexpected facing state %d,%d\n", res.rowDelta, res.colDelta)
	}
	return res
}

func (s *State2) rotateRight() State2 {
	res := s.rotateLeft()
	res.rowDelta *= -1
	res.colDelta *= -1
	return res
}
