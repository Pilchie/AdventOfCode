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

	scanner := bufio.NewScanner(f)

	start, end, theMap := ParseMap(scanner)

	fmt.Printf("Start is (%d, %d), End is (%d, %d)\n",
		start.row, start.col, end.row, end.col)

	path := A_Star(start, end, theMap)

	fmt.Println()
	for i := range path {
		fmt.Printf("(r:%d,c:%d)->\n", path[i].row, path[i].col)
	}
	fmt.Println()
	fmt.Printf("The shortest path is %d steps\n", len(path)-1)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

type Point struct {
	row int
	col int
}

func ParseMap(scanner *bufio.Scanner) (Point, Point, [][]byte) {
	start := Point{}
	end := Point{}
	rows := [][]byte{}

	rowi := 0
	for scanner.Scan() {
		line := scanner.Text()
		row := []byte{}
		for coli := range line {
			c := line[coli]
			if c == 'S' {
				start = Point{row: rowi, col: coli}
				c = 'a'
			} else if c == 'E' {
				end = Point{row: rowi, col: coli}
				c = 'z'
			}
			row = append(row, c-'a')
		}
		rows = append(rows, row)
		rowi++
	}

	return start, end, rows
}

func reconstruct_path(cameFrom map[Point]Point, current Point) []Point {
	total_path := []Point{current}
	c := current
	ok := false
	for {
		c, ok = cameFrom[c]
		if !ok {
			break
		}
		total_path = append([]Point{c}, total_path...)
	}
	return total_path
}

func A_Star(start Point, goal Point, theMap [][]byte) []Point {
	// The set of discovered nodes that may need to be (re-)expanded.
	// Initially, only the start node is known.
	// This is usually implemented as a min-heap or priority queue rather than a hash-set.
	openSet := []Point{start}

	// For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
	// to n currently known.
	cameFrom := map[Point]Point{}

	// For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
	gScore := map[Point]int{}
	gScore[start] = 0

	// For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
	// how cheap a path could be from start to finish if it goes through n.
	fScore := map[Point]int{}
	fScore[start] = h(start, goal)

	for len(openSet) > 0 {

		// This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
		var current Point
		var minScore = math.MaxInt
		for n := range openSet {
			if s, ok := fScore[openSet[n]]; ok {
				if s < minScore {
					current = openSet[n]
					minScore = s
				}
			}
		}

		if current == goal {
			return reconstruct_path(cameFrom, current)
		}

		openSet = remove(openSet, current)

		fmt.Printf("Trying current = Row:%d, Col:%d\n", current.row, current.col)
		neighbors := neighbors(current, theMap)
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

			fmt.Printf(" Considering neighbor r:%d,c:%d, with tentative_gScore %d and gneighbor %d\n", neighbor.row, neighbor.col, tentative_gScore, gneighbor)
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

	log.Fatal("Unable to find a path")
	return openSet
}

func d(p1 Point, p2 Point) int {
	return 1
}

func h(p1 Point, p2 Point) int {
	y := p2.row - p1.row
	x := p2.col - p1.col
	return int(math.Sqrt(float64(x*x + y*y)))
}

func neighbors(p Point, m [][]byte) []Point {
	res := []Point{}
	val := m[p.row][p.col]

	if p.row > 0 && m[p.row-1][p.col] <= val+1 {
		res = append(res, Point{row: p.row - 1, col: p.col})
	}

	if p.col > 0 && m[p.row][p.col-1] <= val+1 {
		res = append(res, Point{row: p.row, col: p.col - 1})
	}

	if p.row < len(m)-1 && m[p.row+1][p.col] <= val+1 {
		res = append(res, Point{row: p.row + 1, col: p.col})
	}

	if p.col < len(m[p.row])-1 && m[p.row][p.col+1] <= val+1 {
		res = append(res, Point{row: p.row, col: p.col + 1})
	}

	return res
}

func remove(set []Point, p Point) []Point {
	res := []Point{}
	for i := range set {
		if set[i] != p {
			res = append(res, set[i])
		}
	}

	return res
}

func contains(set []Point, p Point) bool {
	return indexof(set, p) >= 0
}

func indexof(set []Point, p Point) int {
	for i := range set {
		if set[i] == p {
			return i
		}
	}
	return -1
}
