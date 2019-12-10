import unittest
import orbits

def count_orbits(map):
    rels = build_rels(map)
    count = 0
    for key in rels.keys():
        val = rels[key]
        while val != None:
            count = count + 1
            val = rels[val]

    return count

def build_rels(lines):
    rels = { "COM": None }
    for relationship in lines:
        items = relationship.split(")")
        rels[items[1]] = items[0]
    return rels

def count_transfers(lines):
    rels = build_rels(lines)
    path1 = build_path(rels["YOU"], rels)
    path2 = build_path(rels["SAN"], rels)

    ancestor = None
    count = 0
    for cur in path1:
        #print(f"Path1, transferring to {cur}")
        count = count + 1
        if cur in path2:
            ancestor = cur
            #print(f"Stopping at {ancestor}")
            break

    for cur in path2:
        #print(f"Path2, transferring to {cur}")
        count = count + 1
        if cur == ancestor:
            #print(f"Stopping at {ancestor}")
            break;

    return count

def build_path(start, rels):
    path = []
    while rels[start] != None:
        step = rels[start]
        path.append(step)
        start = step
    return path

class orbits_test(unittest.TestCase):
    def test_count_orbits(self):
        orbit_map = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"""
        count = count_orbits(orbit_map.splitlines())
        self.assertEqual(42, count)

    def test_count_transfers(self):
        map = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"""
        count = count_transfers(map.splitlines())
        self.assertEqual(4, count)

def main():
    #unittest.main()
    solve()

def solve():
    with open("input.txt") as f:
        content = f.readlines()
    content = [x.strip() for x in content] 
    #count = count_orbits(content)
    count = count_transfers(content)
    print(count)

if __name__ == "__main__":
    main()