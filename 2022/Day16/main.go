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

	open := map[string]bool{}
	position := "AA"
	step := 0
	limit := 30
	pressure := step_recursive(position, valves, open, step, limit, "AA")
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
		destinations = append(destinations, strings.TrimSpace(rawValves[i]))
	}

	return Valve{name, flowRate, destinations}
}

func current_flow(open map[string]bool, valves map[string]Valve) int {
	pressure := 0
	for o := range open {
		pressure += valves[o].flowRate
	}
	return pressure
}

func step_recursive(position string, valves map[string]Valve, open map[string]bool, step int, limit int, path string) int {
	step++
	if step > limit {
		return 0
	}

	current := current_flow(open, valves)

	// All valves are open, do nothing
	if len(open) == len(valves) {
		return current + step_recursive(position, valves, open, step, limit, path+position)
	}

	// Open the current valve
	valve := valves[position]
	max_children := math.MinInt
	if valve.flowRate > 0 {
		if v, f := open[position]; !f || !v {
			// clone open, add current
			cloned := clone(open)
			cloned[position] = true
			child := step_recursive(position, valves, cloned, step, limit, path+"o")
			if child > max_children {
				max_children = child
			}
		}
	} else {
		// Mark this open, so we don't look at it again
		open[position] = true
	}

	// or, travel through one of the tunnels
	for d := range valve.destinations {
		child := step_recursive(valve.destinations[d], valves, open, step, limit, path+valve.destinations[d])
		if child > max_children {
			max_children = child
		}
	}

	res := current + max_children
	if step < 15 {
		fmt.Printf("%s - %d\n", path, res)
	}
	return res
}

func clone(m map[string]bool) map[string]bool {
	res := map[string]bool{}
	for k := range m {
		res[k] = m[k]
	}
	return res
}
