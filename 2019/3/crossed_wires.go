package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	scanner.Scan()
	firstWire := parseWire(scanner.Text())
	scanner.Scan()
	secondWire := parseWire(scanner.Text())

	distance := CalculateDistance(firstWire, secondWire)
	fmt.Println(distance)
}

func parseWire(input string) []string {
	wire := []string{}
	start := 0
	for index, char := range input {
		if char == ',' {
			wire = append(wire, string(input[start:index]))
			start = index + 1
		}
	}
	return wire
}

// Point is an x, y pair.
type Point struct {
	X int
	Y int
}

// Line is a pair of points.  Lines are either vertical or horizontal
type Line struct {
	P1 Point
	P2 Point
}

// PointAndLength represents an intersection point and the length of wire
// needed to reach that intersection from the origin
type PointAndLength struct {
	Intersection Point
	WireLength   int
}

// CalculateDistance calculates the Manhattan Distance between the
// origin and the closest intersection of the two given wires.
func CalculateDistance(firstWire []string, secondWire []string) int {
	firstLines := makeLines(firstWire)
	secondLines := makeLines(secondWire)

	intersections := []PointAndLength{}
	firstLength := 0
	for _, firstLine := range firstLines {
		secondLength := 0
		for _, secondLine := range secondLines {
			found, intersection := intersection(firstLine, secondLine)
			if found && (intersection.X != 0 || intersection.Y != 0) {
				fmt.Printf("Adding intersection (%d, %d)\n", intersection.X, intersection.Y)
				length := firstLength + secondLength + lengthOf(Line{firstLine.P1, intersection}) + lengthOf(Line{secondLine.P1, intersection})
				intersections = append(intersections, PointAndLength{intersection, length})
			}

			secondLength += lengthOf(secondLine)
		}

		firstLength += lengthOf(firstLine)
	}

	// Map intersections to manhattan distances, then find the minimum
	min := math.MaxInt32
	for _, intersection := range intersections {
		//distance := manhattanDistance(intersection.Intersection)
		distance := intersection.WireLength
		if distance < min {
			min = distance
		}
	}

	return min
}

func lengthOf(line Line) int {
	if line.P1.X == line.P2.X {
		return int(math.Abs(float64(line.P2.Y - line.P1.Y)))
	}
	return int(math.Abs(float64(line.P2.X - line.P1.X)))
}

func manhattanDistance(point Point) int {
	return int(math.Abs(float64(point.X)) + math.Abs(float64(point.Y)))
}

func normalize(line Line) Line {
	if line.P2.X < line.P1.X ||
		line.P2.Y < line.P1.Y {
		return Line{line.P2, line.P1}
	}
	return line
}

func intersection(firstLine Line, secondLine Line) (bool, Point) {
	firstLine = normalize(firstLine)
	secondLine = normalize(secondLine)

	if firstLine.P1.X != firstLine.P2.X {
		if secondLine.P1.X != secondLine.P2.X {
			// Both horizontal
			if firstLine.P1.Y == secondLine.P1.Y {
				// On same Y.
			}
			return false, Point{0, 0}
		}

		if firstLine.P1.X <= secondLine.P1.X && firstLine.P2.X >= secondLine.P1.X {
			if secondLine.P1.Y <= firstLine.P1.Y && secondLine.P2.Y >= firstLine.P1.Y {
				return true, Point{secondLine.P1.X, firstLine.P1.Y}
			}
		}
	} else {
		if secondLine.P1.Y != secondLine.P2.Y {
			// Both vertical
			if firstLine.P1.X == secondLine.P1.X {
				//On the same X
			}
			return false, Point{0, 0}

		}

		if firstLine.P1.Y <= secondLine.P1.Y && firstLine.P2.Y >= secondLine.P1.Y {
			if secondLine.P1.X <= firstLine.P1.X && secondLine.P2.X >= firstLine.P1.X {
				return true, Point{firstLine.P1.X, secondLine.P1.Y}
			}
		}
	}

	return false, Point{0, 0}
}

func makeLines(wire []string) []Line {
	l1 := len(wire)
	lines := make([]Line, l1)
	firstPoint := Point{0, 0}
	for i := 0; i < l1; i++ {
		dir := wire[i][0:1]
		offset, _ := strconv.Atoi(wire[i][1:])
		var secondPoint Point
		switch dir {
		case "U":
			secondPoint = Point{firstPoint.X, firstPoint.Y + offset}
		case "D":
			secondPoint = Point{firstPoint.X, firstPoint.Y - offset}
		case "R":
			secondPoint = Point{firstPoint.X + offset, firstPoint.Y}
		case "L":
			secondPoint = Point{firstPoint.X - offset, firstPoint.Y}
		}

		lines[i] = Line{firstPoint, secondPoint}
		firstPoint = secondPoint
	}

	return lines
}
