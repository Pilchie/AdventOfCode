package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Map struct {
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	var rows [][]int

	for scanner.Scan() {
		line := scanner.Text()
		row := make([]int, len(line))
		for i := range line {
			row[i] = (int)(line[i] - '0')
		}
		rows = append(rows, row)
	}

	// start with a count for the outside rows
	count := 2*len(rows) + 2*len(rows[0]) - 4

	// now add the interior nodes
	for r := 1; r < len(rows)-1; r++ {
		for c := 1; c < len(rows[r])-1; c++ {
			if is_visble(rows, r, c) {
				count++
			}
		}
	}

	fmt.Printf("There are %d visible trees\n", count)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func is_visble(rows [][]int, row int, col int) bool {
	h := rows[row][col]
	visible := true
	for r := 0; r < row; r++ {
		if rows[r][col] >= h {
			visible = false
			break
		}
	}
	if visible {
		fmt.Printf("Tree at (%d, %d) was visible from above\n", row, col)
		return true
	}

	visible = true
	for r := row + 1; r < len(rows); r++ {
		if rows[r][col] >= h {
			visible = false
			break
		}
	}
	if visible {
		fmt.Printf("Tree at (%d, %d) was visible from below\n", row, col)
		return true
	}

	visible = true
	for c := 0; c < col; c++ {
		if rows[row][c] >= h {
			visible = false
			break
		}
	}
	if visible {
		fmt.Printf("Tree at (%d, %d) was visible from left\n", row, col)
		return true
	}

	visible = true
	for c := col + 1; c < len(rows[row]); c++ {
		if rows[row][c] >= h {
			visible = false
			break
		}
	}
	if visible {
		fmt.Printf("Tree at (%d, %d) was visible from right\n", row, col)
		return true
	}

	return false
}
