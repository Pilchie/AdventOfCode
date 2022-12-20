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
	part2(bluePrints)
}

type BluePrint struct {
	id    byte
	costs [4][3]byte
	maxes [4]byte
}

func (bp *BluePrint) maxResourceCost(resourceIndex int) byte {
	return bp.maxes[resourceIndex]
}

func parseBlueBrint(input string) BluePrint {
	parts := strings.Split(input, " ")
	bp := BluePrint{
		id: parseByte(strings.TrimRight(parts[1], ":")),
		costs: [4][3]byte{
			{parseByte(parts[6]), 0, 0},
			{parseByte(parts[12]), 0, 0},
			{parseByte(parts[18]), parseByte(parts[21]), 0},
			{parseByte(parts[27]), 0, parseByte(parts[30])},
		},
	}

	for i := 0; i < len(bp.costs[0]); i++ {

		max := bp.costs[0][i]
		for _, c := range bp.costs[1:] {
			if c[i] > max {
				max = c[i]
			}
		}
		bp.maxes[i] = max
	}
	return bp
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
		geodes := maxGeodes(bp, 24)
		qualityLevel := int(bp.id) * int(geodes)
		res += qualityLevel
		fmt.Printf("Completed BluePrint %d - geodes: %d, qualityLevel: %d\n", bp.id, geodes, qualityLevel)
	}

	fmt.Printf("The sum of the quality levels is %d (took %v)\n", res, time.Since(start))
}

func part2(bluePrints []BluePrint) {
	res := 1

	start := time.Now()
	for i, bp := range bluePrints {
		if i > 2 {
			break
		}

		fmt.Printf("Starting BluePrint %d\n", bp.id)
		geodes := maxGeodes(bp, 32)
		res *= int(geodes)
		fmt.Printf("Completed BluePrint %d - geodes: %d\n", bp.id, geodes)
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

func maxGeodes(bluePrint BluePrint, limit byte) byte {
	best := State{minute: 1, opened: 0, robots: [4]byte{1, 0, 0, 0}}

	searchSpace := Queue{}
	searchSpace.push(best)
	seen := map[Cache]byte{}

	prevMinute := byte(0)
	for !searchSpace.isEmpty() {
		current := searchSpace.pop()

		cacheEntry := Cache{current.materials, current.robots}
		if val, ok := seen[cacheEntry]; ok && val >= current.opened {
			continue
		}
		seen[cacheEntry] = current.opened

		if current.minute > limit {
			if current.opened > best.opened {
				best = current
			}
			continue
		}

		if current.minute != prevMinute {
			prevMinute = current.minute

			current.printState()
			fmt.Printf(" - Remaining search space %d\n", searchSpace.length())
		}

		// If we're in the last minute, there is no point building another robot, since it won't have time do anything.
		if current.minute != limit {
			// Greedily try to build the most advanced robot first. This means we're more likely
			// to find a better match in our memoization table.
			for robotToBuild := 0; robotToBuild < len(bluePrint.costs); robotToBuild++ {
				// If we already produce enough of the resource robotToBuild produces to build any robot every minute,
				// don't build it.
				if robotToBuild != len(bluePrint.costs)-1 {
					//timeLeft := limit - current.minute
					xRobots := current.robots[robotToBuild]
					//yStock := current.materials[robotToBuild]
					zMaxCost := bluePrint.maxResourceCost(robotToBuild)
					if xRobots >= zMaxCost {
						continue
					}
				}

				build := true
				next := next(current)
				for resourceIndex, resourceCost := range bluePrint.costs[robotToBuild] {
					if current.materials[resourceIndex] < resourceCost {
						build = false
					} else {
						next.materials[resourceIndex] -= resourceCost
					}
				}
				if build {
					next.robots[robotToBuild]++
					searchSpace.push(next)
				}
			}
		}

		// There is always an option where we don't build anything.
		searchSpace.push(next(current, 1))
	}

	cur := &best
	fmt.Println()
	fmt.Println("Done - best path was:")
	fmt.Println("---------------------")
	for cur != nil {
		cur.printState()
		fmt.Println()
		cur = cur.prev
	}
	fmt.Println("---------------------")
	return best.opened
}

func next(current State, minutes int) State {
	next := current
	next.prev = &current
	next.minute += byte(minutes)
	next.opened = current.opened + byte(minutes)*current.robots[3]
	for i := 0; i < len(current.robots); i++ {
		next.materials[i] += byte(minutes) * current.robots[i]
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
	fmt.Printf("Opened: %d", current.opened)
}

type Node struct {
	elements [10000]State
	next     *Node

	start int
	end   int
}
type Queue struct {
	head *Node
	tail *Node
}

func (q *Queue) push(s State) {
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
		q.tail.next = &n
		q.tail = &n
	} else {
		q.tail.elements[q.tail.end] = s
		q.tail.end++
	}
}

func (q *Queue) pop() State {
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

func (q *Queue) isEmpty() bool {
	return q.head == nil
}

func (q *Queue) length() int {
	len := 0
	for c := q.head; c != nil; c = c.next {
		len += c.end - c.start
	}
	return len
}
