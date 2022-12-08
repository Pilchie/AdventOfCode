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

	highest := 0
	for r := 1; r < len(rows)-1; r++ {
		for c := 1; c < len(rows[r])-1; c++ {
			ss := scenic_score(rows, r, c)
			if ss > highest {
				highest = ss
			}
		}
	}

	fmt.Printf("The highest scenic score is %d\n", highest)
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

func scenic_score(rows [][]int, row int, col int) int {
	height := rows[row][col]

	left := 0
	for c := col - 1; c >= 0; c-- {
		left++
		if rows[row][c] >= height {
			break
		}
	}

	right := 0
	for c := col + 1; c < len(rows[row]); c++ {
		right++
		if rows[row][c] >= height {
			break
		}
	}

	up := 0
	for r := row - 1; r >= 0; r-- {
		up++
		if rows[r][col] >= height {
			break
		}
	}

	down := 0
	for r := row + 1; r < len(rows); r++ {
		down++
		if rows[r][col] >= height {
			break
		}
	}

	ss := left * right * up * down
	fmt.Printf("For (%d, %d), the results are: u: %d, l: %d, r: %d, d: %d => %d\n",
		row, col, up, left, right, down, ss)
	return ss
}
