package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	valves := map[string]Valve{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		valve := parseValve(line)
		valves[valve.name] = valve

		valve.print()
	}

	pressure := 0
	open := map[string]bool{}
	position := "AA"
	for step := 0; step < 30; step++ {
		fmt.Printf("Step %2d - ", step)
		stepPressure := minute(open, valves)
		if len(open) < len(valves) {
			v := valves[position]
			if v.flowRate == 0 {
				// Mark that we've visited this one so we don't come back
				open[position] = true
			}

			if _, found := open[position]; found {
				// It's open or jammed, pick the tunnel out of it that leads to the
				// valve with the highest flow rate that is still closed

				max := math.MinInt
				maxname := ""
				for di := range v.destinations {
					dname := v.destinations[di]
					dvalve := valves[dname]
					if _, dfound := open[dname]; !dfound {
						if dvalve.flowRate > max {
							max = dvalve.flowRate
							maxname = dvalve.name
						}
					}
				}
				fmt.Printf("Moving to '%s'", maxname)
				position = maxname
			} else {
				// It's not open, open it
				open[position] = true
				fmt.Printf("Opening   '%s'", position)
			}
		}

		pressure += stepPressure
		fmt.Printf(" - released %2d pressure this step for a total of %4d\n", stepPressure, pressure)
	}

	fmt.Printf("Released %d pressure\n", pressure)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

type Valve struct {
	name         string
	flowRate     int
	destinations []string
}

func (v *Valve) print() {
	fmt.Printf("Valve, name: '%s', flow: '%d', tunnels: {", v.name, v.flowRate)
	for i := range v.destinations {
		fmt.Printf("'%s', ", v.destinations[i])
	}
	fmt.Println("}")
}

func parseValve(input string) Valve {
	name := input[6:8]
	parts := strings.Split(input, ";")
	flowRate, _ := strconv.Atoi(parts[0][23:])
	rawValves := strings.Split(parts[1][23:], ",")

	destinations := []string{}
	for i := range rawValves {
		destinations = append(destinations, strings.Trim(rawValves[i], " "))
	}

	return Valve{name, flowRate, destinations}
}

func minute(open map[string]bool, valves map[string]Valve) int {
	pressure := 0
	for o := range open {
		v := valves[o]
		pressure += v.flowRate
	}
	return pressure
}
