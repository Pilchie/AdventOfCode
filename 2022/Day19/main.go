package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
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

	bluePrints := []BluePrint{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		bluePrints = append(bluePrints, parseBlueBrint(scanner.Text()))
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	part1(bluePrints)
}

type BluePrint struct {
	id    byte
	costs [4][3]byte
}

func parseBlueBrint(input string) BluePrint {
	parts := strings.Split(input, " ")
	return BluePrint{
		id: parseByte(strings.TrimRight(parts[1], ":")),
		costs: [4][3]byte{
			{parseByte(parts[6]), 0, 0},
			{parseByte(parts[12]), 0, 0},
			{parseByte(parts[18]), parseByte(parts[21]), 0},
			{parseByte(parts[27]), 0, parseByte(parts[30])},
		},
	}
}

func parseByte(input string) byte {
	val, err := strconv.Atoi(input)
	if err != nil {
		log.Fatal(err.Error())
	}
	return byte(val)
}

func part1(bluePrints []BluePrint) {
	res := 0

	start := time.Now()
	for _, bp := range bluePrints {
		fmt.Printf("Starting BluePrint %d\n", bp.id)
		geodes := maxGeodes(bp)
		qualityLevel := int(bp.id) * int(geodes)
		res += qualityLevel
		fmt.Printf("Completed BluePrint %d - geodes: %d, qualityLevel: %d\n", bp.id, geodes, qualityLevel)
	}

	fmt.Printf("The sum of the quality levels is %d (took %v)\n", res, time.Since(start))
}

type State struct {
	minute    byte
	opened    byte
	materials [4]byte
	robots    [4]byte
	prev      *State
}

type Cache struct {
	materials [4]byte
	robots    [4]byte
}

func maxGeodes(bluePrint BluePrint) byte {
	best := State{minute: 1, opened: 0, robots: [4]byte{1, 0, 0, 0}}

	searchSpace := []State{best}
	seen := map[Cache]byte{}

	prevMinute := byte(0)
	for len(searchSpace) > 0 {
		current := searchSpace[0]
		searchSpace = searchSpace[1:]

		cacheEntry := Cache{current.materials, current.robots}
		if val, ok := seen[cacheEntry]; ok && val >= current.opened {
			continue
		}
		seen[cacheEntry] = current.opened

		if current.minute > 24 {
			if current.opened > best.opened {
				best = current
			}
			continue
		}

		if current.minute != prevMinute {
			prevMinute = current.minute

			current.printState()
			fmt.Printf("Remaining search space %d\n", len(searchSpace))
		}

		// Greedily try to build the most advanced robot first
		for costsIdx := len(bluePrint.costs) - 1; costsIdx >= 0; costsIdx-- {
			next := next(current)
			build := true
			for rc, cc := range bluePrint.costs[costsIdx] {
				if current.materials[rc] < cc {
					build = false
				} else {
					next.materials[rc] -= cc
				}
			}
			if build {
				next.robots[costsIdx]++
				searchSpace = append(searchSpace, next)
			}
		}

		// There is always an option where we don't build anything.
		searchSpace = append(searchSpace, next(current))
	}

	cur := &best
	for cur != nil {
		cur.printState()
		cur = cur.prev
	}
	return best.opened
}

func next(current State) State {
	next := current
	next.prev = &current
	next.minute++
	next.opened = current.opened + current.robots[3]
	for i := 0; i < len(current.robots); i++ {
		next.materials[i] += current.robots[i]
	}
	return next
}

func (current *State) printState() {
	fmt.Printf("  Minute: %d: Materials: ", current.minute)
	for _, m := range current.materials {
		fmt.Printf("%d, ", m)
	}
	fmt.Print("Robots: ")
	for _, r := range current.robots {
		fmt.Printf("%d, ", r)
	}
	fmt.Printf("Opened: %d\n", current.opened)
}
