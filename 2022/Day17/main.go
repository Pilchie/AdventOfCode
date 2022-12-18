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

	targetCount := 1000000000000

	repeat := 5 * len(jets)
	remainder := targetCount % repeat
	repetitions := targetCount / repeat
	base := dropUntil(repeat, jets)
	rest := dropUntil(remainder, jets)
	fmt.Printf("Highest rock is at %d (base: %d, repetitions: %d, rest: %d)\n", base*repetitions+rest, base, repetitions, rest)

	// res := dropUntil(targetCount, jets)
	// fmt.Printf("Highest rock is at %d\n", res)

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

	t0 := time.Now()
	for countStoppedRocks < steps {
		done, newShapeAndLocation := fall(&stoppedRocks, jets[jetIdx], currentShapeAndLocation)
		if done {
			countStoppedRocks++
			stoppedRocks = append(stoppedRocks, newShapeAndLocation)
			length := len(stoppedRocks)
			if length > 1000 {
				stoppedRocks = stoppedRocks[length-100:]
			}
			highestSoFar = highest(&stoppedRocks)
			if countStoppedRocks%10000 == 0 {
				//printcave(stoppedRocks, newShapeAndLocation, highestSoFar-100)
				fmt.Printf("Done %d rocks in %v\n", countStoppedRocks, time.Since(t0))
			}
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
	return res
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
