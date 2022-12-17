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

	openValveMask := make(map[string]uint16)
	bitPosition := 0
	for k, v := range valves {
		if v.flowRate > 0 {
			fmt.Printf("Setting mask for %s to %04x\n", k, 1<<bitPosition)
			openValveMask[k] = 1 << bitPosition
			bitPosition++
		}
	}

	pressure, path := part1(valves, openValveMask, 30)
	fmt.Printf("Released %d pressure via %s\n", pressure, path)

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

type State struct {
	openState   uint16
	position    string
	minute      int
	currentFlow int
	totalFlow   int
	path        string
}

func count(m map[string]map[uint16]int) int {
	res := 0
	for _, v := range m {
		res += len(v)
	}
	return res
}

func part1(valves map[string]Valve, openValveMask map[string]uint16, limit int) (int, string) {
	largestFlow := math.MinInt
	largestPath := ""

	allTrue := uint16(0xFFFF)
	var initialOpens uint16
	for i := len(openValveMask); i < 16; i++ {
		initialOpens = initialOpens | (1 << i)
	}
	fmt.Printf("Initial Opens is %x\n", initialOpens)

	searchSpace := []State{{position: "AA", minute: 0, openState: initialOpens, path: "AA"}}

	seen := make(map[string]map[uint16]int)
	prevMinute := 0

nextState:
	for {
		searchSpaceSize := len(searchSpace)
		if searchSpaceSize == 0 {
			break
		}

		currentState := searchSpace[0]
		searchSpace = searchSpace[1:]

		if currentState.minute != prevMinute {
			fmt.Printf("Processing minute %d, with %d items, seen %d, currentVal: %d\n", currentState.minute, searchSpaceSize, count(seen), currentState.totalFlow)
			prevMinute = currentState.minute
		}

		if currentState.minute >= limit {
			if currentState.totalFlow > largestFlow {
				largestFlow = currentState.totalFlow
				largestPath = currentState.path
			}
			continue nextState
		}

		if opens, ok := seen[currentState.position]; !ok {
			seen[currentState.position] = make(map[uint16]int)
		} else {
			for key, val := range opens {
				if val > currentState.totalFlow {
					// If we've seen the same *or more* valves open, we don't need to continue down this path
					if key&currentState.openState == currentState.openState {
						continue nextState
					}
				}
			}
		}
		seen[currentState.position][currentState.openState] = currentState.totalFlow

		if currentState.openState == allTrue {
			// Don't need to move, everything is open
			searchSpace = append(searchSpace, State{
				position:    currentState.position,
				openState:   currentState.openState,
				minute:      currentState.minute + 1,
				currentFlow: currentState.currentFlow,
				totalFlow:   currentState.totalFlow + currentState.currentFlow,
				path:        currentState.path + "->" + currentState.position,
			})
		} else {
			// If we're at a closed valve, we could open it.
			open := currentState.openState&openValveMask[currentState.position] != 0
			if !open && valves[currentState.position].flowRate > 0 {
				openState := currentState.openState
				openState = openState | openValveMask[currentState.position]
				searchSpace = append(searchSpace, State{
					position:    currentState.position,
					openState:   openState,
					minute:      currentState.minute + 1,
					currentFlow: currentState.currentFlow + valves[currentState.position].flowRate,
					totalFlow:   currentState.totalFlow + currentState.currentFlow,
					path:        currentState.path + "-o",
				})
			}

			for _, v := range valves[currentState.position].destinations {
				searchSpace = append(searchSpace, State{
					position:    v,
					openState:   currentState.openState,
					minute:      currentState.minute + 1,
					currentFlow: currentState.currentFlow,
					totalFlow:   currentState.totalFlow + currentState.currentFlow,
					path:        currentState.path + "->" + v,
				})
			}
		}
	}

	return largestFlow, largestPath
}
