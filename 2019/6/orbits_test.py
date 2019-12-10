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

def main():
    with open("input.txt") as f:
        content = f.readlines()
    content = [x.strip() for x in content] 
    count = count_orbits(content)
    print(count)

if __name__ == "__main__":
    main()