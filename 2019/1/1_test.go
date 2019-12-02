package main

import (
	"testing")

func TestMassOf12(t *testing.T) {
	testMass(t, 12, 2)
}

func TestMassOf14(t *testing.T) {
	testMass(t, 14, 2)
}

func TestMassOf1969(t *testing.T) {
	testMass(t, 1969, 654)
}

func TestMassOf100756(t *testing.T) {
	testMass(t, 100756, 33583)
}

func testMass(t *testing.T, mass int, expected int) {
	actual := FuelForModule(mass)
	if actual != expected {
		t.Fatalf("Expected: '%d', Actual: '%d'", expected, actual)
	}
}
