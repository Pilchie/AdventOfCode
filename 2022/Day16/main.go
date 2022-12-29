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

	valveNameToId := map[string]uint8{}
	valveIdxToName := map[uint8]string{}
	valves := []Valve{}
	valveIdx := uint8(0)
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		valve := parseValve(line, valveIdx)
		valveNameToId[valve.name] = valveIdx
		valveIdxToName[valveIdx] = valve.name
		valves = append(valves, valve)
		valveIdx++
	}

	// Make the algorithm a bit more greedy by listing the
	// destinations with the highest flowRate first.
	for vi := 0; vi < len(valves); vi++ {
		for _, dn := range valves[vi].destinationNames {
			valves[vi].destinations = append(valves[vi].destinations, valveNameToId[dn])
		}
		sort.Slice(valves[vi].destinations, func(i, j int) bool {
			return valves[valves[vi].destinations[i]].flowRate > valves[valves[vi].destinations[j]].flowRate
		})
		valves[vi].print()
	}

	openValveMask := make(map[uint8]uint16)
	bitPosition := 0
	for _, v := range valves {
		if v.flowRate > 0 {
			openValveMask[uint8(v.id)] = 1 << bitPosition
			bitPosition++
		}
	}

	pressure, path := part1(valves, openValveMask, 30, valveIdxToName)
	fmt.Printf("Part 1: Released %d pressure, via path %s\n", pressure, path)
	pressure = part2(valves, openValveMask, 26)
	fmt.Printf("Part2: Released %d pressure\n", pressure)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

type Valve struct {
	id               uint8
	name             string
	flowRate         int
	destinations     []uint8
	destinationNames []string
}

func (v *Valve) print() {
	fmt.Printf("Valve, name: '%s', flow: '%d', tunnels: {", v.name, v.flowRate)
	for i := range v.destinations {
		fmt.Printf("'%s', ", v.destinationNames[i])
	}
	fmt.Println("}")
}

func parseValve(input string, id uint8) Valve {
	name := input[6:8]
	parts := strings.Split(input, ";")
	flowRate, _ := strconv.Atoi(parts[0][23:])
	rawValves := strings.Split(parts[1][23:], ",")

	destinationNames := []string{}
	for i := range rawValves {
		destinationNames = append(destinationNames, strings.TrimSpace(rawValves[i]))
	}

	return Valve{id: id, name: name, flowRate: flowRate, destinationNames: destinationNames}
}

type State struct {
	openState   uint16
	position    uint8
	minute      int
	currentFlow int
	totalFlow   int
	path        string
}

func count2(m map[uint16]map[uint16]int) int {
	res := 0
	for _, v := range m {
		res += len(v)
	}
	return res
}

func count1(m map[uint8]map[uint16]int) int {
	res := 0
	for _, v := range m {
		res += len(v)
	}
	return res
}

func part1(valves []Valve, openValveMask map[uint8]uint16, limit int, valveIdxToName map[uint8]string) (int, string) {
	largestFlow := math.MinInt
	largestPath := ""

	allTrue := uint16(0xFFFF)
	var initialOpens uint16
	for i := len(openValveMask); i < 16; i++ {
		initialOpens = initialOpens | (1 << i)
	}
	fmt.Printf("Initial Opens is %x\n", initialOpens)

	valveAA := uint8(0)
	for _, v := range valves {
		if v.name == "AA" {
			valveAA = v.id
			break
		}
	}
	searchSpace := []State{{position: valveAA, minute: 0, openState: initialOpens, path: "AA"}}

	seen := make(map[uint8]map[uint16]int)
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
				if val >= currentState.totalFlow {
					// If we've seen the same *or more* valves open, we don't need to continue down this path
					if key&currentState.openState == currentState.openState {
						continue nextState
					}
				}
			}
		}
		seen[currentState.position][currentState.openState] = currentState.totalFlow

		if currentState.openState == allTrue {
			valveName := valveIdxToName[currentState.position]

			// Don't need to move, everything is open
			searchSpace = append(searchSpace, State{
				position:    currentState.position,
				openState:   currentState.openState,
				minute:      currentState.minute + 1,
				currentFlow: currentState.currentFlow,
				totalFlow:   currentState.totalFlow + currentState.currentFlow,
				path:        currentState.path + "->" + valveName,
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
				valveName := valveIdxToName[v]
				searchSpace = append(searchSpace, State{
					position:    v,
					openState:   currentState.openState,
					minute:      currentState.minute + 1,
					currentFlow: currentState.currentFlow,
					totalFlow:   currentState.totalFlow + currentState.currentFlow,
					path:        currentState.path + "->" + valveName,
				})
			}
		}
	}

	return largestFlow, largestPath
}

type State2 struct {
	openState   uint16
	positions   uint16
	minute      uint8
	currentFlow int
	totalFlow   int
}

func part2(valves []Valve, openValveMask map[uint8]uint16, limit int) int {
	largestFlow := 0

	var initialOpens uint16
	for i := len(openValveMask); i < 16; i++ {
		initialOpens = initialOpens | (1 << i)
	}

	fmt.Printf("Initial opens are: %x\n", initialOpens)

	valveAA := uint16(0)
	for _, v := range valves {
		if v.name == "AA" {
			valveAA = uint16(v.id)
			break
		}
	}

	searchSpace := Queue{}
	searchSpace.Push(State2{positions: valveAA<<8 | valveAA, minute: 0, openState: initialOpens})

	seen := make(map[uint16]map[uint16]int)
	t0 := time.Now()
	tLast := t0

	dist := make([][]uint, len(valves))
	next := make([][]uint8, len(valves))
	for i := 0; i < len(valves); i++ {
		dist[i] = make([]uint, len(valves))
		next[i] = make([]uint8, len(valves))

		for j := 0; j < len(valves); j++ {
			dist[i][j] = 1000000
			next[i][j] = 0xFF
		}
	}

	FloydWarshallWithPathReconstruction(dist, next, valves)

	for _, v := range valves {
		if v.flowRate > 0 {
			path := shortestPath(uint8(valveAA), v.id, next)
			fmt.Printf("Shortest path from 'AA' to '%s' is %2d: %s", v.name, len(path), valves[path[0]].name)
			for _, p := range path[1:] {
				fmt.Printf("->%s", valves[p].name)
			}
			fmt.Println()
		}
	}

	lastLen := 0
	lastProcessed := 0
	completed := 0
	skipped := 0

nextState:
	for !searchSpace.IsEmpty() {
		currentState := searchSpace.Pop()

		if time.Since(tLast).Milliseconds() >= 5000 {
			len := searchSpace.Len()
			fmt.Printf("Processing minute %2d, open: %x, best: %4d, seen %6d, completed: %10d, skipped: %10d, remaining %10d (delta: %10d), processed: %10d (delta: %10d). Elapsed time: %v\n",
				currentState.minute, currentState.openState, largestFlow, count2(seen), completed, skipped, len, len-lastLen, searchSpace.processed, searchSpace.processed-lastProcessed, time.Since(t0))
			tLast = time.Now()
			lastLen = len
			lastProcessed = searchSpace.processed
		}

		if currentState.totalFlow > largestFlow {
			largestFlow = currentState.totalFlow
		}

		if int(currentState.minute) >= limit {
			completed++
			continue nextState
		}

		if opens, ok := seen[currentState.positions]; !ok {
			seen[currentState.positions] = make(map[uint16]int)
		} else {
			for key, flow := range opens {
				if flow >= currentState.totalFlow {
					// If we've seen the same *or more* valves open, we don't need to continue down this path
					if key&currentState.openState == currentState.openState {
						skipped++
						continue nextState
					}
				}
			}
		}
		seen[currentState.positions][currentState.openState] = currentState.totalFlow

		if currentState.openState == math.MaxUint16 {
			searchSpace.Push(State2{
				positions:   currentState.positions,
				minute:      uint8(limit),
				currentFlow: currentState.currentFlow,
				totalFlow:   currentState.totalFlow + currentState.currentFlow*(limit-int(currentState.minute)),
				openState:   currentState.openState,
			})
		} else {
			candidateMoves := [2][]Move{}
			myPos := uint8(currentState.positions >> 8)
			elPos := uint8(currentState.positions & 0x00FF)
			candidateMoves[0] = moves(myPos, currentState.openState, valves, openValveMask, limit-int(currentState.minute), next, false)
			candidateMoves[1] = moves(elPos, currentState.openState, valves, openValveMask, limit-int(currentState.minute), next, myPos == elPos)

			for _, you := range candidateMoves[0] {
				for _, elephant := range candidateMoves[1] {
					// We can't both open the save valve
					if you.newOpen != 0xFF && you.newOpen == elephant.newOpen {
						continue
					}

					openState := currentState.openState
					if you.newOpen != 0xFF {
						openState |= openValveMask[you.newOpen]
					}
					if elephant.newOpen != 0xFF {
						openState |= openValveMask[elephant.newOpen]
					}

					// Sort positions so that they appear consistently in the "seen" table.
					var positions uint16
					if you.position <= elephant.position {
						positions = (uint16(you.position) << 8) + uint16(elephant.position)
					} else {
						positions = (uint16(elephant.position) << 8) + uint16(you.position)
					}

					searchSpace.Push(State2{
						positions:   positions,
						minute:      currentState.minute + 1,
						currentFlow: currentState.currentFlow + you.addFlow + elephant.addFlow,
						totalFlow:   currentState.currentFlow + currentState.totalFlow,
						openState:   openState,
					})
				}
			}
		}
	}

	return largestFlow
}

type Move struct {
	position uint8
	newOpen  uint8
	addFlow  int
}

func moves(position uint8, openState uint16, valves []Valve, openValveMask map[uint8]uint16, remaining int, next [][]uint8, skipTop bool) []Move {
	res := []Move{}

	open := openState&openValveMask[position] > 0
	if !open && valves[position].flowRate > 0 {
		res = append(res, Move{position: position, newOpen: position, addFlow: valves[position].flowRate})
	}

	// paths := []Path{}

	// // Calculate the expected value of all closed valves, and go to the highest one.
	// for _, valve := range valves {
	// 	if openState&openValveMask[uint8(valve.id)] == 0 && valve.flowRate > 0 {
	// 		path := shortestPath(position, valve.id, next)
	// 		expectedValue := (remaining - len(path)) * valve.flowRate
	// 		if expectedValue > 0 && len(path) > 1 {
	// 			paths = append(paths, Path{expected: expectedValue, steps: path})
	// 		}
	// 	}
	// }

	// if len(paths) > 0 {
	// 	sort.Slice(paths, func(i, j int) bool {
	// 		// We want to sort from largest expected value to smallest
	// 		return paths[i].expected > paths[j].expected
	// 	})
	// 	added := map[uint8]bool{}
	// 	if skipTop {
	// 		paths = paths[1:]
	// 	}

	// 	for _, p := range paths {
	// 		if _, ok := added[p.steps[1]]; !ok {
	// 			res = append(res, Move{position: p.steps[1], newOpen: 0xFF})
	// 			added[p.steps[0]] = true
	// 		}
	// 	}
	// }
	for _, v := range valves[position].destinations {
		res = append(res, Move{position: v, newOpen: 0xFF})
	}
	return res
}

type Edge struct {
	from, to uint8
}

func edges(valves []Valve) []Edge {
	res := []Edge{}
	for _, v := range valves {
		for _, d := range v.destinations {
			res = append(res, Edge{from: v.id, to: d})
		}
	}
	return res
}

func FloydWarshallWithPathReconstruction(dist [][]uint, next [][]uint8, valves []Valve) {

	for _, e := range edges(valves) {
		dist[e.from][e.to] = 1 // The weight of the edge (u, v)
		next[e.from][e.to] = e.to
	}
	for v := range valves {
		dist[v][v] = 0
		next[v][v] = uint8(v)
	}
	for k := 0; k < len(valves); k++ { // standard Floyd-Warshall implementation
		for i := 0; i < len(valves); i++ {
			for j := 0; j < len(valves); j++ {
				if dist[i][j] > dist[i][k]+dist[k][j] {
					dist[i][j] = dist[i][k] + dist[k][j]
					next[i][j] = next[i][k]
				}
			}
		}
	}
}

func shortestPath(u, v uint8, next [][]uint8) []uint8 {
	if next[u][v] == 0xFF {
		return []uint8{}
	}
	path := []uint8{u}
	for u != v {
		u = next[u][v]
		path = append(path, u)
	}
	return path
}

type Path struct {
	expected int
	steps    []uint8
}

type Node struct {
	elements [1000000]State2
	next     *Node

	start int
	end   int
}

type Queue struct {
	head *Node
	tail *Node

	processed int
}

func (q *Queue) Push(s State2) {
	if q.head == nil {
		q.head = &Node{}
		q.tail = q.head
		q.head.elements[0] = s
		q.head.end = 1
	} else if q.tail.end == len(q.tail.elements) {
		n := Node{
			start: 0,
			end:   1,
		}
		n.elements[0] = s
		q.tail.next = &n
		q.tail = &n
	} else {
		q.tail.elements[q.tail.end] = s
		q.tail.end++
	}

	q.processed++
}

func (q *Queue) Pop() State2 {
	n := q.head.elements[q.head.start]
	q.head.start++
	if q.head.start == q.head.end {
		q.head = q.head.next
		if q.head == nil {
			q.tail = nil
		}
	}
	return n
}

func (q *Queue) IsEmpty() bool {
	return q.head == nil
}

func (q *Queue) Len() int {
	len := 0
	for c := q.head; c != nil; c = c.next {
		len += c.end - c.start
	}
	return len
}
