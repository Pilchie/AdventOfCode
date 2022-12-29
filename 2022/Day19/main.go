package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
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
	id    uint8
	costs [4][3]uint8
	maxes [4]uint8
}

func (bp *BluePrint) maxResourceCost(resourceIndex int) uint8 {
	return bp.maxes[resourceIndex]
}

func parseBlueBrint(input string) BluePrint {
	parts := strings.Split(input, " ")
	bp := BluePrint{
		id: parseUInt8(strings.TrimRight(parts[1], ":")),
		costs: [4][3]byte{
			{parseUInt8(parts[6]), 0, 0},
			{parseUInt8(parts[12]), 0, 0},
			{parseUInt8(parts[18]), parseUInt8(parts[21]), 0},
			{parseUInt8(parts[27]), 0, parseUInt8(parts[30])},
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

func parseUInt8(input string) uint8 {
	val, err := strconv.Atoi(input)
	if err != nil {
		log.Fatal(err.Error())
	}
	return uint8(val)
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
	minute    uint8
	opened    uint8
	materials [4]int16
	robots    [4]uint8
	prev      *State
}

type CacheKey struct {
	materials uint64
	robots    uint32
}

func constructCacheKey(materials [4]int16, robots [4]uint8) CacheKey {
	mkey := uint64(0)
	for i, m := range materials {
		mkey |= uint64(m) << (i * 16)
	}
	rkey := uint32(0)
	for i, r := range robots {
		rkey |= uint32(r) << (i * 8)
	}
	return CacheKey{materials: mkey, robots: rkey}
}

func maxGeodes(bluePrint BluePrint, limit uint8) uint8 {
	best := State{minute: 1, opened: 0, robots: [4]uint8{1, 0, 0, 0}}

	searchSpace := Queue{}
	searchSpace.push(best)
	seen := map[CacheKey][]uint8{}
	skipped := 0
	t0 := time.Now()
	tLast := time.Now()
	lastProcessed := int64(0)
	lastLength := 0
	lastSkipped := 0
	lastSeen := 0

nextState:
	for !searchSpace.isEmpty() {
		current := searchSpace.pop()

		cacheEntry := constructCacheKey(current.materials, current.robots)
		if val, ok := seen[cacheEntry]; ok {
			// Note this is < because we are storing items in the value before them, since our minute is 1-based.
			for m := uint8(0); m < current.minute; m++ {
				if val[m] >= current.opened {
					skipped++
					continue nextState
				}
			}
		} else {
			seen[cacheEntry] = make([]uint8, limit+1)
		}
		seen[cacheEntry][current.minute-1] = current.opened

		if current.opened > best.opened {
			best = current
		}

		if time.Since(tLast).Milliseconds() > 5000 {
			length := searchSpace.length()
			fmt.Printf("Best %3d - space %9d(%9d), seen %9d(%9d), skipped %9d(%9d), processed %12d(%9d) - Current: ", best.opened, length, length-lastLength, len(seen), len(seen)-lastSeen, skipped, skipped-lastSkipped, searchSpace.processed, searchSpace.processed-lastProcessed)
			current.printState()
			fmt.Println(", elapsed: ", time.Since(t0))
			tLast = time.Now()
			lastProcessed = searchSpace.processed
			lastLength = length
			lastSkipped = skipped
			lastSeen = len(seen)
		}

		if current.minute > limit {
			continue
		}

		// If we're in the last minute, there is no point building another robot, since it won't have time do anything.
		if current.minute <= limit {
			// Pick which robot to build next
			for robotIdx := len(bluePrint.costs) - 1; robotIdx >= 0; robotIdx-- {
				if robotIdx < len(bluePrint.costs)-1 {
					robots := current.robots[robotIdx]
					timeLeft := limit - current.minute
					materials := current.materials[robotIdx]
					// If we already have enough of robotIdx to build any other robot every minute, don't bother building more.
					if int16(robots*timeLeft)+materials >= int16(bluePrint.maxResourceCost(robotIdx)*timeLeft) {
						continue
					}
				}
				canBuild := true
				timeToBuild := uint8(0)
				robotCosts := bluePrint.costs[robotIdx]
				for costIdx, cost := range robotCosts {
					if cost != 0 && current.robots[costIdx] == 0 {
						canBuild = false
						break
					} else {
						if int16(cost) > current.materials[costIdx] {
							resourcesNeeded := int16(cost) - current.materials[costIdx]
							timeResource := uint8(math.Ceil(float64(resourcesNeeded) / float64(current.robots[costIdx])))
							if timeResource > timeToBuild {
								timeToBuild = timeResource
							}
						}
					}
				}

				if canBuild && current.minute+timeToBuild < limit {
					n := next(current, timeToBuild+1)
					n.robots[robotIdx]++
					for costIdx, cost := range robotCosts {
						n.materials[costIdx] -= int16(cost)
						if n.materials[costIdx] < 0 {
							fmt.Println("Underflowed resources")
						}
					}
					searchSpace.push(n)
				}
			}
		}

		// There is always an option where we don't build anything.
		searchSpace.push(next(current, 1))
	}

	cur := &best
	fmt.Println()
	fmt.Printf("Done - Processed %v states. Best path was:\n", searchSpace.processed)
	fmt.Println("---------------------")
	for cur != nil {
		cur.printState()
		fmt.Println()
		cur = cur.prev
	}
	fmt.Println("---------------------")
	return best.opened
}

func next(current State, minutes uint8) State {
	next := current
	//next.prev = &current
	next.minute += minutes
	next.opened = current.opened + minutes*current.robots[3]
	for i := 0; i < len(current.robots); i++ {
		next.materials[i] += int16(minutes * current.robots[i])
	}
	return next
}

func (current *State) printState() {
	fmt.Printf("Minute: %2d, Opened: %2d, Materials: ", current.minute, current.opened)
	for _, m := range current.materials {
		fmt.Printf("%2d, ", m)
	}
	fmt.Print("Robots: ")
	for _, r := range current.robots {
		fmt.Printf("%2d, ", r)
	}

}

type Node struct {
	elements [100000]State
	next     *Node

	start int
	end   int
}
type Queue struct {
	head *Node
	tail *Node

	processed int64
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
		n.elements[0] = s
		q.tail.next = &n
		q.tail = &n
	} else {
		q.tail.elements[q.tail.end] = s
		q.tail.end++
	}

	q.processed++
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
