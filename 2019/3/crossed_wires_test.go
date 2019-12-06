package main

import "testing"

func Test1(t *testing.T) {
	verify(t,
		[]string{"R8", "U5", "L5", "D3"},
		[]string{"U7", "R6", "D4", "L4"},
		6)
}

func Test2(t *testing.T) {
	verify(t,
		[]string{"R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"},
		[]string{"U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"},
		159)
}

func Test3(t *testing.T) {
	verify(t,
		[]string{"R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"},
		[]string{"U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"},
		135)
}

func verify(t *testing.T, firstWire []string, secondWire []string, expected int) {
	actual := CalculateDistance(firstWire, secondWire)
	if actual != expected {
		t.Fatalf("Expected: '%d', Actual: '%d'", expected, actual)
	}
}
