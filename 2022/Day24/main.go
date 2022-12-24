package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
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

type Point struct {
	row, col int
}

type Size struct {
	width, height int
}

type Board struct {
	blizzards map[Point][]byte
	size      Size
}

type State struct {
	pos      Point
	boardIdx int
}

func part1(input []string) {
	path := A_Star(input)
	fmt.Printf("Shortest path is %d steps\n", len(path)-1)
}

func A_Star(input []string) []State {
	board := parseBoard(input)

	fmt.Println("Initial board")
	board.print()
	boards := make([]Board, lcm(board.size.height, board.size.width))
	boards[0] = board
	for i := 1; i < len(boards); i++ {
		boards[i] = boards[i-1].next()
	}

	start := State{pos: Point{row: -1, col: 0}, boardIdx: 0}
	goal := State{pos: Point{row: board.size.height, col: board.size.width - 1}}

	// The set of discovered nodes that may need to be (re-)expanded.
	// Initially, only the start node is known.
	// This is usually implemented as a min-heap or priority queue rather than a hash-set.
	openSet := []State{start}

	// For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
	// to n currently known.
	cameFrom := map[State]State{}

	// For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
	gScore := map[State]int{}
	gScore[start] = 0

	// For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
	// how cheap a path could be from start to finish if it goes through n.
	fScore := map[State]int{}
	fScore[start] = h(start, goal)

	for len(openSet) > 0 {

		// This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
		var current State
		var minScore = math.MaxInt
		for n := range openSet {
			if s, ok := fScore[openSet[n]]; ok {
				if s < minScore {
					current = openSet[n]
					minScore = s
				}
			}
		}

		if current.pos == goal.pos {
			return reconstruct_path(cameFrom, current)
		}

		openSet = remove(openSet, current)

		//fmt.Printf("Trying current = Row:%d, Col:%d\n", current.row, current.col)
		neighbors := neighbors(current, boards)
		for n := range neighbors {
			neighbor := neighbors[n]
			// d(current,neighbor) is the weight of the edge from current to neighbor
			// tentative_gScore is the distance from start to the neighbor through current
			gcurrent, ok := gScore[current]
			if !ok {
				gcurrent = math.MaxInt
			}
			tentative_gScore := gcurrent + d(current, neighbor)

			gneighbor, ok := gScore[neighbor]
			if !ok {
				gneighbor = math.MaxInt
			}

			//fmt.Printf(" Considering neighbor r:%d,c:%d, with tentative_gScore %d and gneighbor %d\n", neighbor.row, neighbor.col, tentative_gScore, gneighbor)
			if tentative_gScore < gneighbor {
				// This path to neighbor is better than any previous one. Record it!
				cameFrom[neighbor] = current
				gScore[neighbor] = tentative_gScore
				fScore[neighbor] = tentative_gScore + h(neighbor, goal)
				if !contains(openSet, neighbor) {
					openSet = append(openSet, neighbor)
				}
			}
		}
	}

	return openSet
}

func h(s1, s2 State) int {
	return int(math.Abs(float64(s2.pos.row-s1.pos.row)) + math.Abs(float64(s2.pos.col-s1.pos.col)))
}

func d(s1, s2 State) int {
	return 1
}

func reconstruct_path(cameFrom map[State]State, current State) []State {
	total_path := []State{current}
	c := current
	ok := false
	for {
		c, ok = cameFrom[c]
		if !ok {
			break
		}
		total_path = append([]State{c}, total_path...)
	}
	return total_path
}

func next(current State, pos Point, nextBoardIdx int) State {
	return State{pos: pos, boardIdx: nextBoardIdx}
}

func parseBoard(input []string) Board {
	blizzards := map[Point][]byte{}
	width := 0
	for row, line := range input[1 : len(input)-1] {
		width = len(line) - 2
		for col := 1; col < len(line)-1; col++ {
			b := line[col]
			if b == '<' || b == '>' || b == '^' || b == 'v' {
				blizzards[Point{row: row, col: col - 1}] = []byte{line[col]}
			}
		}
	}

	return Board{
		blizzards: blizzards,
		size:      Size{width: width, height: len(input) - 2}}
}

func (board *Board) next() Board {
	new := map[Point][]byte{}
	for p, v := range board.blizzards {
		for _, b := range v {
			var next Point
			switch b {
			case '>':
				next = Point{col: p.col + 1, row: p.row}
				if next.col >= board.size.width {
					next.col = 0
				}
			case '<':
				next = Point{col: p.col - 1, row: p.row}
				if next.col < 0 {
					next.col = board.size.width - 1
				}
			case 'v':
				next = Point{col: p.col, row: p.row + 1}
				if next.row >= board.size.height {
					next.row = 0
				}
			case '^':
				next = Point{col: p.col, row: p.row - 1}
				if next.row < 0 {
					next.row = board.size.height - 1
				}
			default:
			}
			if existing, ok := new[next]; ok {
				new[next] = append(existing, b)
			} else {
				new[next] = []byte{b}
			}
		}
	}
	return Board{blizzards: new, size: board.size}
}

func (b *Board) unoccupied(p Point) bool {
	// Start and target are ok
	if p.row == -1 && p.col == 0 {
		return true
	} else if p.row == b.size.height && p.col == b.size.width-1 {
		return true
	}

	if p.row < 0 || p.col < 0 {
		return false
	} else if p.row >= b.size.height || p.col >= b.size.width {
		return false
	}

	_, occupied := b.blizzards[p]
	return !occupied
}

func (b *Board) print() {
	fmt.Print("#.")
	for i := 0; i < b.size.width; i++ {
		fmt.Print("#")
	}
	fmt.Println()
	for row := 0; row < b.size.height; row++ {
		fmt.Print("#")
		for col := 0; col < b.size.width; col++ {
			if v, ok := b.blizzards[Point{row: row, col: col}]; ok {
				if len(v) > 1 {
					fmt.Print(len(v))
				} else {
					fmt.Printf("%c", v[0])
				}
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println("#")
	}
	for i := 0; i < b.size.width; i++ {
		fmt.Print("#")
	}
	fmt.Println(".#")
	fmt.Println()
}

func part2(input []string) {
}

func lcm(a, b int) int {
	return a * b / gcd(a, b)
}

func gcd(a, b int) int {
	if b > a {
		return gcd(b, a)
	}

	if b == 0 {
		return a
	}

	return gcd(b, a%b)
}

func remove(set []State, s State) []State {
	res := []State{}
	for i := range set {
		if set[i] != s {
			res = append(res, set[i])
		}
	}

	return res
}

func contains(set []State, s State) bool {
	return indexof(set, s) >= 0
}

func indexof(set []State, s State) int {
	for i := range set {
		if set[i] == s {
			return i
		}
	}
	return -1
}

func neighbors(current State, boards []Board) []State {
	res := []State{}
	nextBoardIdx := (current.boardIdx + 1) % len(boards)
	nextBoard := boards[nextBoardIdx]

	below := Point{row: current.pos.row + 1, col: current.pos.col}
	if nextBoard.unoccupied(below) {
		res = append(res, next(current, below, nextBoardIdx))
	}
	right := Point{row: current.pos.row, col: current.pos.col + 1}
	if nextBoard.unoccupied(right) {
		res = append(res, next(current, right, nextBoardIdx))
	}
	if nextBoard.unoccupied(current.pos) {
		res = append(res, next(current, current.pos, nextBoardIdx))
	}
	left := Point{row: current.pos.row, col: current.pos.col - 1}
	if nextBoard.unoccupied(left) {
		res = append(res, next(current, left, nextBoardIdx))
	}
	above := Point{row: current.pos.row - 1, col: current.pos.col}
	if nextBoard.unoccupied(above) {
		res = append(res, next(current, above, nextBoardIdx))
	}
	return res
}
