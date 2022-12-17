package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
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
	}

	// Make the algorithm a bit more greedy by listing the
	// destinations with the highest flowRate first.
	for _, v := range valves {
		sort.Slice(v.destinations, func(i, j int) bool {
			return valves[v.destinations[i]].flowRate > valves[v.destinations[j]].flowRate
		})
		v.print()
	}

	openValveMask := make(map[string]uint16)
	bitPosition := 0
	for k, v := range valves {
		if v.flowRate > 0 {
			openValveMask[k] = 1 << bitPosition
			bitPosition++
		}
	}

	//pressure, path := part1(valves, openValveMask, 30)
	pressure := part2(valves, openValveMask, 26)
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

type State struct {
	openState   uint16
	position    string
	minute      int
	currentFlow int
	totalFlow   int
	path        string
}

func count2(m map[[2]string]map[uint16]int) int {
	res := 0
	for _, v := range m {
		res += len(v)
	}
	return res
}

func count1(m map[string]map[uint16]int) int {
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
			fmt.Printf("Processing minute %d, with %d items, seen %d, currentVal: %d\n", currentState.minute, searchSpaceSize, count1(seen), currentState.totalFlow)
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

type State2 struct {
	openState   uint16
	positions   [2]string
	minute      int
	currentFlow int
	totalFlow   int
}

func part2(valves map[string]Valve, openValveMask map[string]uint16, limit int) int {
	largestFlow := math.MinInt

	var initialOpens uint16
	for i := len(openValveMask); i < 16; i++ {
		initialOpens = initialOpens | (1 << i)
	}

	searchSpace := []State2{{positions: [2]string{"AA", "AA"}, minute: 0, openState: initialOpens}}

	seen := make(map[[2]string]map[uint16]int)
	prevMinute := 0
	t0 := time.Now()

nextState:
	for {
		searchSpaceSize := len(searchSpace)
		if searchSpaceSize == 0 {
			break
		}

		currentState := searchSpace[0]
		searchSpace = searchSpace[1:]

		if currentState.minute != prevMinute {
			fmt.Printf("Elapsed: %v, Processing minute %d, with %d items, seen %d, currentVal: %d\n", time.Since(t0), currentState.minute, searchSpaceSize, count2(seen), currentState.totalFlow)
			prevMinute = currentState.minute
		}

		if currentState.minute >= limit {
			if currentState.totalFlow > largestFlow {
				largestFlow = currentState.totalFlow
			}
			continue nextState
		}

		if opens, ok := seen[currentState.positions]; !ok {
			seen[currentState.positions] = make(map[uint16]int)
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
		seen[currentState.positions][currentState.openState] = currentState.totalFlow

		candidateMoves := [2][]Move{}
		for i := 0; i < 2; i++ {
			candidateMoves[i] = moves(currentState.positions[i], currentState.openState, valves, openValveMask)
		}

		for _, you := range candidateMoves[0] {
			for _, elephant := range candidateMoves[1] {
				// We can't both open the save valve
				if you.newOpen != "" && you.newOpen == elephant.newOpen {
					continue
				}

				openState := currentState.openState
				if you.newOpen != "" {
					openState |= openValveMask[you.newOpen]
				}
				if elephant.newOpen != "" {
					openState |= openValveMask[elephant.newOpen]
				}

				// Sort positions so that they appear consistently in the "seen" table.
				var positions [2]string
				if you.position <= elephant.position {
					positions = [2]string{you.position, elephant.position}
				} else {
					positions = [2]string{elephant.position, you.position}
				}

				searchSpace = append(searchSpace, State2{
					positions:   positions,
					minute:      currentState.minute + 1,
					currentFlow: currentState.currentFlow + you.addFlow + elephant.addFlow,
					totalFlow:   currentState.currentFlow + currentState.totalFlow,
					openState:   openState,
				})
			}
		}
	}

	return largestFlow
}

type Move struct {
	position string
	newOpen  string
	addFlow  int
}

func moves(position string, openState uint16, valves map[string]Valve, openValveMask map[string]uint16) []Move {
	res := []Move{}
	allTrue := uint16(math.MaxUint16)

	if openState == allTrue {
		res = append(res, Move{position: position})
	} else {
		open := openState&openValveMask[position] > 0
		if !open && valves[position].flowRate > 0 {
			res = append(res, Move{position: position, newOpen: position, addFlow: valves[position].flowRate})
		}
		for _, v := range valves[position].destinations {
			res = append(res, Move{position: v})
		}
	}
	return res
}
