package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"time"

	"github.com/edwingeng/deque/v2"
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

type Tuple[T1 any, T2 any] struct {
	first  T1
	second T2
}

type State struct {
	pos      Point
	boardIdx int
	minute   int
	prev     *State
}

func part1(input []string) {
	board := parseBoard(input)

	fmt.Println("Initial board")
	board.print()

	boards := make([]Board, lcm(board.size.height, board.size.width))
	boards[0] = board
	for i := 1; i < len(boards); i++ {
		boards[i] = boards[i-1].next()
	}

	size := board.size
	target := Point{row: size.height, col: size.width - 1}
	searchSpace := deque.NewDeque[State]()
	searchSpace.PushBack(State{pos: Point{row: -1, col: 0}, boardIdx: 0, minute: 0, prev: nil})

	best := math.MaxInt
	var bestState *State
	seen := map[Tuple[Point, int]]int{}

	t0 := time.Now()

	for !searchSpace.IsEmpty() {
		current := searchSpace.PopFront()
		if current.pos == target && current.minute < best {
			best = current.minute
			bestState = &current
			continue
		}

		if time.Since(t0).Milliseconds() >= 2000 {
			t0 = time.Now()
			fmt.Printf("  Minute: %2d, at (r:%2d, c:%2d), %10d states left\n", current.minute, current.pos.row, current.pos.col, searchSpace.Len())
		}

		cacheEntry := Tuple[Point, int]{first: current.pos, second: current.boardIdx}
		if v, ok := seen[cacheEntry]; ok && v < current.minute {
			continue
		}
		seen[cacheEntry] = current.minute

		nextBoardIdx := (current.boardIdx + 1) % len(boards)
		nextBoard := boards[nextBoardIdx]

		below := Point{row: current.pos.row + 1, col: current.pos.col}
		if nextBoard.unoccupied(below) {
			searchSpace.PushFront(next(current, below, nextBoardIdx))
		}
		right := Point{row: current.pos.row, col: current.pos.col + 1}
		if nextBoard.unoccupied(right) {
			searchSpace.PushFront(next(current, right, nextBoardIdx))
		}
		if nextBoard.unoccupied(current.pos) {
			searchSpace.PushBack(next(current, current.pos, nextBoardIdx))
		}
		left := Point{row: current.pos.row, col: current.pos.col - 1}
		if nextBoard.unoccupied(left) {
			searchSpace.PushBack(next(current, left, nextBoardIdx))
		}
		above := Point{row: current.pos.row - 1, col: current.pos.col}
		if nextBoard.unoccupied(above) {
			searchSpace.PushBack(next(current, above, nextBoardIdx))
		}
	}

	fmt.Println("Final path was:")
	for cur := bestState; cur != nil; cur = cur.prev {
		fmt.Printf("Pos (r:%2d, c:%2d), boardIdx: %2d, minute: %2d\n", cur.pos.row, cur.pos.col, cur.boardIdx, cur.minute)
	}

	fmt.Printf("Shortest path is %d steps\n", best)
}

func next(current State, pos Point, nextBoardIdx int) State {
	return State{pos: pos, boardIdx: nextBoardIdx, minute: current.minute + 1, prev: &current}
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
