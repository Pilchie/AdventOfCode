package main

// FuelForModule returns the fuel required for a module of a given mass.
func FuelForModule(mass int) int {
	fuel := mass/3 - 2
	return fuel + fuelForMassRec(fuel)
}

func fuelForMassRec(mass int) int {
	fuel := mass/3 - 2
	if fuel > 0 {
		fuel += fuelForMassRec(fuel)
	} else {
		fuel = 0
	}
	return fuel
}
