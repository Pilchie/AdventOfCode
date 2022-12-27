package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"time"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Scan()
	jets := scanner.Text()
	//part1(jets)
	part2(jets)
}

func part1(jets string) {
	res := dropUntil(2022, jets)
	fmt.Printf("Highest rock is at %d\n", res)
}

func part2(jets string) {
	res := dropUntil(1000000000000, jets)
	fmt.Printf("Highest rock is at %d\n", res)
}

func dropUntil(steps int, jets string) int {
	stoppedRocks := []ShapeAndLoc{}
	highestSoFar := 0
	countStoppedRocks := 0
	jetIdx := 0
	shapeIdx := 0

	shapePoints := [][]Point{
		{{x: 0, y: 0}, {x: 1, y: 0}, {x: 2, y: 0}, {x: 3, y: 0}},
		{{x: 1, y: 0}, {x: 0, y: 1}, {x: 1, y: 1}, {x: 2, y: 1}, {x: 1, y: 2}},
		{{x: 0, y: 0}, {x: 1, y: 0}, {x: 2, y: 0}, {x: 2, y: 1}, {x: 2, y: 2}},
		{{x: 0, y: 0}, {x: 0, y: 1}, {x: 0, y: 2}, {x: 0, y: 3}},
		{{x: 0, y: 0}, {x: 1, y: 0}, {x: 0, y: 1}, {x: 1, y: 1}},
	}

	shapes := parseShapes(shapePoints)
	currentShapeAndLocation := ShapeAndLoc{shape: &shapes[shapeIdx], location: Point{x: 2, y: 3}}

	seenStates := map[StateKey]StateValue{}
	cycleHeight := 0
	cycleLength := 0
	streakCount := 0
	repetitions := 0

	verifyCycle := lcm(len(shapes), len(jets))

	t0 := time.Now()
	for countStoppedRocks < steps {
		done, newShapeAndLocation := fall(&stoppedRocks, jets[jetIdx], currentShapeAndLocation)
		if done {
			countStoppedRocks++
			stoppedRocks = append(stoppedRocks, newShapeAndLocation)
			if len(stoppedRocks)%1000 == 0 {
				stoppedRocks = stoppedRocks[len(stoppedRocks)-500:]
			}
			highestSoFar = highest(&stoppedRocks)

			stateKey := StateKey{shapeIdx: shapeIdx, jetIdx: jetIdx}
			if stateValue, ok := seenStates[stateKey]; ok {
				currentHeight := highestSoFar - stateValue.height
				currentLength := countStoppedRocks - stateValue.rockCount
				if currentHeight != cycleHeight || currentLength != cycleLength {
					cycleHeight = currentHeight
					cycleLength = currentLength
					streakCount = 0
				} else {
					streakCount++
				}
				if streakCount%1000 == 0 {
					fmt.Printf("Completed streak of length: %d (time: %v)\n", streakCount, time.Since(t0))
					t0 = time.Now()
				}
				if streakCount == verifyCycle {
					repetitions = (steps - countStoppedRocks) / cycleLength
					steps -= repetitions * cycleLength
					fmt.Printf("At step %d, continuing for %d more, then repeating %d times\n", countStoppedRocks, steps-countStoppedRocks, repetitions)
				}
			}
			seenStates[stateKey] = StateValue{rockCount: countStoppedRocks, height: highestSoFar}
			shapeIdx = increment(shapeIdx, len(shapes))
			currentShapeAndLocation = ShapeAndLoc{shape: &shapes[shapeIdx], location: Point{x: 2, y: highestSoFar + 3}}

			jetIdx = increment(jetIdx, len(jets))
			currentShapeAndLocation = fallSimple(jets[jetIdx], currentShapeAndLocation)

			jetIdx = increment(jetIdx, len(jets))
			currentShapeAndLocation = fallSimple(jets[jetIdx], currentShapeAndLocation)

			jetIdx = increment(jetIdx, len(jets))
			currentShapeAndLocation = fallSimple(jets[jetIdx], currentShapeAndLocation)

			jetIdx = increment(jetIdx, len(jets))
		} else {
			currentShapeAndLocation = newShapeAndLocation
			jetIdx = increment(jetIdx, len(jets))
		}
	}

	res := highest(&stoppedRocks)
	//printcave(stoppedRocks, currentShapeAndLocation, 0)
	return res + repetitions*cycleHeight
}

type StateKey struct {
	shapeIdx, jetIdx int
}

type StateValue struct {
	rockCount, height int
}

type Shape struct {
	minx   int
	maxx   int
	miny   int
	maxy   int
	points []Point
}

type ShapeAndLoc struct {
	shape    *Shape
	location Point
}

func parseShapes(shapePoints [][]Point) []Shape {
	res := []Shape{}

	for _, sp := range shapePoints {
		minx := math.MaxInt
		miny := math.MaxInt
		maxx := math.MinInt
		maxy := math.MinInt

		for _, p := range sp {
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

		res = append(res, Shape{
			minx, maxx, miny, maxy, sp,
		})
	}

	return res
}

func intersects(s1 ShapeAndLoc, s2 ShapeAndLoc) bool {
	for _, p1 := range s1.shape.points {
		for _, p2 := range s2.shape.points {
			if p1.x+s1.location.x == p2.x+s2.location.x && p1.y+s1.location.y == p2.y+s2.location.y {
				return true
			}
		}
	}
	return false
}

func highest(stopped *[]ShapeAndLoc) int {
	max := 0
	len := len(*stopped)
	for i := len - 1; i >= 0 && i > len-100; i-- {
		sl := (*stopped)[i]
		if sl.shape.maxy+sl.location.y > max {
			max = sl.shape.maxy + sl.location.y
		}
	}
	return max + 1
}

type Point struct {
	x int
	y int
}

func intersectsAny(stoppedRocksPtr *[]ShapeAndLoc, sl ShapeAndLoc) bool {
	stoppedRocks := *stoppedRocksPtr
	for i := len(stoppedRocks) - 1; i >= 0; i-- {
		if intersects(sl, stoppedRocks[i]) {
			return true
		}
	}
	return false
}

func fall(stoppedRocksPtr *[]ShapeAndLoc, jet byte, sl ShapeAndLoc) (bool, ShapeAndLoc) {
	// handle the jet
	if jet == '<' {
		sl.location.x--
		if sl.shape.minx+sl.location.x < 0 || intersectsAny(stoppedRocksPtr, sl) {
			sl.location.x++
		}
	} else if jet == '>' {
		sl.location.x++
		if sl.shape.maxx+sl.location.x >= 7 || intersectsAny(stoppedRocksPtr, sl) {
			sl.location.x--
		}
	} else {
		log.Fatalf("Unexpected jet input '%c'", jet)
	}

	// handle falling
	sl.location.y--
	if sl.shape.miny+sl.location.y < 0 || intersectsAny(stoppedRocksPtr, sl) {
		sl.location.y++
		return true, sl
	} else {
		return false, sl
	}
}

func fallSimple(jet byte, sl ShapeAndLoc) ShapeAndLoc {
	newLocation := sl.location
	if jet == '<' {
		newLocation.x--
		if sl.shape.minx+newLocation.x < 0 {
			newLocation = sl.location
		}
	} else if jet == '>' {
		newLocation.x++
		if sl.shape.maxx+newLocation.x >= 7 {
			newLocation = sl.location
		}
	} else {
		log.Fatalf("Unexpected jet input '%c'", jet)
	}

	// handle falling
	newLocation.y--
	return ShapeAndLoc{shape: sl.shape, location: newLocation}
}

func printcave(stoppedRocksPtr *[]ShapeAndLoc, sl ShapeAndLoc, lowest int) {
	stoppedRocks := *stoppedRocksPtr

	starty := 0
	for _, p := range sl.shape.points {
		if p.y+sl.location.y > starty {
			starty = p.y + sl.location.y
		}
	}

	rocks := map[Point]bool{}
	for _, r := range stoppedRocks {
		for _, p := range r.shape.points {
			rocks[Point{x: p.x + r.location.x, y: p.y + r.location.y}] = true
		}
	}

	for y := starty; y >= lowest; y-- {
		fmt.Print("|")
		for x := 0; x < 7; x++ {
			inShape := false
			for _, ps := range sl.shape.points {
				if x == ps.x+sl.location.x && y == ps.y+sl.location.y {
					inShape = true
					break
				}
			}
			if inShape {
				fmt.Print("@")
			} else {
				p := Point{x, y}
				if _, ok := rocks[p]; ok {
					fmt.Print("#")
				} else {
					fmt.Print(".")
				}
			}
		}
		fmt.Println("|")
	}

	fmt.Println("+-------+")
	fmt.Println()
}

func increment(i int, limit int) int {
	i++
	if i == limit {
		i = 0
	}
	return i
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
