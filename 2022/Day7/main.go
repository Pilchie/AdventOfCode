package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Node struct {
	parent   *Node
	children []*Node
	name     string
	size     int
}

func (n *Node) SizeRecursive() int {
	size := n.size
	for i := range n.children {
		size += n.children[i].SizeRecursive()
	}
	return size
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	root := Node{}
	cur := &root
	done := !scanner.Scan()

	for !done {
		line := scanner.Text()
		if line == "" {
			break
		}
		fmt.Printf("%v\n", line)
		if line == "$ cd /" {
			cur = &root
		} else if line == "$ cd .." {
			cur = cur.parent
		} else if strings.HasPrefix(line, "$ cd ") {
			name := strings.TrimPrefix(line, "$ cd ")
			for c := range cur.children {
				if name == cur.children[c].name {
					cur = cur.children[c]
					break
				}
			}
		} else if line == "$ ls" {
			for scanner.Scan() && scanner.Text()[0] != '$' {
				line = scanner.Text()
				parts := strings.Split(line, " ")
				size := 0
				if parts[0] == "dir" {
					size = 0
				} else {
					size, _ = strconv.Atoi(parts[0])
				}
				cur.children = append(cur.children, &Node{size: size, name: parts[1], parent: cur})
			}
			continue
		} else {
			log.Fatalf("Unexpected command: %v\n", line)
		}
		done = !scanner.Scan()
	}

	total := 70000000
	need := 30000000
	avail := total - root.SizeRecursive()
	to_free := need - avail

	d, s := find_smallest_bigger(&root, to_free)
	fmt.Printf("Delete dir %s of size %d\n", d.name, s)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func find_smallest_bigger(n *Node, size int) (*Node, int) {
	var cur_node *Node = nil
	cur_size := 0
	for i := range n.children {
		n, s := find_smallest_bigger(n.children[i], size)
		if n != nil && (cur_node == nil || s < cur_size) {
			cur_node = n
			cur_size = s
		}
	}

	if cur_node != nil {
		return cur_node, cur_size
	}

	if sr := n.SizeRecursive(); sr > size {
		return n, sr
	}

	return nil, 0
}

func print_smaller_dirs(n *Node, size int) int {
	if n.size == 0 {
		sizeRec := n.SizeRecursive()
		sum := 0
		if sizeRec < size {
			sum = sizeRec
			fmt.Printf("Node %s is %d\n", n.name, sizeRec)
		}

		for i := range n.children {
			sum += print_smaller_dirs(n.children[i], size)
		}
		return sum
	} else {
		return 0
	}
}
