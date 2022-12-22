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

	board, directions := parseInput(lines)

	part1(board, directions)
	part2(board, directions)
}

func part1(board Board, directions []string) {
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

func part2(board Board, directions []string) {

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
