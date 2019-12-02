package main

// FuelForModule returns the fuel required for a module of a given mass.
func FuelForModule(mass int) int {
	return mass / 3 - 2
}
