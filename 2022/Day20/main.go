package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	numbers := []int{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		val, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatalf(err.Error())
		}
		numbers = append(numbers, val)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	part1(numbers)
	part2(numbers)
}

type CircularList struct {
	head *Node
}

type Node struct {
	val  int
	next *Node
	prev *Node
}

func (cl *CircularList) append(val int) *Node {
	if cl.head == nil {
		cl.head = &Node{
			val: val,
		}
		cl.head.next = cl.head
		cl.head.prev = cl.head
		return cl.head
	} else {
		end := cl.head.prev
		n := &Node{val: val,
			prev: end,
			next: cl.head}
		end.next = n
		cl.head.prev = n
		return n
	}
}

func (cl *CircularList) print() {
	cur := cl.head
	fmt.Print("[")
	for {
		fmt.Printf("%d, ", cur.val)

		cur = cur.next
		if cur == cl.head {
			break
		}
	}
	fmt.Print("]")
}

func part1(numbers []int) {
	fmt.Println("-----------------")
	fmt.Println("Starting Part One")
	res := CircularList{}
	nodes := []*Node{}
	for _, n := range numbers {
		nodes = append(nodes, res.append(n))
	}

	for i, changeby := range numbers {
		cur := nodes[i]
		if changeby > 0 {
			for i := 0; i < changeby; i++ {
				p := cur.prev
				n := cur.next
				nn := n.next
				p.next = n
				n.prev = p
				n.next = cur
				cur.prev = n
				cur.next = nn
				nn.prev = cur
			}
		} else {
			for i := 0; i > changeby; i-- {
				p := cur.prev
				pp := p.prev
				n := cur.next
				pp.next = cur
				cur.prev = pp
				cur.next = p
				p.prev = cur
				p.next = n
				n.prev = p
			}
		}
	}

	cur := res.head
	afterZero := 0
	started := false
	sum := 0
	for {
		if cur.val == 0 {
			started = true
		}

		if started {
			if afterZero%1000 == 0 {
				sum += cur.val
				fmt.Printf("Val %d is %d\n", afterZero, cur.val)
			}
			if afterZero == 3000 {
				break
			}
			afterZero++
		}

		cur = cur.next
	}
	fmt.Printf("The sum is %d\n", sum)
}

func part2(numbers []int) {
	fmt.Println("-----------------")
	fmt.Println("Starting Part Two")
	res := CircularList{}
	nodes := []*Node{}
	for i, n := range numbers {
		new := n * 811589153
		numbers[i] = new % (len(numbers) - 1)
		nodes = append(nodes, res.append(new))
	}

	fmt.Print("Initial ")
	res.print()
	fmt.Println()

	for mix := 0; mix < 10; mix++ {
		for i, changeby := range numbers {
			cur := nodes[i]
			if changeby > 0 {
				changeby = changeby % len(numbers)
				for i := 0; i < changeby; i++ {
					p := cur.prev
					n := cur.next
					nn := n.next
					p.next = n
					n.prev = p
					n.next = cur
					cur.prev = n
					cur.next = nn
					nn.prev = cur
				}
			} else {
				changeby = changeby % len(numbers)
				for i := 0; i > changeby; i-- {
					p := cur.prev
					pp := p.prev
					n := cur.next
					pp.next = cur
					cur.prev = pp
					cur.next = p
					p.prev = cur
					p.next = n
					n.prev = p
				}
			}
		}

		fmt.Printf("After mixing %d times: ", mix+1)
		//res.print()
		fmt.Println()
	}

	cur := res.head
	afterZero := 0
	started := false
	sum := 0
	for {
		if cur.val == 0 {
			started = true
		}

		if started {
			if afterZero%1000 == 0 {
				sum += cur.val
				fmt.Printf("Val %d is %d\n", afterZero, cur.val)
			}
			if afterZero == 3000 {
				break
			}
			afterZero++
		}

		cur = cur.next
	}
	fmt.Printf("The sum is %d\n", sum)
}
